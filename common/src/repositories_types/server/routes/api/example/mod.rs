#[derive(Debug, postgresql_crud::GeneratePostgresqlCrud)]//
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

    // pub column_0: postgresql_crud::postgresql_type::StdPrimitiveI16AsNotNullInt2,
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

    pub column_154: crate::repositories_types::server::routes::api::example::AnimalAsNotNullJsonbObject,//bug
    // pub column_155: crate::repositories_types::server::routes::api::example::VecOfAnimalWithIdAsNotNullArrayOfNotNullJsonbObjectWithId,
    // pub column_156: crate::repositories_types::server::routes::api::example::OptionAnimalAsNullableJsonbObject,
    // pub column_157: crate::repositories_types::server::routes::api::example::OptionVecOfAnimalWithIdAsNullableArrayOfNotNullJsonbObjectWithId,
}

#[derive(
    Debug
    // , postgresql_crud::GeneratePostgresqlJsonObjectType
)]//
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
//             // }
//             // ,
//             // {
//             //     "not_null_or_nullable": "Nullable",
//             //     "postgresql_json_object_type_pattern": "Standart",
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
pub struct Animal {
    // pub field_0: postgresql_crud::postgresql_json_type::StdPrimitiveI8AsNotNullJsonbNumber,
    // pub field_1: postgresql_crud::postgresql_json_type::OptionStdPrimitiveI8AsNullableJsonbNumber,
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

    // pub field_111: DoggieAsNotNullJsonbObject,
    // pub field_807: VecOfDoggieWithIdAsNotNullArrayOfNotNullJsonbObjectWithId,
    pub field_333: OptionDoggieAsNullableJsonbObject,
    pub field_444: OptionDoggieAsNullableJsonbObject,
    // pub field_777: OptionVecOfDoggieWithIdAsNullableArrayOfNotNullJsonbObjectWithId,//bug
}

#[derive(Debug
    // , postgresql_crud::GeneratePostgresqlJsonObjectType
)]
// #[postgresql_crud::postgresql_json_object_type_pattern{"All"}]
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
//             //     "not_null_or_nullable": "NotNull",
//             //     "postgresql_json_object_type_pattern": "Array",
//             //     "trait_gen": "PostgresqlTypeAndPostgresqlJsonType"
//             // }
//             // ,
//             {
//                 "not_null_or_nullable": "Nullable",
//                 "postgresql_json_object_type_pattern": "Standart",
//                 "trait_gen": "PostgresqlTypeAndPostgresqlJsonType"
//             }
//             // ,
//             // {
//             //     "not_null_or_nullable": "Nullable",
//             //     "postgresql_json_object_type_pattern": "Array",
//             //     "trait_gen": "PostgresqlTypeAndPostgresqlJsonType"
//             // }
//         // ]
//     }
// }]
pub struct Doggie {
    // pub field_806: CatAsNotNullJsonbObject,
    pub field_807: VecOfCatWithIdAsNotNullArrayOfNotNullJsonbObjectWithId,
    // pub field_808: OptionCatAsNullableJsonbObject,
    // pub field_809: OptionVecOfCatWithIdAsNullableArrayOfNotNullJsonbObjectWithId,//bug
}


#[derive(Debug
    // , postgresql_crud::GeneratePostgresqlJsonObjectType
)]
// #[postgresql_crud::postgresql_json_object_type_pattern{"All"}]
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
//             {
//                 "not_null_or_nullable": "NotNull",
//                 "postgresql_json_object_type_pattern": "Array",
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
//             //     "not_null_or_nullable": "Nullable",
//             //     "postgresql_json_object_type_pattern": "Array",
//             //     "trait_gen": "PostgresqlTypeAndPostgresqlJsonType"
//             // }
//         // ]
//     }
// }]
pub struct Cat {
    pub field_444: postgresql_crud::postgresql_json_type::StdPrimitiveI8AsNotNullJsonbNumber,
}
///////////
#[derive(Debug)]
pub struct AnimalAsNotNullJsonbObject;
#[derive(Debug)]
pub struct AnimalWithIdAsNotNullJsonbObjectWithId;
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct AnimalAsNotNullJsonbObjectTableTypeDeclaration {
    field_333: <OptionDoggieAsNullableJsonbObject as postgresql_crud::PostgresqlJsonType>::TableTypeDeclaration,
    field_444: <OptionDoggieAsNullableJsonbObject as postgresql_crud::PostgresqlJsonType>::TableTypeDeclaration,
}
impl AnimalAsNotNullJsonbObjectTableTypeDeclaration {
    pub fn new(field_333: <OptionDoggieAsNullableJsonbObject as postgresql_crud::PostgresqlJsonType>::TableTypeDeclaration, field_444: <OptionDoggieAsNullableJsonbObject as postgresql_crud::PostgresqlJsonType>::TableTypeDeclaration) -> Self {
        Self { field_333, field_444 }
    }
}
impl postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for AnimalAsNotNullJsonbObjectTableTypeDeclaration {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self {
            field_333: postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            field_444: postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct AnimalWithIdAsNotNullJsonbObjectWithIdTableTypeDeclaration {
    id: <postgresql_crud::postgresql_json_type::UuidUuidAsNotNullJsonbString as postgresql_crud::PostgresqlJsonType>::TableTypeDeclaration,
    field_333: <OptionDoggieAsNullableJsonbObject as postgresql_crud::PostgresqlJsonType>::TableTypeDeclaration,
    field_444: <OptionDoggieAsNullableJsonbObject as postgresql_crud::PostgresqlJsonType>::TableTypeDeclaration,
}
impl AnimalWithIdAsNotNullJsonbObjectWithIdTableTypeDeclaration {
    pub fn new(id: <postgresql_crud::postgresql_json_type::UuidUuidAsNotNullJsonbString as postgresql_crud::PostgresqlJsonType>::TableTypeDeclaration, field_333: <OptionDoggieAsNullableJsonbObject as postgresql_crud::PostgresqlJsonType>::TableTypeDeclaration, field_444: <OptionDoggieAsNullableJsonbObject as postgresql_crud::PostgresqlJsonType>::TableTypeDeclaration) -> Self {
        Self { id, field_333, field_444 }
    }
}
impl postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for AnimalWithIdAsNotNullJsonbObjectWithIdTableTypeDeclaration {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self {
            id: postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            field_333: postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            field_444: postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
        }
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
    field_333: <OptionDoggieAsNullableJsonbObject as postgresql_crud::PostgresqlJsonType>::Create,
    field_444: <OptionDoggieAsNullableJsonbObject as postgresql_crud::PostgresqlJsonType>::Create,
}
impl AnimalAsNotNullJsonbObjectCreate {
    pub fn new(field_333: <OptionDoggieAsNullableJsonbObject as postgresql_crud::PostgresqlJsonType>::Create, field_444: <OptionDoggieAsNullableJsonbObject as postgresql_crud::PostgresqlJsonType>::Create) -> Self {
        Self { field_333, field_444 }
    }
}
impl postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for AnimalAsNotNullJsonbObjectCreate {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self {
            field_333: postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            field_444: postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct AnimalWithIdAsNotNullJsonbObjectWithIdCreate {
    field_333: <OptionDoggieAsNullableJsonbObject as postgresql_crud::PostgresqlJsonType>::Create,
    field_444: <OptionDoggieAsNullableJsonbObject as postgresql_crud::PostgresqlJsonType>::Create,
}
impl AnimalWithIdAsNotNullJsonbObjectWithIdCreate {
    pub fn new(field_333: <OptionDoggieAsNullableJsonbObject as postgresql_crud::PostgresqlJsonType>::Create, field_444: <OptionDoggieAsNullableJsonbObject as postgresql_crud::PostgresqlJsonType>::Create) -> Self {
        Self { field_333, field_444 }
    }
}
impl postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for AnimalWithIdAsNotNullJsonbObjectWithIdCreate {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self {
            field_333: postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            field_444: postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
        }
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
        let mut increments = std::string::String::from("");
        match <OptionDoggieAsNullableJsonbObject as postgresql_crud::PostgresqlJsonType>::create_query_part(&self.field_333, increment) {
            Ok(value) => {
                increments.push_str(&postgresql_crud::wrap_into_jsonb_build_object("field_333", &value));
            }
            Err(error) => {
                return Err(error);
            }
        }
        match <OptionDoggieAsNullableJsonbObject as postgresql_crud::PostgresqlJsonType>::create_query_part(&self.field_444, increment) {
            Ok(value) => {
                increments.push_str(&postgresql_crud::wrap_into_jsonb_build_object("field_444", &value));
            }
            Err(error) => {
                return Err(error);
            }
        }
        let _ = increments.pop();
        let _ = increments.pop();
        Ok(format!("{increments}"))
    }
    fn create_query_bind(self, mut query: sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments> {
        query = <OptionDoggieAsNullableJsonbObject as postgresql_crud::PostgresqlJsonType>::create_query_bind(self.field_333, query);
        query = <OptionDoggieAsNullableJsonbObject as postgresql_crud::PostgresqlJsonType>::create_query_bind(self.field_444, query);
        query
    }
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
        match <OptionDoggieAsNullableJsonbObject as postgresql_crud::PostgresqlJsonType>::create_query_part(&self.field_333, increment) {
            Ok(value) => {
                increments.push_str(&postgresql_crud::wrap_into_jsonb_build_object("field_333", &value));
            }
            Err(error) => {
                return Err(error);
            }
        }
        match <OptionDoggieAsNullableJsonbObject as postgresql_crud::PostgresqlJsonType>::create_query_part(&self.field_444, increment) {
            Ok(value) => {
                increments.push_str(&postgresql_crud::wrap_into_jsonb_build_object("field_444", &value));
            }
            Err(error) => {
                return Err(error);
            }
        }
        let _ = increments.pop();
        let _ = increments.pop();
        Ok(format!("jsonb_build_object('id', to_jsonb(gen_random_uuid()))||{increments}"))
    }
    fn create_query_bind(self, mut query: sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments> {
        query = <OptionDoggieAsNullableJsonbObject as postgresql_crud::PostgresqlJsonType>::create_query_bind(self.field_333, query);
        query = <OptionDoggieAsNullableJsonbObject as postgresql_crud::PostgresqlJsonType>::create_query_bind(self.field_444, query);
        query
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
                    AnimalAsNotNullJsonbObjectSelectElement::Field333(value) => <OptionDoggieAsNullableJsonbObject as postgresql_crud::PostgresqlJsonType>::select_query_part(&value, "field_333", &column_name_and_maybe_field_getter_field_ident, &column_name_and_maybe_field_getter_for_error_message_field_ident, false,),
                    AnimalAsNotNullJsonbObjectSelectElement::Field444(value) => <OptionDoggieAsNullableJsonbObject as postgresql_crud::PostgresqlJsonType>::select_query_part(&value, "field_444", &column_name_and_maybe_field_getter_field_ident, &column_name_and_maybe_field_getter_for_error_message_field_ident, false,),
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
    #[serde(rename(serialize = "field_333", deserialize = "field_333"))]
    Field333(<OptionDoggieAsNullableJsonbObject as postgresql_crud::PostgresqlJsonType>::Select),
    #[serde(rename(serialize = "field_444", deserialize = "field_444"))]
    Field444(<OptionDoggieAsNullableJsonbObject as postgresql_crud::PostgresqlJsonType>::Select),
}
impl error_occurence_lib::ToStdStringString for AnimalAsNotNullJsonbObjectSelectElement {
    fn to_std_string_string(&self) -> std::string::String {
        format!("{self:?}")
    }
}
impl postgresql_crud::AllEnumVariantsArrayDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for AnimalAsNotNullJsonbObjectSelectElement {
    fn all_enum_variants_array_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> std::vec::Vec<Self> {
        vec![
            AnimalAsNotNullJsonbObjectSelectElement::Field333(postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            AnimalAsNotNullJsonbObjectSelectElement::Field444(postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
        ]
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
    Id(<postgresql_crud::postgresql_json_type::UuidUuidAsNotNullJsonbString as postgresql_crud::PostgresqlJsonType>::Select),
    #[serde(rename(serialize = "field_333", deserialize = "field_333"))]
    Field333(<OptionDoggieAsNullableJsonbObject as postgresql_crud::PostgresqlJsonType>::Select),
    #[serde(rename(serialize = "field_444", deserialize = "field_444"))]
    Field444(<OptionDoggieAsNullableJsonbObject as postgresql_crud::PostgresqlJsonType>::Select),
}
impl error_occurence_lib::ToStdStringString for AnimalWithIdAsNotNullJsonbObjectWithIdSelectElement {
    fn to_std_string_string(&self) -> std::string::String {
        format!("{self:?}")
    }
}
impl postgresql_crud::AllEnumVariantsArrayDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for AnimalWithIdAsNotNullJsonbObjectWithIdSelectElement {
    fn all_enum_variants_array_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> std::vec::Vec<Self> {
        vec![
            AnimalWithIdAsNotNullJsonbObjectWithIdSelectElement::Id(postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            AnimalWithIdAsNotNullJsonbObjectWithIdSelectElement::Field333(postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            AnimalWithIdAsNotNullJsonbObjectWithIdSelectElement::Field444(postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
        ]
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub enum AnimalAsNotNullJsonbObjectWhereElement {
    Field333(postgresql_crud::PostgresqlTypeWhere<<OptionDoggieAsNullableJsonbObject as postgresql_crud::PostgresqlJsonType>::WhereElement>),
    Field444(postgresql_crud::PostgresqlTypeWhere<<OptionDoggieAsNullableJsonbObject as postgresql_crud::PostgresqlJsonType>::WhereElement>),
}
impl<'a> postgresql_crud::PostgresqlTypeWhereFilter<'a> for AnimalAsNotNullJsonbObjectWhereElement {
    fn query_part(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, postgresql_crud::QueryPartErrorNamed> {
        match &self {
            Self::Field333(value) => postgresql_crud::PostgresqlTypeWhereFilter::query_part(value, increment, &format!("{column}->'field_333'"), is_need_to_add_logical_operator),
            Self::Field444(value) => postgresql_crud::PostgresqlTypeWhereFilter::query_part(value, increment, &format!("{column}->'field_444'"), is_need_to_add_logical_operator),
        }
    }
    fn query_bind(self, query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
        match self {
            Self::Field333(value) => postgresql_crud::PostgresqlTypeWhereFilter::query_bind(value, query),
            Self::Field444(value) => postgresql_crud::PostgresqlTypeWhereFilter::query_bind(value, query),
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
        vec![
            Self::Field333(postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            Self::Field444(postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
        ]
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub enum AnimalWithIdAsNotNullJsonbObjectWithIdWhereElement {
    Id(postgresql_crud::PostgresqlTypeWhere<<postgresql_crud::postgresql_json_type::UuidUuidAsNotNullJsonbString as postgresql_crud::PostgresqlJsonType>::WhereElement>),
    Field333(postgresql_crud::PostgresqlTypeWhere<<OptionDoggieAsNullableJsonbObject as postgresql_crud::PostgresqlJsonType>::WhereElement>),
    Field444(postgresql_crud::PostgresqlTypeWhere<<OptionDoggieAsNullableJsonbObject as postgresql_crud::PostgresqlJsonType>::WhereElement>),
}
impl<'a> postgresql_crud::PostgresqlTypeWhereFilter<'a> for AnimalWithIdAsNotNullJsonbObjectWithIdWhereElement {
    fn query_part(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, postgresql_crud::QueryPartErrorNamed> {
        match &self {
            Self::Id(value) => postgresql_crud::PostgresqlTypeWhereFilter::query_part(value, increment, &format!("{column}->'id'"), is_need_to_add_logical_operator),
            Self::Field333(value) => postgresql_crud::PostgresqlTypeWhereFilter::query_part(value, increment, &format!("{column}->'field_333'"), is_need_to_add_logical_operator),
            Self::Field444(value) => postgresql_crud::PostgresqlTypeWhereFilter::query_part(value, increment, &format!("{column}->'field_444'"), is_need_to_add_logical_operator),
        }
    }
    fn query_bind(self, query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
        match self {
            Self::Id(value) => postgresql_crud::PostgresqlTypeWhereFilter::query_bind(value, query),
            Self::Field333(value) => postgresql_crud::PostgresqlTypeWhereFilter::query_bind(value, query),
            Self::Field444(value) => postgresql_crud::PostgresqlTypeWhereFilter::query_bind(value, query),
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
        vec![
            Self::Id(postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            Self::Field333(postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            Self::Field444(postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
        ]
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct AnimalAsNotNullJsonbObjectRead {
    #[serde(skip_serializing_if = "Option::is_none")]
    field_333: std::option::Option<postgresql_crud::Value<<OptionDoggieAsNullableJsonbObject as postgresql_crud::PostgresqlJsonType>::Read>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    field_444: std::option::Option<postgresql_crud::Value<<OptionDoggieAsNullableJsonbObject as postgresql_crud::PostgresqlJsonType>::Read>>,
}
impl AnimalAsNotNullJsonbObjectRead {
    fn into_inner(self) -> AnimalAsNotNullJsonbObjectReadInner {
        AnimalAsNotNullJsonbObjectReadInner {
            field_333: match self.field_333 {
                Some(value) => Some(postgresql_crud::Value {
                    value: <OptionDoggieAsNullableJsonbObject as postgresql_crud::PostgresqlJsonType>::into_inner(value.value),
                }),
                None => None,
            },
            field_444: match self.field_444 {
                Some(value) => Some(postgresql_crud::Value {
                    value: <OptionDoggieAsNullableJsonbObject as postgresql_crud::PostgresqlJsonType>::into_inner(value.value),
                }),
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
    pub fn try_new(field_333: std::option::Option<postgresql_crud::Value<<OptionDoggieAsNullableJsonbObject as postgresql_crud::PostgresqlJsonType>::Read>>, field_444: std::option::Option<postgresql_crud::Value<<OptionDoggieAsNullableJsonbObject as postgresql_crud::PostgresqlJsonType>::Read>>) -> Result<Self, AnimalAsNotNullJsonbObjectReadTryFromErrorNamed> {
        if let (None, None) = (&field_333, &field_444) {
            return Err(AnimalAsNotNullJsonbObjectReadTryFromErrorNamed::AllFieldsAreNone { code_occurence: error_occurence_lib::code_occurence!() });
        }
        Ok(Self { field_333, field_444 })
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
                        "field_333" => serde::__private::Ok(__Field::__field0),
                        "field_444" => serde::__private::Ok(__Field::__field1),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
                fn visit_bytes<__E>(self, __value: &[u8]) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        b"field_333" => serde::__private::Ok(__Field::__field0),
                        b"field_444" => serde::__private::Ok(__Field::__field1),
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
                    let __field0 = match serde::de::SeqAccess::next_element::<std::option::Option<postgresql_crud::Value<<OptionDoggieAsNullableJsonbObject as postgresql_crud::PostgresqlJsonType>::Read>>>(&mut __seq)? {
                        serde::__private::Some(__value) => __value,
                        serde::__private::None => {
                            return serde::__private::Err(serde::de::Error::invalid_length(0usize, &"struct AnimalAsNotNullJsonbObjectRead with 2 elements"));
                        }
                    };
                    let __field1 = match serde::de::SeqAccess::next_element::<std::option::Option<postgresql_crud::Value<<OptionDoggieAsNullableJsonbObject as postgresql_crud::PostgresqlJsonType>::Read>>>(&mut __seq)? {
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
                    let mut __field0: serde::__private::Option<std::option::Option<postgresql_crud::Value<<OptionDoggieAsNullableJsonbObject as postgresql_crud::PostgresqlJsonType>::Read>>> = serde::__private::None;
                    let mut __field1: serde::__private::Option<std::option::Option<postgresql_crud::Value<<OptionDoggieAsNullableJsonbObject as postgresql_crud::PostgresqlJsonType>::Read>>> = serde::__private::None;
                    while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<__Field>(&mut __map)? {
                        match __key {
                            __Field::__field0 => {
                                if serde::__private::Option::is_some(&__field0) {
                                    return serde::__private::Err(<__A::Error as serde::de::Error>::duplicate_field("field_333"));
                                }
                                __field0 = serde::__private::Some(serde::de::MapAccess::next_value::<std::option::Option<postgresql_crud::Value<<OptionDoggieAsNullableJsonbObject as postgresql_crud::PostgresqlJsonType>::Read>>>(&mut __map)?);
                            }
                            __Field::__field1 => {
                                if serde::__private::Option::is_some(&__field1) {
                                    return serde::__private::Err(<__A::Error as serde::de::Error>::duplicate_field("field_444"));
                                }
                                __field1 = serde::__private::Some(serde::de::MapAccess::next_value::<std::option::Option<postgresql_crud::Value<<OptionDoggieAsNullableJsonbObject as postgresql_crud::PostgresqlJsonType>::Read>>>(&mut __map)?);
                            }
                            _ => {
                                let _ = _serde::de::MapAccess::next_value::<_serde::de::IgnoredAny>(&mut __map)?;
                            }
                        }
                    }
                    let __field0 = match __field0 {
                        serde::__private::Some(__field0) => __field0,
                        serde::__private::None => serde::__private::de::missing_field("field_333")?,
                    };
                    let __field1 = match __field1 {
                        serde::__private::Some(__field1) => __field1,
                        serde::__private::None => serde::__private::de::missing_field("field_444")?,
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
            const FIELDS: &'static [&'static str] = &["field_333", "field_444"];
            _serde::Deserializer::deserialize_struct(
                __deserializer,
                "AnimalAsNotNullJsonbObjectRead",
                FIELDS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<AnimalAsNotNullJsonbObjectRead>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
impl postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for AnimalAsNotNullJsonbObjectRead {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self {
            field_333: Some(postgresql_crud::Value {
                value: postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            }),
            field_444: Some(postgresql_crud::Value {
                value: postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            }),
        }
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
    id: std::option::Option<postgresql_crud::Value<<postgresql_crud::postgresql_json_type::UuidUuidAsNotNullJsonbString as postgresql_crud::PostgresqlJsonType>::Read>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    field_333: std::option::Option<postgresql_crud::Value<<OptionDoggieAsNullableJsonbObject as postgresql_crud::PostgresqlJsonType>::Read>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    field_444: std::option::Option<postgresql_crud::Value<<OptionDoggieAsNullableJsonbObject as postgresql_crud::PostgresqlJsonType>::Read>>,
}
impl AnimalWithIdAsNotNullJsonbObjectWithIdRead {
    fn into_inner(self) -> AnimalWithIdAsNotNullJsonbObjectWithIdReadInner {
        AnimalWithIdAsNotNullJsonbObjectWithIdReadInner {
            id: match self.id {
                Some(value) => Some(postgresql_crud::Value {
                    value: <postgresql_crud::postgresql_json_type::UuidUuidAsNotNullJsonbString as postgresql_crud::PostgresqlJsonType>::into_inner(value.value),
                }),
                None => None,
            },
            field_333: match self.field_333 {
                Some(value) => Some(postgresql_crud::Value {
                    value: <OptionDoggieAsNullableJsonbObject as postgresql_crud::PostgresqlJsonType>::into_inner(value.value),
                }),
                None => None,
            },
            field_444: match self.field_444 {
                Some(value) => Some(postgresql_crud::Value {
                    value: <OptionDoggieAsNullableJsonbObject as postgresql_crud::PostgresqlJsonType>::into_inner(value.value),
                }),
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
    pub fn try_new(
        id: std::option::Option<postgresql_crud::Value<<postgresql_crud::postgresql_json_type::UuidUuidAsNotNullJsonbString as postgresql_crud::PostgresqlJsonType>::Read>>,
        field_333: std::option::Option<postgresql_crud::Value<<OptionDoggieAsNullableJsonbObject as postgresql_crud::PostgresqlJsonType>::Read>>,
        field_444: std::option::Option<postgresql_crud::Value<<OptionDoggieAsNullableJsonbObject as postgresql_crud::PostgresqlJsonType>::Read>>,
    ) -> Result<Self, AnimalWithIdAsNotNullJsonbObjectWithIdReadTryFromErrorNamed> {
        if let (None, None, None) = (&id, &field_333, &field_444) {
            return Err(AnimalWithIdAsNotNullJsonbObjectWithIdReadTryFromErrorNamed::AllFieldsAreNone { code_occurence: error_occurence_lib::code_occurence!() });
        }
        Ok(Self { id, field_333, field_444 })
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
                        "field_333" => serde::__private::Ok(__Field::__field1),
                        "field_444" => serde::__private::Ok(__Field::__field2),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
                fn visit_bytes<__E>(self, __value: &[u8]) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        b"id" => serde::__private::Ok(__Field::__field0),
                        b"field_333" => serde::__private::Ok(__Field::__field1),
                        b"field_444" => serde::__private::Ok(__Field::__field2),
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
                    let __field0 = match serde::de::SeqAccess::next_element::<std::option::Option<postgresql_crud::Value<<postgresql_crud::postgresql_json_type::UuidUuidAsNotNullJsonbString as postgresql_crud::PostgresqlJsonType>::Read>>>(&mut __seq)? {
                        serde::__private::Some(__value) => __value,
                        serde::__private::None => {
                            return serde::__private::Err(serde::de::Error::invalid_length(0usize, &"struct AnimalWithIdAsNotNullJsonbObjectWithIdRead with 3 elements"));
                        }
                    };
                    let __field1 = match serde::de::SeqAccess::next_element::<std::option::Option<postgresql_crud::Value<<OptionDoggieAsNullableJsonbObject as postgresql_crud::PostgresqlJsonType>::Read>>>(&mut __seq)? {
                        serde::__private::Some(__value) => __value,
                        serde::__private::None => {
                            return serde::__private::Err(serde::de::Error::invalid_length(0usize, &"struct AnimalWithIdAsNotNullJsonbObjectWithIdRead with 3 elements"));
                        }
                    };
                    let __field2 = match serde::de::SeqAccess::next_element::<std::option::Option<postgresql_crud::Value<<OptionDoggieAsNullableJsonbObject as postgresql_crud::PostgresqlJsonType>::Read>>>(&mut __seq)? {
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
                    let mut __field0: serde::__private::Option<std::option::Option<postgresql_crud::Value<<postgresql_crud::postgresql_json_type::UuidUuidAsNotNullJsonbString as postgresql_crud::PostgresqlJsonType>::Read>>> = serde::__private::None;
                    let mut __field1: serde::__private::Option<std::option::Option<postgresql_crud::Value<<OptionDoggieAsNullableJsonbObject as postgresql_crud::PostgresqlJsonType>::Read>>> = serde::__private::None;
                    let mut __field2: serde::__private::Option<std::option::Option<postgresql_crud::Value<<OptionDoggieAsNullableJsonbObject as postgresql_crud::PostgresqlJsonType>::Read>>> = serde::__private::None;
                    while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<__Field>(&mut __map)? {
                        match __key {
                            __Field::__field0 => {
                                if serde::__private::Option::is_some(&__field0) {
                                    return serde::__private::Err(<__A::Error as serde::de::Error>::duplicate_field("id"));
                                }
                                __field0 = serde::__private::Some(serde::de::MapAccess::next_value::<std::option::Option<postgresql_crud::Value<<postgresql_crud::postgresql_json_type::UuidUuidAsNotNullJsonbString as postgresql_crud::PostgresqlJsonType>::Read>>>(&mut __map)?);
                            }
                            __Field::__field1 => {
                                if serde::__private::Option::is_some(&__field1) {
                                    return serde::__private::Err(<__A::Error as serde::de::Error>::duplicate_field("field_333"));
                                }
                                __field1 = serde::__private::Some(serde::de::MapAccess::next_value::<std::option::Option<postgresql_crud::Value<<OptionDoggieAsNullableJsonbObject as postgresql_crud::PostgresqlJsonType>::Read>>>(&mut __map)?);
                            }
                            __Field::__field2 => {
                                if serde::__private::Option::is_some(&__field2) {
                                    return serde::__private::Err(<__A::Error as serde::de::Error>::duplicate_field("field_444"));
                                }
                                __field2 = serde::__private::Some(serde::de::MapAccess::next_value::<std::option::Option<postgresql_crud::Value<<OptionDoggieAsNullableJsonbObject as postgresql_crud::PostgresqlJsonType>::Read>>>(&mut __map)?);
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
                        serde::__private::None => serde::__private::de::missing_field("field_333")?,
                    };
                    let __field2 = match __field2 {
                        serde::__private::Some(__field2) => __field2,
                        serde::__private::None => serde::__private::de::missing_field("field_444")?,
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
            const FIELDS: &'static [&'static str] = &["id", "field_333", "field_444"];
            _serde::Deserializer::deserialize_struct(
                __deserializer,
                "AnimalWithIdAsNotNullJsonbObjectWithIdRead",
                FIELDS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<AnimalWithIdAsNotNullJsonbObjectWithIdRead>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
impl postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for AnimalWithIdAsNotNullJsonbObjectWithIdRead {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self {
            id: Some(postgresql_crud::Value {
                value: postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            }),
            field_333: Some(postgresql_crud::Value {
                value: postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            }),
            field_444: Some(postgresql_crud::Value {
                value: postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            }),
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
    field_333: <OptionDoggieAsNullableJsonbObject as postgresql_crud::PostgresqlJsonType>::ReadOnlyIds,
    field_444: <OptionDoggieAsNullableJsonbObject as postgresql_crud::PostgresqlJsonType>::ReadOnlyIds,
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
    id: <postgresql_crud::postgresql_json_type::UuidUuidAsNotNullJsonbString as postgresql_crud::PostgresqlJsonType>::ReadOnlyIds,
    field_333: <OptionDoggieAsNullableJsonbObject as postgresql_crud::PostgresqlJsonType>::ReadOnlyIds,
    field_444: <OptionDoggieAsNullableJsonbObject as postgresql_crud::PostgresqlJsonType>::ReadOnlyIds,
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
    field_333: std::option::Option<postgresql_crud::Value<<OptionDoggieAsNullableJsonbObject as postgresql_crud::PostgresqlJsonType>::ReadInner>>,
    field_444: std::option::Option<postgresql_crud::Value<<OptionDoggieAsNullableJsonbObject as postgresql_crud::PostgresqlJsonType>::ReadInner>>,
}
#[derive(Debug, Clone, PartialEq)]
pub struct AnimalWithIdAsNotNullJsonbObjectWithIdReadInner {
    id: std::option::Option<postgresql_crud::Value<<postgresql_crud::postgresql_json_type::UuidUuidAsNotNullJsonbString as postgresql_crud::PostgresqlJsonType>::ReadInner>>,
    field_333: std::option::Option<postgresql_crud::Value<<OptionDoggieAsNullableJsonbObject as postgresql_crud::PostgresqlJsonType>::ReadInner>>,
    field_444: std::option::Option<postgresql_crud::Value<<OptionDoggieAsNullableJsonbObject as postgresql_crud::PostgresqlJsonType>::ReadInner>>,
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
    #[serde(rename(serialize = "field_333", deserialize = "field_333"))]
    Field333(postgresql_crud::Value<<OptionDoggieAsNullableJsonbObject as postgresql_crud::PostgresqlJsonType>::Update>),
    #[serde(rename(serialize = "field_444", deserialize = "field_444"))]
    Field444(postgresql_crud::Value<<OptionDoggieAsNullableJsonbObject as postgresql_crud::PostgresqlJsonType>::Update>),
}
impl postgresql_crud::AllEnumVariantsArrayDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for AnimalAsNotNullJsonbObjectUpdateElement {
    fn all_enum_variants_array_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> std::vec::Vec<Self> {
        vec![
            AnimalAsNotNullJsonbObjectUpdateElement::Field333(postgresql_crud::Value {
                value: postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            }),
            AnimalAsNotNullJsonbObjectUpdateElement::Field444(postgresql_crud::Value {
                value: postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            }),
        ]
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct AnimalWithIdAsNotNullJsonbObjectWithIdUpdateElement {
    id: <postgresql_crud::postgresql_json_type::UuidUuidAsNotNullJsonbString as postgresql_crud::PostgresqlJsonType>::Update,
    fields: <AnimalAsNotNullJsonbObject as postgresql_crud::PostgresqlJsonType>::Update,
}
impl AnimalWithIdAsNotNullJsonbObjectWithIdUpdateElement {
    pub fn new(id: <postgresql_crud::postgresql_json_type::UuidUuidAsNotNullJsonbString as postgresql_crud::PostgresqlJsonType>::Update, fields: <AnimalAsNotNullJsonbObject as postgresql_crud::PostgresqlJsonType>::Update) -> Self {
        Self { id, fields }
    }
}
impl postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for AnimalWithIdAsNotNullJsonbObjectWithIdUpdateElement {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self {
            id: postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            fields: postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct AnimalWithIdAsNotNullJsonbObjectWithIdUpdateHandle(postgresql_crud::UniqueVec<AnimalWithIdAsNotNullJsonbObjectWithIdUpdateElement>);
impl postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for AnimalWithIdAsNotNullJsonbObjectWithIdUpdateHandle {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self(postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element())
    }
}
impl AnimalWithIdAsNotNullJsonbObjectWithIdUpdateHandle {
    pub fn new(value: postgresql_crud::UniqueVec<AnimalWithIdAsNotNullJsonbObjectWithIdUpdateElement>) -> Self {
        Self(value)
    }
}
impl std::default::Default for AnimalWithIdAsNotNullJsonbObjectWithIdUpdateHandle {
    fn default() -> Self {
        Self(postgresql_crud::UniqueVec::default())
    }
}
impl postgresql_crud::PostgresqlJsonType for AnimalAsNotNullJsonbObject {
    type TableTypeDeclaration = AnimalAsNotNullJsonbObjectTableTypeDeclaration;
    type Create = AnimalAsNotNullJsonbObjectCreate;
    fn create_query_part(value: &Self::Create, increment: &mut std::primitive::u64) -> Result<std::string::String, postgresql_crud::QueryPartErrorNamed> {
        value.create_query_part(increment)
    }
    fn create_query_bind(value: Self::Create, query: sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments> {
        value.create_query_bind(query)
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
                    AnimalAsNotNullJsonbObjectSelectElement::Field333(value) => <OptionDoggieAsNullableJsonbObject as postgresql_crud::PostgresqlJsonType>::select_query_part(&value, "field_333", &column_name_and_maybe_field_getter_field_ident, &column_name_and_maybe_field_getter_for_error_message_field_ident, false,),
                    AnimalAsNotNullJsonbObjectSelectElement::Field444(value) => <OptionDoggieAsNullableJsonbObject as postgresql_crud::PostgresqlJsonType>::select_query_part(&value, "field_444", &column_name_and_maybe_field_getter_field_ident, &column_name_and_maybe_field_getter_for_error_message_field_ident, false,),
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
        format!(
            "jsonb_build_object('value',jsonb_build_object('field_333',{},'field_444',{}))",
            <OptionDoggieAsNullableJsonbObject as postgresql_crud::PostgresqlJsonType>::select_only_ids_query_part(&format!("{column_name_and_maybe_field_getter}->'field_333'")),
            <OptionDoggieAsNullableJsonbObject as postgresql_crud::PostgresqlJsonType>::select_only_ids_query_part(&format!("{column_name_and_maybe_field_getter}->'field_444'"))
        )
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
                AnimalAsNotNullJsonbObjectUpdateElement::Field333(value) => match <OptionDoggieAsNullableJsonbObject as postgresql_crud::PostgresqlJsonType>::update_query_part(&value.value, &std_option_option_object_acc, &generate_jsonb_set_target("field_333"), "field_333", increment) {
                    Ok(value) => {
                        std_option_option_object_acc = value;
                    }
                    Err(error) => {
                        return Err(error);
                    }
                },
                AnimalAsNotNullJsonbObjectUpdateElement::Field444(value) => match <OptionDoggieAsNullableJsonbObject as postgresql_crud::PostgresqlJsonType>::update_query_part(&value.value, &std_option_option_object_acc, &generate_jsonb_set_target("field_444"), "field_444", increment) {
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
    fn update_query_bind(value: Self::Update, mut query: sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments> {
        for element in value.0.into_vec() {
            match element {
                AnimalAsNotNullJsonbObjectUpdateElement::Field333(value) => {
                    query = <OptionDoggieAsNullableJsonbObject as postgresql_crud::PostgresqlJsonType>::update_query_bind(value.value, query);
                }
                AnimalAsNotNullJsonbObjectUpdateElement::Field444(value) => {
                    query = <OptionDoggieAsNullableJsonbObject as postgresql_crud::PostgresqlJsonType>::update_query_bind(value.value, query);
                }
            }
        }
        query
    }
    fn select_only_updated_ids_query_part(value: &Self::Update, field_ident: &std::primitive::str, column_name_and_maybe_field_getter: &std::primitive::str, increment: &mut std::primitive::u64) -> Result<std::string::String, postgresql_crud::QueryPartErrorNamed> {
        let mut acc = std::string::String::default();
        for element in value.0.to_vec() {
            match &element {
                AnimalAsNotNullJsonbObjectUpdateElement::Field333(value) => match <OptionDoggieAsNullableJsonbObject as postgresql_crud::PostgresqlJsonType>::select_only_updated_ids_query_part(&value.value, &"field_333", &format!("{column_name_and_maybe_field_getter}->'{field_ident}'"), increment) {
                    Ok(value) => {
                        acc.push_str(&value);
                    }
                    Err(error) => {
                        return Err(error);
                    }
                },
                AnimalAsNotNullJsonbObjectUpdateElement::Field444(value) => match <OptionDoggieAsNullableJsonbObject as postgresql_crud::PostgresqlJsonType>::select_only_updated_ids_query_part(&value.value, &"field_444", &format!("{column_name_and_maybe_field_getter}->'{field_ident}'"), increment) {
                    Ok(value) => {
                        acc.push_str(&value);
                    }
                    Err(error) => {
                        return Err(error);
                    }
                },
            }
        }
        let _ = acc.pop();
        Ok(format!("'{field_ident}',jsonb_build_object('value',jsonb_build_object({acc})),"))
    }
}
impl postgresql_crud::PostgresqlType for AnimalAsNotNullJsonbObject {
    type TableTypeDeclaration = AnimalAsNotNullJsonbObjectTableTypeDeclaration;
    type Create = AnimalAsNotNullJsonbObjectCreate;
    fn create_query_part(value: &Self::Create, increment: &mut std::primitive::u64) -> Result<std::string::String, postgresql_crud::QueryPartErrorNamed> {
        value.create_query_part(increment)
    }
    fn create_query_bind(value: Self::Create, query: sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments> {
        value.create_query_bind(query)
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
                AnimalAsNotNullJsonbObjectUpdateElement::Field333(value) => match <OptionDoggieAsNullableJsonbObject as postgresql_crud::PostgresqlJsonType>::update_query_part(&value.value, &std_option_option_object_acc, &generate_jsonb_set_target("field_333"), "field_333", increment) {
                    Ok(value) => {
                        std_option_option_object_acc = value;
                    }
                    Err(error) => {
                        return Err(error);
                    }
                },
                AnimalAsNotNullJsonbObjectUpdateElement::Field444(value) => match <OptionDoggieAsNullableJsonbObject as postgresql_crud::PostgresqlJsonType>::update_query_part(&value.value, &std_option_option_object_acc, &generate_jsonb_set_target("field_444"), "field_444", increment) {
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
    fn update_query_bind<'a>(value: Self::Update, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
        for element in value.0.into_vec() {
            match element {
                AnimalAsNotNullJsonbObjectUpdateElement::Field333(value) => {
                    query = <OptionDoggieAsNullableJsonbObject as postgresql_crud::PostgresqlJsonType>::update_query_bind(value.value, query);
                }
                AnimalAsNotNullJsonbObjectUpdateElement::Field444(value) => {
                    query = <OptionDoggieAsNullableJsonbObject as postgresql_crud::PostgresqlJsonType>::update_query_bind(value.value, query);
                }
            }
        }
        query
    }
    fn select_only_updated_ids_query_part(value: &Self::Update, column: &std::primitive::str, increment: &mut std::primitive::u64, is_primary_key: std::primitive::bool) -> Result<std::string::String, postgresql_crud::QueryPartErrorNamed> {
        println!("@@@2");
        let mut acc = std::string::String::default();
        for element in value.0.to_vec() {
            match &element {
                AnimalAsNotNullJsonbObjectUpdateElement::Field333(value) => match <OptionDoggieAsNullableJsonbObject as postgresql_crud::PostgresqlJsonType>::select_only_updated_ids_query_part(&value.value, &"field_333", &column, increment) {
                    Ok(value) => {
                        acc.push_str(&value);
                    }
                    Err(error) => {
                        return Err(error);
                    }
                },
                AnimalAsNotNullJsonbObjectUpdateElement::Field444(value) => match <OptionDoggieAsNullableJsonbObject as postgresql_crud::PostgresqlJsonType>::select_only_updated_ids_query_part(&value.value, &"field_444", &column, increment) {
                    Ok(value) => {
                        acc.push_str(&value);
                    }
                    Err(error) => {
                        return Err(error);
                    }
                },
            }
        }
        let _ = acc.pop();
        // Ok(format!("jsonb_build_object('value',jsonb_build_object({acc})) as {column},"))
        // Ok(r#"

        // '{
        //   "value": {
        //     "field_333": {
        //       "value": {
        //         "value": {
        //           "field_807": {
        //             "value": [
        //               {
        //                 "value": {
        //                   "id": {
        //                     "value": "51544112-2b5b-42b0-8a0b-2b9857b1270c"
        //                   },
        //                   "field_444": {
        //                     "value": null
        //                   }
        //                 }
        //               }
        //             ]
        //           }
        //         }
        //       }
        //     },
        //     "field_444": {
        //       "value": {
        //         "value": {
        //           "field_807": {
        //             "value": [
        //               {
        //                 "value": {
        //                   "id": {
        //                     "value": "51544112-2b5b-42b0-8a0b-2b9857b1270c"
        //                   },
        //                   "field_444": {
        //                     "value": null
        //                   }
        //                 }
        //               }
        //             ]
        //           }
        //         }
        //       }
        //     }
        //   }
        // }'::jsonb 
        
        // as column_154,"#.to_string())

        Ok(r#"
        
        '{
          "value": {
            "field_333": {
              "value": {
                "value": {
                  "field_807": {
                    "value": null
                  }
                }
              }
            },
            "field_444": {
              "value": {
                "value": {
                  "field_807": {
                    "value": [
                      {
                        "value": {
                          "id": {
                            "value": "51544112-2b5b-42b0-8a0b-2b9857b1270c"
                          },
                          "field_444": {
                            "value": null
                          }
                        }
                      }
                    ]
                  }
                }
              }
            }
          }
        }'::jsonb 
        
        as column_154,"#.to_string())
    }
}
impl postgresql_crud::PostgresqlJsonType for AnimalWithIdAsNotNullJsonbObjectWithId {
    type TableTypeDeclaration = AnimalWithIdAsNotNullJsonbObjectWithIdTableTypeDeclaration;
    type Create = AnimalWithIdAsNotNullJsonbObjectWithIdCreate;
    fn create_query_part(value: &Self::Create, increment: &mut std::primitive::u64) -> Result<std::string::String, postgresql_crud::QueryPartErrorNamed> {
        value.create_query_part(increment)
    }
    fn create_query_bind(value: Self::Create, query: sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments> {
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
                    AnimalWithIdAsNotNullJsonbObjectWithIdSelectElement::Id(value) => <postgresql_crud::postgresql_json_type::UuidUuidAsNotNullJsonbString as postgresql_crud::PostgresqlJsonType>::select_query_part(&value, "id", &column_name_and_maybe_field_getter_field_ident, &column_name_and_maybe_field_getter_for_error_message_field_ident, false,),
                    AnimalWithIdAsNotNullJsonbObjectWithIdSelectElement::Field333(value) => <OptionDoggieAsNullableJsonbObject as postgresql_crud::PostgresqlJsonType>::select_query_part(&value, "field_333", &column_name_and_maybe_field_getter_field_ident, &column_name_and_maybe_field_getter_for_error_message_field_ident, false,),
                    AnimalWithIdAsNotNullJsonbObjectWithIdSelectElement::Field444(value) => <OptionDoggieAsNullableJsonbObject as postgresql_crud::PostgresqlJsonType>::select_query_part(&value, "field_444", &column_name_and_maybe_field_getter_field_ident, &column_name_and_maybe_field_getter_for_error_message_field_ident, false,),
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
        format!(
            "jsonb_build_object('value',jsonb_build_object('id',{},'field_333',{},'field_444',{}))",
            <postgresql_crud::postgresql_json_type::UuidUuidAsNotNullJsonbString as postgresql_crud::PostgresqlJsonType>::select_only_ids_query_part(&format!("{column_name_and_maybe_field_getter}->'id'")),
            <OptionDoggieAsNullableJsonbObject as postgresql_crud::PostgresqlJsonType>::select_only_ids_query_part(&format!("{column_name_and_maybe_field_getter}->'field_333'")),
            <OptionDoggieAsNullableJsonbObject as postgresql_crud::PostgresqlJsonType>::select_only_ids_query_part(&format!("{column_name_and_maybe_field_getter}->'field_444'"))
        )
    }
    type ReadInner = AnimalWithIdAsNotNullJsonbObjectWithIdReadInner;
    fn into_inner(value: Self::Read) -> Self::ReadInner {
        value.into_inner()
    }
    type Update = AnimalWithIdAsNotNullJsonbObjectWithIdUpdateHandle;
    fn update_query_part(value: &Self::Update, jsonb_set_accumulator: &std::primitive::str, jsonb_set_target: &std::primitive::str, jsonb_set_path: &std::primitive::str, increment: &mut std::primitive::u64) -> Result<std::string::String, postgresql_crud::QueryPartErrorNamed> {
        todo!()
    }
    fn update_query_bind(value: Self::Update, query: sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments> {
        todo!()
    }
    fn select_only_updated_ids_query_part(value: &Self::Update, field_ident: &std::primitive::str, column_name_and_maybe_field_getter: &std::primitive::str, increment: &mut std::primitive::u64) -> Result<std::string::String, postgresql_crud::QueryPartErrorNamed> {
        let mut acc = std::string::String::new();
        for element in value.0.to_vec() {
            acc.push_str(&format!("jsonb_build_object('id',jsonb_build_object('value','{}'),{})||", element.id.get_inner(), {
                let mut acc = std::string::String::new();
                for element in element.fields.0.to_vec() {
                    match &element {
                        AnimalAsNotNullJsonbObjectUpdateElement::Field333(value) => match <OptionDoggieAsNullableJsonbObject as postgresql_crud::PostgresqlJsonType>::select_only_updated_ids_query_part(&value.value, &"field_333", &column_name_and_maybe_field_getter, increment) {
                            Ok(value) => {
                                acc.push_str(&value);
                            }
                            Err(error) => {
                                return Err(error);
                            }
                        },
                        AnimalAsNotNullJsonbObjectUpdateElement::Field444(value) => match <OptionDoggieAsNullableJsonbObject as postgresql_crud::PostgresqlJsonType>::select_only_updated_ids_query_part(&value.value, &"field_444", &column_name_and_maybe_field_getter, increment) {
                            Ok(value) => {
                                acc.push_str(&value);
                            }
                            Err(error) => {
                                return Err(error);
                            }
                        },
                    }
                }
                let _ = acc.pop();
                acc
            }));
        }
        let _ = acc.pop();
        let _ = acc.pop();
        Ok(format!("jsonb_build_object('value',{acc})"))
    }
}
#[cfg(test)]
impl postgresql_crud::tests::PostgresqlTypeTestCases for AnimalAsNotNullJsonbObject {
    type Element = Self;
    fn test_cases(read_only_ids: &<Self::Element as postgresql_crud::PostgresqlType>::ReadOnlyIds) -> std::vec::Vec<std::vec::Vec<<Self::Element as postgresql_crud::PostgresqlType>::ReadInner>> {
        let mut acc = vec![];
        for element in <OptionDoggieAsNullableJsonbObject as postgresql_crud::tests::PostgresqlJsonTypeTestCases>::test_cases(&read_only_ids.0.value.field_333) {
            for current_element in element {
                acc.push(AnimalAsNotNullJsonbObjectReadInner {
                    field_333: Some(postgresql_crud::Value { value: current_element }),
                    field_444: <OptionDoggieAsNullableJsonbObject as postgresql_crud::tests::PostgresqlJsonTypeTestCases>::read_only_ids_to_option_value_read_inner(read_only_ids.0.value.field_444.clone()),
                });
            }
        }
        for element in <OptionDoggieAsNullableJsonbObject as postgresql_crud::tests::PostgresqlJsonTypeTestCases>::test_cases(&read_only_ids.0.value.field_444) {
            for current_element in element {
                acc.push(AnimalAsNotNullJsonbObjectReadInner {
                    field_333: <OptionDoggieAsNullableJsonbObject as postgresql_crud::tests::PostgresqlJsonTypeTestCases>::read_only_ids_to_option_value_read_inner(read_only_ids.0.value.field_333.clone()),
                    field_444: Some(postgresql_crud::Value { value: current_element }),
                });
            }
        }
        vec![acc]
    }
    fn read_new_or_try_new_unwraped_for_test(value: AnimalAsNotNullJsonbObjectReadInner) -> <Self::Element as postgresql_crud::PostgresqlType>::Read {
        <Self::Element as postgresql_crud::PostgresqlType>::Read::try_new(
            match value.field_333 {
                Some(value) => Some(postgresql_crud::Value {
                    value: <OptionDoggieAsNullableJsonbObject as postgresql_crud::tests::PostgresqlJsonTypeTestCases>::read_new_or_try_new_unwraped_for_test(value.value),
                }),
                None => None,
            },
            match value.field_444 {
                Some(value) => Some(postgresql_crud::Value {
                    value: <OptionDoggieAsNullableJsonbObject as postgresql_crud::tests::PostgresqlJsonTypeTestCases>::read_new_or_try_new_unwraped_for_test(value.value),
                }),
                None => None,
            },
        )
        .unwrap()
    }
    fn update_new_or_try_new_unwraped_for_test(value: AnimalAsNotNullJsonbObjectReadInner) -> <Self::Element as postgresql_crud::PostgresqlType>::Update {
        <Self::Element as postgresql_crud::PostgresqlType>::Update::new(
            postgresql_crud::NotEmptyUniqueEnumVec::try_new({
                let mut acc = vec![];
                if let Some(value) = value.field_333 {
                    acc.push(AnimalAsNotNullJsonbObjectUpdateElement::Field333(postgresql_crud::Value {
                        value: <OptionDoggieAsNullableJsonbObject as postgresql_crud::tests::PostgresqlJsonTypeTestCases>::update_new_or_try_new_unwraped_for_test(value.value),
                    }));
                }
                if let Some(value) = value.field_444 {
                    acc.push(AnimalAsNotNullJsonbObjectUpdateElement::Field444(postgresql_crud::Value {
                        value: <OptionDoggieAsNullableJsonbObject as postgresql_crud::tests::PostgresqlJsonTypeTestCases>::update_new_or_try_new_unwraped_for_test(value.value),
                    }));
                }
                acc
            })
            .unwrap(),
        )
    }
    fn read_only_ids_to_option_value_read_inner(value: <Self::Element as postgresql_crud::PostgresqlType>::ReadOnlyIds) -> std::option::Option<postgresql_crud::Value<<Self::Element as postgresql_crud::PostgresqlType>::ReadInner>> {
        todo!()
    }
    fn update_to_read_only_ids(value: &<Self::Element as postgresql_crud::PostgresqlType>::Update) -> <Self::Element as postgresql_crud::PostgresqlType>::ReadOnlyIds {
        let mut field_333 = None;
        let mut field_444 = None;
        for element in value.0.to_vec() {
            match element {
                AnimalAsNotNullJsonbObjectUpdateElement::Field333(value) => {
                    field_333 = Some(<OptionDoggieAsNullableJsonbObject as postgresql_crud::tests::PostgresqlJsonTypeTestCases>::update_to_read_only_ids(&value.value));
                }
                AnimalAsNotNullJsonbObjectUpdateElement::Field444(value) => {
                    field_444 = Some(<OptionDoggieAsNullableJsonbObject as postgresql_crud::tests::PostgresqlJsonTypeTestCases>::update_to_read_only_ids(&value.value));
                }
            }
        }
        AnimalAsNotNullJsonbObjectReadOnlyIds(postgresql_crud::Value {
            value: AnimalAsNotNullJsonbObjectReadOnlyIdsHandle {
                field_333: field_333.expect("expect 106f16f2-ae03-48b3-9239-f4b1b60746d5"),
                field_444: field_444.expect("expect 106f16f2-ae03-48b3-9239-f4b1b60746d5"),
            },
        })
    }
}
#[cfg(test)]
impl postgresql_crud::tests::PostgresqlJsonTypeTestCases for AnimalAsNotNullJsonbObject {
    type Element = Self;
    fn test_cases(read_only_ids: &<Self::Element as postgresql_crud::PostgresqlJsonType>::ReadOnlyIds) -> std::vec::Vec<std::vec::Vec<<Self::Element as postgresql_crud::PostgresqlJsonType>::ReadInner>> {
        let mut acc = vec![];
        for element in <OptionDoggieAsNullableJsonbObject as postgresql_crud::tests::PostgresqlJsonTypeTestCases>::test_cases(&read_only_ids.0.value.field_333) {
            for current_element in element {
                acc.push(AnimalAsNotNullJsonbObjectReadInner {
                    field_333: Some(postgresql_crud::Value { value: current_element }),
                    field_444: <OptionDoggieAsNullableJsonbObject as postgresql_crud::tests::PostgresqlJsonTypeTestCases>::read_only_ids_to_option_value_read_inner(read_only_ids.0.value.field_444.clone()),
                });
            }
        }
        for element in <OptionDoggieAsNullableJsonbObject as postgresql_crud::tests::PostgresqlJsonTypeTestCases>::test_cases(&read_only_ids.0.value.field_444) {
            for current_element in element {
                acc.push(AnimalAsNotNullJsonbObjectReadInner {
                    field_333: <OptionDoggieAsNullableJsonbObject as postgresql_crud::tests::PostgresqlJsonTypeTestCases>::read_only_ids_to_option_value_read_inner(read_only_ids.0.value.field_333.clone()),
                    field_444: Some(postgresql_crud::Value { value: current_element }),
                });
            }
        }
        vec![acc]
    }
    fn read_new_or_try_new_unwraped_for_test(value: AnimalAsNotNullJsonbObjectReadInner) -> <Self::Element as postgresql_crud::PostgresqlJsonType>::Read {
        <Self::Element as postgresql_crud::PostgresqlType>::Read::try_new(
            match value.field_333 {
                Some(value) => Some(postgresql_crud::Value {
                    value: <OptionDoggieAsNullableJsonbObject as postgresql_crud::tests::PostgresqlJsonTypeTestCases>::read_new_or_try_new_unwraped_for_test(value.value),
                }),
                None => None,
            },
            match value.field_444 {
                Some(value) => Some(postgresql_crud::Value {
                    value: <OptionDoggieAsNullableJsonbObject as postgresql_crud::tests::PostgresqlJsonTypeTestCases>::read_new_or_try_new_unwraped_for_test(value.value),
                }),
                None => None,
            },
        )
        .unwrap()
    }
    fn update_new_or_try_new_unwraped_for_test(value: AnimalAsNotNullJsonbObjectReadInner) -> <Self::Element as postgresql_crud::PostgresqlJsonType>::Update {
        <Self::Element as postgresql_crud::PostgresqlType>::Update::new(
            postgresql_crud::NotEmptyUniqueEnumVec::try_new({
                let mut acc = vec![];
                if let Some(value) = value.field_333 {
                    acc.push(AnimalAsNotNullJsonbObjectUpdateElement::Field333(postgresql_crud::Value {
                        value: <OptionDoggieAsNullableJsonbObject as postgresql_crud::tests::PostgresqlJsonTypeTestCases>::update_new_or_try_new_unwraped_for_test(value.value),
                    }));
                }
                if let Some(value) = value.field_444 {
                    acc.push(AnimalAsNotNullJsonbObjectUpdateElement::Field444(postgresql_crud::Value {
                        value: <OptionDoggieAsNullableJsonbObject as postgresql_crud::tests::PostgresqlJsonTypeTestCases>::update_new_or_try_new_unwraped_for_test(value.value),
                    }));
                }
                acc
            })
            .unwrap(),
        )
    }
    fn read_only_ids_to_option_value_read_inner(value: <Self::Element as postgresql_crud::PostgresqlJsonType>::ReadOnlyIds) -> std::option::Option<postgresql_crud::Value<<Self::Element as postgresql_crud::PostgresqlJsonType>::ReadInner>> {
        Some(postgresql_crud::Value {
            value: AnimalAsNotNullJsonbObjectReadInner {
                field_333: match <OptionDoggieAsNullableJsonbObject as postgresql_crud::tests::PostgresqlJsonTypeTestCases>::read_only_ids_to_option_value_read_inner(value.0.value.field_333) {
                    Some(value) => Some(value),
                    None => Some(postgresql_crud::Value {
                        value: <OptionDoggieAsNullableJsonbObject as postgresql_crud::PostgresqlJsonType>::into_inner(<<OptionDoggieAsNullableJsonbObject as postgresql_crud::PostgresqlJsonType>::Read as postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement>::default_but_option_is_always_some_and_vec_always_contains_one_element()),
                    }),
                },
                field_444: match <OptionDoggieAsNullableJsonbObject as postgresql_crud::tests::PostgresqlJsonTypeTestCases>::read_only_ids_to_option_value_read_inner(value.0.value.field_444) {
                    Some(value) => Some(value),
                    None => Some(postgresql_crud::Value {
                        value: <OptionDoggieAsNullableJsonbObject as postgresql_crud::PostgresqlJsonType>::into_inner(<<OptionDoggieAsNullableJsonbObject as postgresql_crud::PostgresqlJsonType>::Read as postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement>::default_but_option_is_always_some_and_vec_always_contains_one_element()),
                    }),
                },
            },
        })
    }
    fn update_to_read_only_ids(value: &<Self::Element as postgresql_crud::PostgresqlJsonType>::Update) -> <Self::Element as postgresql_crud::PostgresqlJsonType>::ReadOnlyIds {
        let mut field_333 = None;
        let mut field_444 = None;
        for element in value.0.to_vec() {
            match element {
                AnimalAsNotNullJsonbObjectUpdateElement::Field333(value) => {
                    field_333 = Some(<OptionDoggieAsNullableJsonbObject as postgresql_crud::tests::PostgresqlJsonTypeTestCases>::update_to_read_only_ids(&value.value));
                }
                AnimalAsNotNullJsonbObjectUpdateElement::Field444(value) => {
                    field_444 = Some(<OptionDoggieAsNullableJsonbObject as postgresql_crud::tests::PostgresqlJsonTypeTestCases>::update_to_read_only_ids(&value.value));
                }
            }
        }
        AnimalAsNotNullJsonbObjectReadOnlyIds(postgresql_crud::Value {
            value: AnimalAsNotNullJsonbObjectReadOnlyIdsHandle {
                field_333: field_333.expect("expect 106f16f2-ae03-48b3-9239-f4b1b60746d5"),
                field_444: field_444.expect("expect 106f16f2-ae03-48b3-9239-f4b1b60746d5"),
            },
        })
    }
}
#[cfg(test)]
impl postgresql_crud::tests::PostgresqlJsonTypeTestCases for AnimalWithIdAsNotNullJsonbObjectWithId {
    type Element = Self;
    fn test_cases(read_only_ids: &<Self::Element as postgresql_crud::PostgresqlJsonType>::ReadOnlyIds) -> std::vec::Vec<std::vec::Vec<<Self::Element as postgresql_crud::PostgresqlJsonType>::ReadInner>> {
        vec![vec![AnimalWithIdAsNotNullJsonbObjectWithIdReadInner {
            id: Some(postgresql_crud::Value { value: read_only_ids.0.value.id.0.value.clone() }),
            field_333: <OptionDoggieAsNullableJsonbObject as postgresql_crud::tests::PostgresqlJsonTypeTestCases>::read_only_ids_to_option_value_read_inner(read_only_ids.0.value.field_333.clone()),
            field_444: <OptionDoggieAsNullableJsonbObject as postgresql_crud::tests::PostgresqlJsonTypeTestCases>::read_only_ids_to_option_value_read_inner(read_only_ids.0.value.field_444.clone()),
        }]]
    }
    fn read_new_or_try_new_unwraped_for_test(value: AnimalWithIdAsNotNullJsonbObjectWithIdReadInner) -> <Self::Element as postgresql_crud::PostgresqlJsonType>::Read {
        AnimalWithIdAsNotNullJsonbObjectWithIdRead {
            id: match value.id {
                Some(value) => Some(postgresql_crud::Value {
                    value: <postgresql_crud::postgresql_json_type::UuidUuidAsNotNullJsonbString as postgresql_crud::tests::PostgresqlJsonTypeTestCases>::read_new_or_try_new_unwraped_for_test(value.value),
                }),
                None => None,
            },
            field_333: match value.field_333 {
                Some(value) => Some(postgresql_crud::Value {
                    value: <OptionDoggieAsNullableJsonbObject as postgresql_crud::tests::PostgresqlJsonTypeTestCases>::read_new_or_try_new_unwraped_for_test(value.value),
                }),
                None => None,
            },
            field_444: match value.field_444 {
                Some(value) => Some(postgresql_crud::Value {
                    value: <OptionDoggieAsNullableJsonbObject as postgresql_crud::tests::PostgresqlJsonTypeTestCases>::read_new_or_try_new_unwraped_for_test(value.value),
                }),
                None => None,
            },
        }
    }
    fn update_new_or_try_new_unwraped_for_test(value: AnimalWithIdAsNotNullJsonbObjectWithIdReadInner) -> <Self::Element as postgresql_crud::PostgresqlJsonType>::Update {
        todo!()
    }
    fn read_only_ids_to_option_value_read_inner(value: <Self::Element as postgresql_crud::PostgresqlJsonType>::ReadOnlyIds) -> std::option::Option<postgresql_crud::Value<<Self::Element as postgresql_crud::PostgresqlJsonType>::ReadInner>> {
        Some(postgresql_crud::Value {
            value: AnimalWithIdAsNotNullJsonbObjectWithIdReadInner {
                id: match <postgresql_crud::postgresql_json_type::UuidUuidAsNotNullJsonbString as postgresql_crud::tests::PostgresqlJsonTypeTestCases>::read_only_ids_to_option_value_read_inner(value.0.value.id) {
                    Some(value) => Some(value),
                    None => Some(postgresql_crud::Value {
                        value: <postgresql_crud::postgresql_json_type::UuidUuidAsNotNullJsonbString as postgresql_crud::PostgresqlJsonType>::into_inner(<<postgresql_crud::postgresql_json_type::UuidUuidAsNotNullJsonbString as postgresql_crud::PostgresqlJsonType>::Read as postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement>::default_but_option_is_always_some_and_vec_always_contains_one_element()),
                    }),
                },
                field_333: match <OptionDoggieAsNullableJsonbObject as postgresql_crud::tests::PostgresqlJsonTypeTestCases>::read_only_ids_to_option_value_read_inner(value.0.value.field_333) {
                    Some(value) => Some(value),
                    None => Some(postgresql_crud::Value {
                        value: <OptionDoggieAsNullableJsonbObject as postgresql_crud::PostgresqlJsonType>::into_inner(<<OptionDoggieAsNullableJsonbObject as postgresql_crud::PostgresqlJsonType>::Read as postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement>::default_but_option_is_always_some_and_vec_always_contains_one_element()),
                    }),
                },
                field_444: match <OptionDoggieAsNullableJsonbObject as postgresql_crud::tests::PostgresqlJsonTypeTestCases>::read_only_ids_to_option_value_read_inner(value.0.value.field_444) {
                    Some(value) => Some(value),
                    None => Some(postgresql_crud::Value {
                        value: <OptionDoggieAsNullableJsonbObject as postgresql_crud::PostgresqlJsonType>::into_inner(<<OptionDoggieAsNullableJsonbObject as postgresql_crud::PostgresqlJsonType>::Read as postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement>::default_but_option_is_always_some_and_vec_always_contains_one_element()),
                    }),
                },
            },
        })
    }
    fn update_to_read_only_ids(value: &<Self::Element as postgresql_crud::PostgresqlJsonType>::Update) -> <Self::Element as postgresql_crud::PostgresqlJsonType>::ReadOnlyIds {
        todo!()
    }
}

///////////
#[derive(Debug)]
pub struct DoggieAsNotNullJsonbObject;
#[derive(Debug)]
pub struct DoggieWithIdAsNotNullJsonbObjectWithId;
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct DoggieAsNotNullJsonbObjectTableTypeDeclaration {
    field_807: <VecOfCatWithIdAsNotNullArrayOfNotNullJsonbObjectWithId as postgresql_crud::PostgresqlJsonType>::TableTypeDeclaration,
}
impl DoggieAsNotNullJsonbObjectTableTypeDeclaration {
    pub fn new(field_807: <VecOfCatWithIdAsNotNullArrayOfNotNullJsonbObjectWithId as postgresql_crud::PostgresqlJsonType>::TableTypeDeclaration) -> Self {
        Self { field_807 }
    }
}
impl postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for DoggieAsNotNullJsonbObjectTableTypeDeclaration {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self {
            field_807: postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct DoggieWithIdAsNotNullJsonbObjectWithIdTableTypeDeclaration {
    id: <postgresql_crud::postgresql_json_type::UuidUuidAsNotNullJsonbString as postgresql_crud::PostgresqlJsonType>::TableTypeDeclaration,
    field_807: <VecOfCatWithIdAsNotNullArrayOfNotNullJsonbObjectWithId as postgresql_crud::PostgresqlJsonType>::TableTypeDeclaration,
}
impl DoggieWithIdAsNotNullJsonbObjectWithIdTableTypeDeclaration {
    pub fn new(id: <postgresql_crud::postgresql_json_type::UuidUuidAsNotNullJsonbString as postgresql_crud::PostgresqlJsonType>::TableTypeDeclaration, field_807: <VecOfCatWithIdAsNotNullArrayOfNotNullJsonbObjectWithId as postgresql_crud::PostgresqlJsonType>::TableTypeDeclaration) -> Self {
        Self { id, field_807 }
    }
}
impl postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for DoggieWithIdAsNotNullJsonbObjectWithIdTableTypeDeclaration {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self {
            id: postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            field_807: postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
        }
    }
}
impl DoggieAsNotNullJsonbObjectTableTypeDeclaration {
    pub fn create_table_column_query_part(column: &dyn std::fmt::Display, _: std::primitive::bool) -> impl std::fmt::Display {
        format!("{column} jsonb not null check (jsonb_matches_schema('{}', {column}))", serde_json::to_string(&schemars::schema_for!(DoggieAsNotNullJsonbObjectTableTypeDeclaration)).unwrap())
    }
}
impl DoggieWithIdAsNotNullJsonbObjectWithIdTableTypeDeclaration {
    pub fn create_table_column_query_part(column: &dyn std::fmt::Display, _: std::primitive::bool) -> impl std::fmt::Display {
        format!("{column} jsonb not null check (jsonb_matches_schema('{}', {column}))", serde_json::to_string(&schemars::schema_for!(DoggieWithIdAsNotNullJsonbObjectWithIdTableTypeDeclaration)).unwrap())
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct DoggieAsNotNullJsonbObjectCreate {
    field_807: <VecOfCatWithIdAsNotNullArrayOfNotNullJsonbObjectWithId as postgresql_crud::PostgresqlJsonType>::Create,
}
impl DoggieAsNotNullJsonbObjectCreate {
    pub fn new(field_807: <VecOfCatWithIdAsNotNullArrayOfNotNullJsonbObjectWithId as postgresql_crud::PostgresqlJsonType>::Create) -> Self {
        Self { field_807 }
    }
}
impl postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for DoggieAsNotNullJsonbObjectCreate {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self {
            field_807: postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct DoggieWithIdAsNotNullJsonbObjectWithIdCreate {
    field_807: <VecOfCatWithIdAsNotNullArrayOfNotNullJsonbObjectWithId as postgresql_crud::PostgresqlJsonType>::Create,
}
impl DoggieWithIdAsNotNullJsonbObjectWithIdCreate {
    pub fn new(field_807: <VecOfCatWithIdAsNotNullArrayOfNotNullJsonbObjectWithId as postgresql_crud::PostgresqlJsonType>::Create) -> Self {
        Self { field_807 }
    }
}
impl postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for DoggieWithIdAsNotNullJsonbObjectWithIdCreate {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self {
            field_807: postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
        }
    }
}
impl std::fmt::Display for DoggieAsNotNullJsonbObjectCreate {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{:?}", self)
    }
}
impl error_occurence_lib::ToStdStringString for DoggieAsNotNullJsonbObjectCreate {
    fn to_std_string_string(&self) -> std::string::String {
        format!("{self:?}")
    }
}
impl DoggieAsNotNullJsonbObjectCreate {
    fn create_query_part(&self, increment: &mut std::primitive::u64) -> Result<std::string::String, postgresql_crud::QueryPartErrorNamed> {
        let mut increments = std::string::String::from("");
        match <VecOfCatWithIdAsNotNullArrayOfNotNullJsonbObjectWithId as postgresql_crud::PostgresqlJsonType>::create_query_part(&self.field_807, increment) {
            Ok(value) => {
                increments.push_str(&postgresql_crud::wrap_into_jsonb_build_object("field_807", &value));
            }
            Err(error) => {
                return Err(error);
            }
        }
        let _ = increments.pop();
        let _ = increments.pop();
        Ok(format!("{increments}"))
    }
    fn create_query_bind(self, mut query: sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments> {
        query = <VecOfCatWithIdAsNotNullArrayOfNotNullJsonbObjectWithId as postgresql_crud::PostgresqlJsonType>::create_query_bind(self.field_807, query);
        query
    }
}
impl std::fmt::Display for DoggieWithIdAsNotNullJsonbObjectWithIdCreate {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{:?}", self)
    }
}
impl error_occurence_lib::ToStdStringString for DoggieWithIdAsNotNullJsonbObjectWithIdCreate {
    fn to_std_string_string(&self) -> std::string::String {
        format!("{self:?}")
    }
}
impl DoggieWithIdAsNotNullJsonbObjectWithIdCreate {
    fn create_query_part(&self, increment: &mut std::primitive::u64) -> Result<std::string::String, postgresql_crud::QueryPartErrorNamed> {
        let mut increments = std::string::String::from("");
        match <VecOfCatWithIdAsNotNullArrayOfNotNullJsonbObjectWithId as postgresql_crud::PostgresqlJsonType>::create_query_part(&self.field_807, increment) {
            Ok(value) => {
                increments.push_str(&postgresql_crud::wrap_into_jsonb_build_object("field_807", &value));
            }
            Err(error) => {
                return Err(error);
            }
        }
        let _ = increments.pop();
        let _ = increments.pop();
        Ok(format!("jsonb_build_object('id', to_jsonb(gen_random_uuid()))||{increments}"))
    }
    fn create_query_bind(self, mut query: sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments> {
        query = <VecOfCatWithIdAsNotNullArrayOfNotNullJsonbObjectWithId as postgresql_crud::PostgresqlJsonType>::create_query_bind(self.field_807, query);
        query
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct DoggieAsNotNullJsonbObjectSelect(postgresql_crud::NotEmptyUniqueEnumVec<DoggieAsNotNullJsonbObjectSelectElement>);
impl DoggieAsNotNullJsonbObjectSelect {
    pub fn new(value: postgresql_crud::NotEmptyUniqueEnumVec<DoggieAsNotNullJsonbObjectSelectElement>) -> Self {
        Self(value)
    }
}
impl DoggieAsNotNullJsonbObjectSelect {
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
                    DoggieAsNotNullJsonbObjectSelectElement::Field807(value) => <VecOfCatWithIdAsNotNullArrayOfNotNullJsonbObjectWithId as postgresql_crud::PostgresqlJsonType>::select_query_part(&value, "field_807", &column_name_and_maybe_field_getter_field_ident, &column_name_and_maybe_field_getter_for_error_message_field_ident, false,),
                }
            ));
        }
        let _ = acc.pop();
        let _ = acc.pop();
        if is_postgresql_type { format!("{acc}") } else { format!("jsonb_build_object('{field_ident}', jsonb_build_object('value',{acc}))") }
    }
}
impl sqlx::Type<sqlx::Postgres> for DoggieAsNotNullJsonbObjectSelect {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <sqlx::types::Json<Self> as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> std::primitive::bool {
        <sqlx::types::Json<Self> as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
impl sqlx::Decode<'_, sqlx::Postgres> for DoggieAsNotNullJsonbObjectSelect {
    fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        match <sqlx::types::Json<Self> as sqlx::Decode<sqlx::Postgres>>::decode(value) {
            Ok(value) => Ok(value.0),
            Err(error) => Err(error),
        }
    }
}
impl postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for DoggieAsNotNullJsonbObjectSelect {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self(postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element())
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub enum DoggieAsNotNullJsonbObjectSelectElement {
    #[serde(rename(serialize = "field_807", deserialize = "field_807"))]
    Field807(<VecOfCatWithIdAsNotNullArrayOfNotNullJsonbObjectWithId as postgresql_crud::PostgresqlJsonType>::Select),
}
impl error_occurence_lib::ToStdStringString for DoggieAsNotNullJsonbObjectSelectElement {
    fn to_std_string_string(&self) -> std::string::String {
        format!("{self:?}")
    }
}
impl postgresql_crud::AllEnumVariantsArrayDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for DoggieAsNotNullJsonbObjectSelectElement {
    fn all_enum_variants_array_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> std::vec::Vec<Self> {
        vec![DoggieAsNotNullJsonbObjectSelectElement::Field807(postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element())]
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct DoggieWithIdAsNotNullJsonbObjectWithIdSelect(postgresql_crud::NotEmptyUniqueEnumVec<DoggieWithIdAsNotNullJsonbObjectWithIdSelectElement>);
impl DoggieWithIdAsNotNullJsonbObjectWithIdSelect {
    pub fn new(value: postgresql_crud::NotEmptyUniqueEnumVec<DoggieWithIdAsNotNullJsonbObjectWithIdSelectElement>) -> Self {
        Self(value)
    }
}
impl sqlx::Type<sqlx::Postgres> for DoggieWithIdAsNotNullJsonbObjectWithIdSelect {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <sqlx::types::Json<Self> as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> std::primitive::bool {
        <sqlx::types::Json<Self> as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
impl sqlx::Decode<'_, sqlx::Postgres> for DoggieWithIdAsNotNullJsonbObjectWithIdSelect {
    fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        match <sqlx::types::Json<Self> as sqlx::Decode<sqlx::Postgres>>::decode(value) {
            Ok(value) => Ok(value.0),
            Err(error) => Err(error),
        }
    }
}
impl postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for DoggieWithIdAsNotNullJsonbObjectWithIdSelect {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self(postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element())
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub enum DoggieWithIdAsNotNullJsonbObjectWithIdSelectElement {
    #[serde(rename(serialize = "id", deserialize = "id"))]
    Id(<postgresql_crud::postgresql_json_type::UuidUuidAsNotNullJsonbString as postgresql_crud::PostgresqlJsonType>::Select),
    #[serde(rename(serialize = "field_807", deserialize = "field_807"))]
    Field807(<VecOfCatWithIdAsNotNullArrayOfNotNullJsonbObjectWithId as postgresql_crud::PostgresqlJsonType>::Select),
}
impl error_occurence_lib::ToStdStringString for DoggieWithIdAsNotNullJsonbObjectWithIdSelectElement {
    fn to_std_string_string(&self) -> std::string::String {
        format!("{self:?}")
    }
}
impl postgresql_crud::AllEnumVariantsArrayDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for DoggieWithIdAsNotNullJsonbObjectWithIdSelectElement {
    fn all_enum_variants_array_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> std::vec::Vec<Self> {
        vec![
            DoggieWithIdAsNotNullJsonbObjectWithIdSelectElement::Id(postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            DoggieWithIdAsNotNullJsonbObjectWithIdSelectElement::Field807(postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
        ]
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub enum DoggieAsNotNullJsonbObjectWhereElement {
    Field807(postgresql_crud::PostgresqlTypeWhere<<VecOfCatWithIdAsNotNullArrayOfNotNullJsonbObjectWithId as postgresql_crud::PostgresqlJsonType>::WhereElement>),
}
impl<'a> postgresql_crud::PostgresqlTypeWhereFilter<'a> for DoggieAsNotNullJsonbObjectWhereElement {
    fn query_part(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, postgresql_crud::QueryPartErrorNamed> {
        match &self {
            Self::Field807(value) => postgresql_crud::PostgresqlTypeWhereFilter::query_part(value, increment, &format!("{column}->'field_807'"), is_need_to_add_logical_operator),
        }
    }
    fn query_bind(self, query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
        match self {
            Self::Field807(value) => postgresql_crud::PostgresqlTypeWhereFilter::query_bind(value, query),
        }
    }
}
impl error_occurence_lib::ToStdStringString for DoggieAsNotNullJsonbObjectWhereElement {
    fn to_std_string_string(&self) -> std::string::String {
        format!("{self:?}")
    }
}
impl postgresql_crud::AllEnumVariantsArrayDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for DoggieAsNotNullJsonbObjectWhereElement {
    fn all_enum_variants_array_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> std::vec::Vec<Self> {
        vec![Self::Field807(postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element())]
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub enum DoggieWithIdAsNotNullJsonbObjectWithIdWhereElement {
    Id(postgresql_crud::PostgresqlTypeWhere<<postgresql_crud::postgresql_json_type::UuidUuidAsNotNullJsonbString as postgresql_crud::PostgresqlJsonType>::WhereElement>),
    Field807(postgresql_crud::PostgresqlTypeWhere<<VecOfCatWithIdAsNotNullArrayOfNotNullJsonbObjectWithId as postgresql_crud::PostgresqlJsonType>::WhereElement>),
}
impl<'a> postgresql_crud::PostgresqlTypeWhereFilter<'a> for DoggieWithIdAsNotNullJsonbObjectWithIdWhereElement {
    fn query_part(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, postgresql_crud::QueryPartErrorNamed> {
        match &self {
            Self::Id(value) => postgresql_crud::PostgresqlTypeWhereFilter::query_part(value, increment, &format!("{column}->'id'"), is_need_to_add_logical_operator),
            Self::Field807(value) => postgresql_crud::PostgresqlTypeWhereFilter::query_part(value, increment, &format!("{column}->'field_807'"), is_need_to_add_logical_operator),
        }
    }
    fn query_bind(self, query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
        match self {
            Self::Id(value) => postgresql_crud::PostgresqlTypeWhereFilter::query_bind(value, query),
            Self::Field807(value) => postgresql_crud::PostgresqlTypeWhereFilter::query_bind(value, query),
        }
    }
}
impl error_occurence_lib::ToStdStringString for DoggieWithIdAsNotNullJsonbObjectWithIdWhereElement {
    fn to_std_string_string(&self) -> std::string::String {
        format!("{self:?}")
    }
}
impl postgresql_crud::AllEnumVariantsArrayDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for DoggieWithIdAsNotNullJsonbObjectWithIdWhereElement {
    fn all_enum_variants_array_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> std::vec::Vec<Self> {
        vec![
            Self::Id(postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            Self::Field807(postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
        ]
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct DoggieAsNotNullJsonbObjectRead {
    #[serde(skip_serializing_if = "Option::is_none")]
    field_807: std::option::Option<postgresql_crud::Value<<VecOfCatWithIdAsNotNullArrayOfNotNullJsonbObjectWithId as postgresql_crud::PostgresqlJsonType>::Read>>,
}
impl DoggieAsNotNullJsonbObjectRead {
    fn into_inner(self) -> DoggieAsNotNullJsonbObjectReadInner {
        DoggieAsNotNullJsonbObjectReadInner {
            field_807: match self.field_807 {
                Some(value) => Some(postgresql_crud::Value {
                    value: <VecOfCatWithIdAsNotNullArrayOfNotNullJsonbObjectWithId as postgresql_crud::PostgresqlJsonType>::into_inner(value.value),
                }),
                None => None,
            },
        }
    }
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum DoggieAsNotNullJsonbObjectReadTryFromErrorNamed {
    AllFieldsAreNone { code_occurence: error_occurence_lib::code_occurence::CodeOccurence },
}
impl DoggieAsNotNullJsonbObjectRead {
    pub fn try_new(field_807: std::option::Option<postgresql_crud::Value<<VecOfCatWithIdAsNotNullArrayOfNotNullJsonbObjectWithId as postgresql_crud::PostgresqlJsonType>::Read>>) -> Result<Self, DoggieAsNotNullJsonbObjectReadTryFromErrorNamed> {
        if let None = &field_807 {
            return Err(DoggieAsNotNullJsonbObjectReadTryFromErrorNamed::AllFieldsAreNone { code_occurence: error_occurence_lib::code_occurence!() });
        }
        Ok(Self { field_807 })
    }
}
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for DoggieAsNotNullJsonbObjectRead {
        fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            #[doc(hidden)]
            enum __Field {
                __field0,
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
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
                fn visit_str<__E>(self, __value: &str) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        "field_807" => serde::__private::Ok(__Field::__field0),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
                fn visit_bytes<__E>(self, __value: &[u8]) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        b"field_807" => serde::__private::Ok(__Field::__field0),
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
                marker: _serde::__private::PhantomData<DoggieAsNotNullJsonbObjectRead>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            #[automatically_derived]
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = DoggieAsNotNullJsonbObjectRead;
                fn expecting(&self, __formatter: &mut _serde::__private::Formatter<'_>) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(__formatter, "struct DoggieAsNotNullJsonbObjectRead")
                }
                #[inline]
                fn visit_seq<__A>(self, mut __seq: __A) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::SeqAccess<'de>,
                {
                    let __field0 = match serde::de::SeqAccess::next_element::<std::option::Option<postgresql_crud::Value<<VecOfCatWithIdAsNotNullArrayOfNotNullJsonbObjectWithId as postgresql_crud::PostgresqlJsonType>::Read>>>(&mut __seq)? {
                        serde::__private::Some(__value) => __value,
                        serde::__private::None => {
                            return serde::__private::Err(serde::de::Error::invalid_length(0usize, &"struct DoggieAsNotNullJsonbObjectRead with 1 elements"));
                        }
                    };
                    match DoggieAsNotNullJsonbObjectRead::try_new(__field0) {
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
                    let mut __field0: serde::__private::Option<std::option::Option<postgresql_crud::Value<<VecOfCatWithIdAsNotNullArrayOfNotNullJsonbObjectWithId as postgresql_crud::PostgresqlJsonType>::Read>>> = serde::__private::None;
                    while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<__Field>(&mut __map)? {
                        match __key {
                            __Field::__field0 => {
                                if serde::__private::Option::is_some(&__field0) {
                                    return serde::__private::Err(<__A::Error as serde::de::Error>::duplicate_field("field_807"));
                                }
                                __field0 = serde::__private::Some(serde::de::MapAccess::next_value::<std::option::Option<postgresql_crud::Value<<VecOfCatWithIdAsNotNullArrayOfNotNullJsonbObjectWithId as postgresql_crud::PostgresqlJsonType>::Read>>>(&mut __map)?);
                            }
                            _ => {
                                let _ = _serde::de::MapAccess::next_value::<_serde::de::IgnoredAny>(&mut __map)?;
                            }
                        }
                    }
                    let __field0 = match __field0 {
                        serde::__private::Some(__field0) => __field0,
                        serde::__private::None => serde::__private::de::missing_field("field_807")?,
                    };
                    match DoggieAsNotNullJsonbObjectRead::try_new(__field0) {
                        Ok(value) => serde::__private::Ok(value),
                        Err(error) => {
                            return Err(serde::de::Error::custom(format!("{error:?}")));
                        }
                    }
                }
            }
            #[doc(hidden)]
            const FIELDS: &'static [&'static str] = &["field_807"];
            _serde::Deserializer::deserialize_struct(
                __deserializer,
                "DoggieAsNotNullJsonbObjectRead",
                FIELDS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<DoggieAsNotNullJsonbObjectRead>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
impl postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for DoggieAsNotNullJsonbObjectRead {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self {
            field_807: Some(postgresql_crud::Value {
                value: postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            }),
        }
    }
}
impl sqlx::Type<sqlx::Postgres> for DoggieAsNotNullJsonbObjectRead {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <sqlx::types::Json<Self> as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> std::primitive::bool {
        <sqlx::types::Json<Self> as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
impl sqlx::Decode<'_, sqlx::Postgres> for DoggieAsNotNullJsonbObjectRead {
    fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        match <sqlx::types::Json<Self> as sqlx::Decode<sqlx::Postgres>>::decode(value) {
            Ok(value) => Ok(value.0),
            Err(error) => Err(error),
        }
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct DoggieWithIdAsNotNullJsonbObjectWithIdRead {
    #[serde(skip_serializing_if = "Option::is_none")]
    id: std::option::Option<postgresql_crud::Value<<postgresql_crud::postgresql_json_type::UuidUuidAsNotNullJsonbString as postgresql_crud::PostgresqlJsonType>::Read>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    field_807: std::option::Option<postgresql_crud::Value<<VecOfCatWithIdAsNotNullArrayOfNotNullJsonbObjectWithId as postgresql_crud::PostgresqlJsonType>::Read>>,
}
impl DoggieWithIdAsNotNullJsonbObjectWithIdRead {
    fn into_inner(self) -> DoggieWithIdAsNotNullJsonbObjectWithIdReadInner {
        DoggieWithIdAsNotNullJsonbObjectWithIdReadInner {
            id: match self.id {
                Some(value) => Some(postgresql_crud::Value {
                    value: <postgresql_crud::postgresql_json_type::UuidUuidAsNotNullJsonbString as postgresql_crud::PostgresqlJsonType>::into_inner(value.value),
                }),
                None => None,
            },
            field_807: match self.field_807 {
                Some(value) => Some(postgresql_crud::Value {
                    value: <VecOfCatWithIdAsNotNullArrayOfNotNullJsonbObjectWithId as postgresql_crud::PostgresqlJsonType>::into_inner(value.value),
                }),
                None => None,
            },
        }
    }
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum DoggieWithIdAsNotNullJsonbObjectWithIdReadTryFromErrorNamed {
    AllFieldsAreNone { code_occurence: error_occurence_lib::code_occurence::CodeOccurence },
}
impl DoggieWithIdAsNotNullJsonbObjectWithIdRead {
    pub fn try_new(id: std::option::Option<postgresql_crud::Value<<postgresql_crud::postgresql_json_type::UuidUuidAsNotNullJsonbString as postgresql_crud::PostgresqlJsonType>::Read>>, field_807: std::option::Option<postgresql_crud::Value<<VecOfCatWithIdAsNotNullArrayOfNotNullJsonbObjectWithId as postgresql_crud::PostgresqlJsonType>::Read>>) -> Result<Self, DoggieWithIdAsNotNullJsonbObjectWithIdReadTryFromErrorNamed> {
        if let (None, None) = (&id, &field_807) {
            return Err(DoggieWithIdAsNotNullJsonbObjectWithIdReadTryFromErrorNamed::AllFieldsAreNone { code_occurence: error_occurence_lib::code_occurence!() });
        }
        Ok(Self { id, field_807 })
    }
}
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for DoggieWithIdAsNotNullJsonbObjectWithIdRead {
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
                        "id" => serde::__private::Ok(__Field::__field0),
                        "field_807" => serde::__private::Ok(__Field::__field1),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
                fn visit_bytes<__E>(self, __value: &[u8]) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        b"id" => serde::__private::Ok(__Field::__field0),
                        b"field_807" => serde::__private::Ok(__Field::__field1),
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
                marker: _serde::__private::PhantomData<DoggieWithIdAsNotNullJsonbObjectWithIdRead>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            #[automatically_derived]
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = DoggieWithIdAsNotNullJsonbObjectWithIdRead;
                fn expecting(&self, __formatter: &mut _serde::__private::Formatter<'_>) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(__formatter, "struct DoggieWithIdAsNotNullJsonbObjectWithIdRead")
                }
                #[inline]
                fn visit_seq<__A>(self, mut __seq: __A) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::SeqAccess<'de>,
                {
                    let __field0 = match serde::de::SeqAccess::next_element::<std::option::Option<postgresql_crud::Value<<postgresql_crud::postgresql_json_type::UuidUuidAsNotNullJsonbString as postgresql_crud::PostgresqlJsonType>::Read>>>(&mut __seq)? {
                        serde::__private::Some(__value) => __value,
                        serde::__private::None => {
                            return serde::__private::Err(serde::de::Error::invalid_length(0usize, &"struct DoggieWithIdAsNotNullJsonbObjectWithIdRead with 2 elements"));
                        }
                    };
                    let __field1 = match serde::de::SeqAccess::next_element::<std::option::Option<postgresql_crud::Value<<VecOfCatWithIdAsNotNullArrayOfNotNullJsonbObjectWithId as postgresql_crud::PostgresqlJsonType>::Read>>>(&mut __seq)? {
                        serde::__private::Some(__value) => __value,
                        serde::__private::None => {
                            return serde::__private::Err(serde::de::Error::invalid_length(0usize, &"struct DoggieWithIdAsNotNullJsonbObjectWithIdRead with 2 elements"));
                        }
                    };
                    match DoggieWithIdAsNotNullJsonbObjectWithIdRead::try_new(__field0, __field1) {
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
                    let mut __field0: serde::__private::Option<std::option::Option<postgresql_crud::Value<<postgresql_crud::postgresql_json_type::UuidUuidAsNotNullJsonbString as postgresql_crud::PostgresqlJsonType>::Read>>> = serde::__private::None;
                    let mut __field1: serde::__private::Option<std::option::Option<postgresql_crud::Value<<VecOfCatWithIdAsNotNullArrayOfNotNullJsonbObjectWithId as postgresql_crud::PostgresqlJsonType>::Read>>> = serde::__private::None;
                    while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<__Field>(&mut __map)? {
                        match __key {
                            __Field::__field0 => {
                                if serde::__private::Option::is_some(&__field0) {
                                    return serde::__private::Err(<__A::Error as serde::de::Error>::duplicate_field("id"));
                                }
                                __field0 = serde::__private::Some(serde::de::MapAccess::next_value::<std::option::Option<postgresql_crud::Value<<postgresql_crud::postgresql_json_type::UuidUuidAsNotNullJsonbString as postgresql_crud::PostgresqlJsonType>::Read>>>(&mut __map)?);
                            }
                            __Field::__field1 => {
                                if serde::__private::Option::is_some(&__field1) {
                                    return serde::__private::Err(<__A::Error as serde::de::Error>::duplicate_field("field_807"));
                                }
                                __field1 = serde::__private::Some(serde::de::MapAccess::next_value::<std::option::Option<postgresql_crud::Value<<VecOfCatWithIdAsNotNullArrayOfNotNullJsonbObjectWithId as postgresql_crud::PostgresqlJsonType>::Read>>>(&mut __map)?);
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
                        serde::__private::None => serde::__private::de::missing_field("field_807")?,
                    };
                    match DoggieWithIdAsNotNullJsonbObjectWithIdRead::try_new(__field0, __field1) {
                        Ok(value) => serde::__private::Ok(value),
                        Err(error) => {
                            return Err(serde::de::Error::custom(format!("{error:?}")));
                        }
                    }
                }
            }
            #[doc(hidden)]
            const FIELDS: &'static [&'static str] = &["id", "field_807"];
            _serde::Deserializer::deserialize_struct(
                __deserializer,
                "DoggieWithIdAsNotNullJsonbObjectWithIdRead",
                FIELDS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<DoggieWithIdAsNotNullJsonbObjectWithIdRead>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
impl postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for DoggieWithIdAsNotNullJsonbObjectWithIdRead {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self {
            id: Some(postgresql_crud::Value {
                value: postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            }),
            field_807: Some(postgresql_crud::Value {
                value: postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            }),
        }
    }
}
impl sqlx::Type<sqlx::Postgres> for DoggieWithIdAsNotNullJsonbObjectWithIdRead {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <sqlx::types::Json<Self> as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> std::primitive::bool {
        <sqlx::types::Json<Self> as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
impl sqlx::Decode<'_, sqlx::Postgres> for DoggieWithIdAsNotNullJsonbObjectWithIdRead {
    fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        match <sqlx::types::Json<Self> as sqlx::Decode<sqlx::Postgres>>::decode(value) {
            Ok(value) => Ok(value.0),
            Err(error) => Err(error),
        }
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct DoggieAsNotNullJsonbObjectReadOnlyIdsHandle {
    field_807: <VecOfCatWithIdAsNotNullArrayOfNotNullJsonbObjectWithId as postgresql_crud::PostgresqlJsonType>::ReadOnlyIds,
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct DoggieAsNotNullJsonbObjectReadOnlyIds(postgresql_crud::Value<DoggieAsNotNullJsonbObjectReadOnlyIdsHandle>);
impl sqlx::Decode<'_, sqlx::Postgres> for DoggieAsNotNullJsonbObjectReadOnlyIds {
    fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        match <sqlx::types::Json<Self> as sqlx::Decode<sqlx::Postgres>>::decode(value) {
            Ok(value) => Ok(value.0),
            Err(error) => Err(error),
        }
    }
}
impl sqlx::Type<sqlx::Postgres> for DoggieAsNotNullJsonbObjectReadOnlyIds {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <sqlx::types::Json<Self> as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> std::primitive::bool {
        <sqlx::types::Json<Self> as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct DoggieWithIdAsNotNullJsonbObjectWithIdReadOnlyIdsHandle {
    id: <postgresql_crud::postgresql_json_type::UuidUuidAsNotNullJsonbString as postgresql_crud::PostgresqlJsonType>::ReadOnlyIds,
    field_807: <VecOfCatWithIdAsNotNullArrayOfNotNullJsonbObjectWithId as postgresql_crud::PostgresqlJsonType>::ReadOnlyIds,
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct DoggieWithIdAsNotNullJsonbObjectWithIdReadOnlyIds(pub postgresql_crud::Value<DoggieWithIdAsNotNullJsonbObjectWithIdReadOnlyIdsHandle>);
impl sqlx::Decode<'_, sqlx::Postgres> for DoggieWithIdAsNotNullJsonbObjectWithIdReadOnlyIds {
    fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        match <sqlx::types::Json<Self> as sqlx::Decode<sqlx::Postgres>>::decode(value) {
            Ok(value) => Ok(value.0),
            Err(error) => Err(error),
        }
    }
}
impl sqlx::Type<sqlx::Postgres> for DoggieWithIdAsNotNullJsonbObjectWithIdReadOnlyIds {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <sqlx::types::Json<Self> as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> std::primitive::bool {
        <sqlx::types::Json<Self> as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct DoggieAsNotNullJsonbObjectReadInner {
    field_807: std::option::Option<postgresql_crud::Value<<VecOfCatWithIdAsNotNullArrayOfNotNullJsonbObjectWithId as postgresql_crud::PostgresqlJsonType>::ReadInner>>,
}
#[derive(Debug, Clone, PartialEq)]
pub struct DoggieWithIdAsNotNullJsonbObjectWithIdReadInner {
    id: std::option::Option<postgresql_crud::Value<<postgresql_crud::postgresql_json_type::UuidUuidAsNotNullJsonbString as postgresql_crud::PostgresqlJsonType>::ReadInner>>,
    field_807: std::option::Option<postgresql_crud::Value<<VecOfCatWithIdAsNotNullArrayOfNotNullJsonbObjectWithId as postgresql_crud::PostgresqlJsonType>::ReadInner>>,
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct DoggieAsNotNullJsonbObjectUpdate(postgresql_crud::NotEmptyUniqueEnumVec<DoggieAsNotNullJsonbObjectUpdateElement>);
impl DoggieAsNotNullJsonbObjectUpdate {
    pub fn new(value: postgresql_crud::NotEmptyUniqueEnumVec<DoggieAsNotNullJsonbObjectUpdateElement>) -> Self {
        Self(value)
    }
}
impl postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for DoggieAsNotNullJsonbObjectUpdate {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self(postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element())
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub enum DoggieAsNotNullJsonbObjectUpdateElement {
    #[serde(rename(serialize = "field_807", deserialize = "field_807"))]
    Field807(postgresql_crud::Value<<VecOfCatWithIdAsNotNullArrayOfNotNullJsonbObjectWithId as postgresql_crud::PostgresqlJsonType>::Update>),
}
impl postgresql_crud::AllEnumVariantsArrayDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for DoggieAsNotNullJsonbObjectUpdateElement {
    fn all_enum_variants_array_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> std::vec::Vec<Self> {
        vec![DoggieAsNotNullJsonbObjectUpdateElement::Field807(postgresql_crud::Value {
            value: postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
        })]
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct DoggieWithIdAsNotNullJsonbObjectWithIdUpdateElement {
    id: <postgresql_crud::postgresql_json_type::UuidUuidAsNotNullJsonbString as postgresql_crud::PostgresqlJsonType>::Update,
    fields: <DoggieAsNotNullJsonbObject as postgresql_crud::PostgresqlJsonType>::Update,
}
impl DoggieWithIdAsNotNullJsonbObjectWithIdUpdateElement {
    pub fn new(id: <postgresql_crud::postgresql_json_type::UuidUuidAsNotNullJsonbString as postgresql_crud::PostgresqlJsonType>::Update, fields: <DoggieAsNotNullJsonbObject as postgresql_crud::PostgresqlJsonType>::Update) -> Self {
        Self { id, fields }
    }
}
impl postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for DoggieWithIdAsNotNullJsonbObjectWithIdUpdateElement {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self {
            id: postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            fields: postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct DoggieWithIdAsNotNullJsonbObjectWithIdUpdateHandle(postgresql_crud::UniqueVec<DoggieWithIdAsNotNullJsonbObjectWithIdUpdateElement>);
impl postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for DoggieWithIdAsNotNullJsonbObjectWithIdUpdateHandle {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self(postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element())
    }
}
impl DoggieWithIdAsNotNullJsonbObjectWithIdUpdateHandle {
    pub fn new(value: postgresql_crud::UniqueVec<DoggieWithIdAsNotNullJsonbObjectWithIdUpdateElement>) -> Self {
        Self(value)
    }
}
impl std::default::Default for DoggieWithIdAsNotNullJsonbObjectWithIdUpdateHandle {
    fn default() -> Self {
        Self(postgresql_crud::UniqueVec::default())
    }
}
impl postgresql_crud::PostgresqlJsonType for DoggieAsNotNullJsonbObject {
    type TableTypeDeclaration = DoggieAsNotNullJsonbObjectTableTypeDeclaration;
    type Create = DoggieAsNotNullJsonbObjectCreate;
    fn create_query_part(value: &Self::Create, increment: &mut std::primitive::u64) -> Result<std::string::String, postgresql_crud::QueryPartErrorNamed> {
        value.create_query_part(increment)
    }
    fn create_query_bind(value: Self::Create, query: sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments> {
        value.create_query_bind(query)
    }
    type Select = DoggieAsNotNullJsonbObjectSelect;
    fn select_query_part(value: &Self::Select, field_ident: &std::primitive::str, column_name_and_maybe_field_getter: &std::primitive::str, column_name_and_maybe_field_getter_for_error_message: &std::primitive::str, is_postgresql_type: std::primitive::bool) -> std::string::String {
        let mut acc = std::string::String::default();
        let column_name_and_maybe_field_getter_field_ident = if is_postgresql_type { column_name_and_maybe_field_getter.to_string() } else { format!("{column_name_and_maybe_field_getter}->'{field_ident}'") };
        let column_name_and_maybe_field_getter_for_error_message_field_ident = format!("{column_name_and_maybe_field_getter_for_error_message}.{field_ident}");
        for element in value.0.to_vec() {
            acc.push_str(&format!(
                "{}||",
                match element {
                    DoggieAsNotNullJsonbObjectSelectElement::Field807(value) => <VecOfCatWithIdAsNotNullArrayOfNotNullJsonbObjectWithId as postgresql_crud::PostgresqlJsonType>::select_query_part(&value, "field_807", &column_name_and_maybe_field_getter_field_ident, &column_name_and_maybe_field_getter_for_error_message_field_ident, false,),
                }
            ));
        }
        let _ = acc.pop();
        let _ = acc.pop();
        if is_postgresql_type { format!("{acc}") } else { format!("jsonb_build_object('{field_ident}', jsonb_build_object('value',{acc}))") }
    }
    type WhereElement = DoggieAsNotNullJsonbObjectWhereElement;
    type Read = DoggieAsNotNullJsonbObjectRead;
    type ReadOnlyIds = DoggieAsNotNullJsonbObjectReadOnlyIds;
    fn select_only_ids_query_part(column_name_and_maybe_field_getter: &std::primitive::str) -> std::string::String {
        format!("jsonb_build_object('value',jsonb_build_object('field_807',{}))", <VecOfCatWithIdAsNotNullArrayOfNotNullJsonbObjectWithId as postgresql_crud::PostgresqlJsonType>::select_only_ids_query_part(&format!("{column_name_and_maybe_field_getter}->'field_807'")))
    }
    type ReadInner = DoggieAsNotNullJsonbObjectReadInner;
    fn into_inner(value: Self::Read) -> Self::ReadInner {
        value.into_inner()
    }
    type Update = DoggieAsNotNullJsonbObjectUpdate;
    fn update_query_part(value: &Self::Update, jsonb_set_accumulator: &std::primitive::str, jsonb_set_target: &std::primitive::str, jsonb_set_path: &std::primitive::str, increment: &mut std::primitive::u64) -> Result<std::string::String, postgresql_crud::QueryPartErrorNamed> {
        let mut std_option_option_object_acc = format!("case when jsonb_typeof({jsonb_set_target}) = 'object' then ({jsonb_set_target})::jsonb else '{{}}'::jsonb end");
        let generate_jsonb_set_target = |value: &std::primitive::str| format!("{jsonb_set_target}->'{value}'");
        for element in value.0.to_vec() {
            match element {
                DoggieAsNotNullJsonbObjectUpdateElement::Field807(value) => match <VecOfCatWithIdAsNotNullArrayOfNotNullJsonbObjectWithId as postgresql_crud::PostgresqlJsonType>::update_query_part(&value.value, &std_option_option_object_acc, &generate_jsonb_set_target("field_807"), "field_807", increment) {
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
    fn update_query_bind(value: Self::Update, mut query: sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments> {
        for element in value.0.into_vec() {
            match element {
                DoggieAsNotNullJsonbObjectUpdateElement::Field807(value) => {
                    query = <VecOfCatWithIdAsNotNullArrayOfNotNullJsonbObjectWithId as postgresql_crud::PostgresqlJsonType>::update_query_bind(value.value, query);
                }
            }
        }
        query
    }
    fn select_only_updated_ids_query_part(value: &Self::Update, field_ident: &std::primitive::str, column_name_and_maybe_field_getter: &std::primitive::str, increment: &mut std::primitive::u64) -> Result<std::string::String, postgresql_crud::QueryPartErrorNamed> {
        let mut acc = std::string::String::default();
        for element in value.0.to_vec() {
            match &element {
                DoggieAsNotNullJsonbObjectUpdateElement::Field807(value) => match <VecOfCatWithIdAsNotNullArrayOfNotNullJsonbObjectWithId as postgresql_crud::PostgresqlJsonType>::select_only_updated_ids_query_part(&value.value, &"field_807", &format!("{column_name_and_maybe_field_getter}->'{field_ident}'"), increment) {
                    Ok(value) => {
                        acc.push_str(&value);
                    }
                    Err(error) => {
                        return Err(error);
                    }
                },
            }
        }
        let _ = acc.pop();
        Ok(format!("'{field_ident}',jsonb_build_object('value',jsonb_build_object({acc})),"))
    }
}
impl postgresql_crud::PostgresqlType for DoggieAsNotNullJsonbObject {
    type TableTypeDeclaration = DoggieAsNotNullJsonbObjectTableTypeDeclaration;
    type Create = DoggieAsNotNullJsonbObjectCreate;
    fn create_query_part(value: &Self::Create, increment: &mut std::primitive::u64) -> Result<std::string::String, postgresql_crud::QueryPartErrorNamed> {
        value.create_query_part(increment)
    }
    fn create_query_bind(value: Self::Create, query: sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments> {
        value.create_query_bind(query)
    }
    type Select = DoggieAsNotNullJsonbObjectSelect;
    fn select_query_part(value: &Self::Select, column: &std::primitive::str) -> std::string::String {
        format!("{} as {column}", value.select_query_part_postgresql_type(column))
    }
    type WhereElement = DoggieAsNotNullJsonbObjectWhereElement;
    type Read = DoggieAsNotNullJsonbObjectRead;
    fn normalize(value: Self::Read) -> Self::Read {
        value
    }
    type ReadOnlyIds = DoggieAsNotNullJsonbObjectReadOnlyIds;
    fn select_only_ids_query_part(column: &std::primitive::str, is_primary_key: std::primitive::bool) -> std::string::String {
        format!("{} as {column},", <DoggieAsNotNullJsonbObject as postgresql_crud::PostgresqlJsonType>::select_only_ids_query_part(&column))
    }
    type ReadInner = DoggieAsNotNullJsonbObjectReadInner;
    fn into_inner(value: Self::Read) -> Self::ReadInner {
        value.into_inner()
    }
    type Update = DoggieAsNotNullJsonbObjectUpdate;
    fn update_query_part(value: &Self::Update, jsonb_set_accumulator: &std::primitive::str, jsonb_set_target: &std::primitive::str, jsonb_set_path: &std::primitive::str, increment: &mut std::primitive::u64) -> Result<std::string::String, postgresql_crud::QueryPartErrorNamed> {
        let mut std_option_option_object_acc = format!("case when jsonb_typeof({jsonb_set_target}) = 'object' then ({jsonb_set_target})::jsonb else '{{}}'::jsonb end");
        let generate_jsonb_set_target = |value: &std::primitive::str| format!("{jsonb_set_target}->'{value}'");
        for element in value.0.to_vec() {
            match element {
                DoggieAsNotNullJsonbObjectUpdateElement::Field807(value) => match <VecOfCatWithIdAsNotNullArrayOfNotNullJsonbObjectWithId as postgresql_crud::PostgresqlJsonType>::update_query_part(&value.value, &std_option_option_object_acc, &generate_jsonb_set_target("field_807"), "field_807", increment) {
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
    fn update_query_bind<'a>(value: Self::Update, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
        for element in value.0.into_vec() {
            match element {
                DoggieAsNotNullJsonbObjectUpdateElement::Field807(value) => {
                    query = <VecOfCatWithIdAsNotNullArrayOfNotNullJsonbObjectWithId as postgresql_crud::PostgresqlJsonType>::update_query_bind(value.value, query);
                }
            }
        }
        query
    }
    fn select_only_updated_ids_query_part(value: &Self::Update, column: &std::primitive::str, increment: &mut std::primitive::u64, is_primary_key: std::primitive::bool) -> Result<std::string::String, postgresql_crud::QueryPartErrorNamed> {
        let mut acc = std::string::String::default();
        for element in value.0.to_vec() {
            match &element {
                DoggieAsNotNullJsonbObjectUpdateElement::Field807(value) => match <VecOfCatWithIdAsNotNullArrayOfNotNullJsonbObjectWithId as postgresql_crud::PostgresqlJsonType>::select_only_updated_ids_query_part(&value.value, &"field_807", &column, increment) {
                    Ok(value) => {
                        acc.push_str(&value);
                    }
                    Err(error) => {
                        return Err(error);
                    }
                },
            }
        }
        let _ = acc.pop();
        Ok(format!("jsonb_build_object('value',jsonb_build_object({acc})) as {column},"))
    }
}
impl postgresql_crud::PostgresqlJsonType for DoggieWithIdAsNotNullJsonbObjectWithId {
    type TableTypeDeclaration = DoggieWithIdAsNotNullJsonbObjectWithIdTableTypeDeclaration;
    type Create = DoggieWithIdAsNotNullJsonbObjectWithIdCreate;
    fn create_query_part(value: &Self::Create, increment: &mut std::primitive::u64) -> Result<std::string::String, postgresql_crud::QueryPartErrorNamed> {
        value.create_query_part(increment)
    }
    fn create_query_bind(value: Self::Create, query: sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments> {
        value.create_query_bind(query)
    }
    type Select = DoggieWithIdAsNotNullJsonbObjectWithIdSelect;
    fn select_query_part(value: &Self::Select, field_ident: &std::primitive::str, column_name_and_maybe_field_getter: &std::primitive::str, column_name_and_maybe_field_getter_for_error_message: &std::primitive::str, is_postgresql_type: std::primitive::bool) -> std::string::String {
        let mut acc = std::string::String::default();
        let column_name_and_maybe_field_getter_field_ident = if is_postgresql_type { column_name_and_maybe_field_getter.to_string() } else { format!("{column_name_and_maybe_field_getter}->'{field_ident}'") };
        let column_name_and_maybe_field_getter_for_error_message_field_ident = format!("{column_name_and_maybe_field_getter_for_error_message}.{field_ident}");
        for element in value.0.to_vec() {
            acc.push_str(&format!(
                "{}||",
                match element {
                    DoggieWithIdAsNotNullJsonbObjectWithIdSelectElement::Id(value) => <postgresql_crud::postgresql_json_type::UuidUuidAsNotNullJsonbString as postgresql_crud::PostgresqlJsonType>::select_query_part(&value, "id", &column_name_and_maybe_field_getter_field_ident, &column_name_and_maybe_field_getter_for_error_message_field_ident, false,),
                    DoggieWithIdAsNotNullJsonbObjectWithIdSelectElement::Field807(value) => <VecOfCatWithIdAsNotNullArrayOfNotNullJsonbObjectWithId as postgresql_crud::PostgresqlJsonType>::select_query_part(&value, "field_807", &column_name_and_maybe_field_getter_field_ident, &column_name_and_maybe_field_getter_for_error_message_field_ident, false,),
                }
            ));
        }
        let _ = acc.pop();
        let _ = acc.pop();
        if is_postgresql_type { format!("{acc}") } else { format!("jsonb_build_object('{field_ident}', jsonb_build_object('value',{acc}))") }
    }
    type WhereElement = DoggieWithIdAsNotNullJsonbObjectWithIdWhereElement;
    type Read = DoggieWithIdAsNotNullJsonbObjectWithIdRead;
    type ReadOnlyIds = DoggieWithIdAsNotNullJsonbObjectWithIdReadOnlyIds;
    fn select_only_ids_query_part(column_name_and_maybe_field_getter: &std::primitive::str) -> std::string::String {
        format!(
            "jsonb_build_object('value',jsonb_build_object('id',{},'field_807',{}))",
            <postgresql_crud::postgresql_json_type::UuidUuidAsNotNullJsonbString as postgresql_crud::PostgresqlJsonType>::select_only_ids_query_part(&format!("{column_name_and_maybe_field_getter}->'id'")),
            <VecOfCatWithIdAsNotNullArrayOfNotNullJsonbObjectWithId as postgresql_crud::PostgresqlJsonType>::select_only_ids_query_part(&format!("{column_name_and_maybe_field_getter}->'field_807'"))
        )
    }
    type ReadInner = DoggieWithIdAsNotNullJsonbObjectWithIdReadInner;
    fn into_inner(value: Self::Read) -> Self::ReadInner {
        value.into_inner()
    }
    type Update = DoggieWithIdAsNotNullJsonbObjectWithIdUpdateHandle;
    fn update_query_part(value: &Self::Update, jsonb_set_accumulator: &std::primitive::str, jsonb_set_target: &std::primitive::str, jsonb_set_path: &std::primitive::str, increment: &mut std::primitive::u64) -> Result<std::string::String, postgresql_crud::QueryPartErrorNamed> {
        todo!()
    }
    fn update_query_bind(value: Self::Update, query: sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments> {
        todo!()
    }
    fn select_only_updated_ids_query_part(value: &Self::Update, field_ident: &std::primitive::str, column_name_and_maybe_field_getter: &std::primitive::str, increment: &mut std::primitive::u64) -> Result<std::string::String, postgresql_crud::QueryPartErrorNamed> {
        let mut acc = std::string::String::new();
        for element in value.0.to_vec() {
            acc.push_str(&format!("jsonb_build_object('id',jsonb_build_object('value','{}'),{})||", element.id.get_inner(), {
                let mut acc = std::string::String::new();
                for element in element.fields.0.to_vec() {
                    match &element {
                        DoggieAsNotNullJsonbObjectUpdateElement::Field807(value) => match <VecOfCatWithIdAsNotNullArrayOfNotNullJsonbObjectWithId as postgresql_crud::PostgresqlJsonType>::select_only_updated_ids_query_part(&value.value, &"field_807", &column_name_and_maybe_field_getter, increment) {
                            Ok(value) => {
                                acc.push_str(&value);
                            }
                            Err(error) => {
                                return Err(error);
                            }
                        },
                    }
                }
                let _ = acc.pop();
                acc
            }));
        }
        let _ = acc.pop();
        let _ = acc.pop();
        Ok(format!("jsonb_build_object('value',{acc})"))
    }
}
#[cfg(test)]
impl postgresql_crud::tests::PostgresqlTypeTestCases for DoggieAsNotNullJsonbObject {
    type Element = Self;
    fn test_cases(read_only_ids: &<Self::Element as postgresql_crud::PostgresqlType>::ReadOnlyIds) -> std::vec::Vec<std::vec::Vec<<Self::Element as postgresql_crud::PostgresqlType>::ReadInner>> {
        let mut acc = vec![];
        for element in <VecOfCatWithIdAsNotNullArrayOfNotNullJsonbObjectWithId as postgresql_crud::tests::PostgresqlJsonTypeTestCases>::test_cases(&read_only_ids.0.value.field_807) {
            for current_element in element {
                acc.push(DoggieAsNotNullJsonbObjectReadInner { field_807: Some(postgresql_crud::Value { value: current_element }) });
            }
        }
        vec![acc]
    }
    fn read_new_or_try_new_unwraped_for_test(value: DoggieAsNotNullJsonbObjectReadInner) -> <Self::Element as postgresql_crud::PostgresqlType>::Read {
        <Self::Element as postgresql_crud::PostgresqlType>::Read::try_new(match value.field_807 {
            Some(value) => Some(postgresql_crud::Value {
                value: <VecOfCatWithIdAsNotNullArrayOfNotNullJsonbObjectWithId as postgresql_crud::tests::PostgresqlJsonTypeTestCases>::read_new_or_try_new_unwraped_for_test(value.value),
            }),
            None => None,
        })
        .unwrap()
    }
    fn update_new_or_try_new_unwraped_for_test(value: DoggieAsNotNullJsonbObjectReadInner) -> <Self::Element as postgresql_crud::PostgresqlType>::Update {
        <Self::Element as postgresql_crud::PostgresqlType>::Update::new(
            postgresql_crud::NotEmptyUniqueEnumVec::try_new({
                let mut acc = vec![];
                if let Some(value) = value.field_807 {
                    acc.push(DoggieAsNotNullJsonbObjectUpdateElement::Field807(postgresql_crud::Value {
                        value: <VecOfCatWithIdAsNotNullArrayOfNotNullJsonbObjectWithId as postgresql_crud::tests::PostgresqlJsonTypeTestCases>::update_new_or_try_new_unwraped_for_test(value.value),
                    }));
                }
                acc
            })
            .unwrap(),
        )
    }
    fn read_only_ids_to_option_value_read_inner(value: <Self::Element as postgresql_crud::PostgresqlType>::ReadOnlyIds) -> std::option::Option<postgresql_crud::Value<<Self::Element as postgresql_crud::PostgresqlType>::ReadInner>> {
        todo!()
    }
    fn update_to_read_only_ids(value: &<Self::Element as postgresql_crud::PostgresqlType>::Update) -> <Self::Element as postgresql_crud::PostgresqlType>::ReadOnlyIds {
        let mut field_807 = None;
        for element in value.0.to_vec() {
            match element {
                DoggieAsNotNullJsonbObjectUpdateElement::Field807(value) => {
                    field_807 = Some(<VecOfCatWithIdAsNotNullArrayOfNotNullJsonbObjectWithId as postgresql_crud::tests::PostgresqlJsonTypeTestCases>::update_to_read_only_ids(&value.value));
                }
            }
        }
        DoggieAsNotNullJsonbObjectReadOnlyIds(postgresql_crud::Value {
            value: DoggieAsNotNullJsonbObjectReadOnlyIdsHandle { field_807: field_807.expect("expect 106f16f2-ae03-48b3-9239-f4b1b60746d5") },
        })
    }
}
#[cfg(test)]
impl postgresql_crud::tests::PostgresqlJsonTypeTestCases for DoggieAsNotNullJsonbObject {
    type Element = Self;
    fn test_cases(read_only_ids: &<Self::Element as postgresql_crud::PostgresqlJsonType>::ReadOnlyIds) -> std::vec::Vec<std::vec::Vec<<Self::Element as postgresql_crud::PostgresqlJsonType>::ReadInner>> {
        let mut acc = vec![];
        for element in <VecOfCatWithIdAsNotNullArrayOfNotNullJsonbObjectWithId as postgresql_crud::tests::PostgresqlJsonTypeTestCases>::test_cases(&read_only_ids.0.value.field_807) {
            for current_element in element {
                acc.push(DoggieAsNotNullJsonbObjectReadInner { field_807: Some(postgresql_crud::Value { value: current_element }) });
            }
        }
        vec![acc]
    }
    fn read_new_or_try_new_unwraped_for_test(value: DoggieAsNotNullJsonbObjectReadInner) -> <Self::Element as postgresql_crud::PostgresqlJsonType>::Read {
        <Self::Element as postgresql_crud::PostgresqlType>::Read::try_new(match value.field_807 {
            Some(value) => Some(postgresql_crud::Value {
                value: <VecOfCatWithIdAsNotNullArrayOfNotNullJsonbObjectWithId as postgresql_crud::tests::PostgresqlJsonTypeTestCases>::read_new_or_try_new_unwraped_for_test(value.value),
            }),
            None => None,
        })
        .unwrap()
    }
    fn update_new_or_try_new_unwraped_for_test(value: DoggieAsNotNullJsonbObjectReadInner) -> <Self::Element as postgresql_crud::PostgresqlJsonType>::Update {
        <Self::Element as postgresql_crud::PostgresqlType>::Update::new(
            postgresql_crud::NotEmptyUniqueEnumVec::try_new({
                let mut acc = vec![];
                if let Some(value) = value.field_807 {
                    acc.push(DoggieAsNotNullJsonbObjectUpdateElement::Field807(postgresql_crud::Value {
                        value: <VecOfCatWithIdAsNotNullArrayOfNotNullJsonbObjectWithId as postgresql_crud::tests::PostgresqlJsonTypeTestCases>::update_new_or_try_new_unwraped_for_test(value.value),
                    }));
                }
                acc
            })
            .unwrap(),
        )
    }
    fn read_only_ids_to_option_value_read_inner(value: <Self::Element as postgresql_crud::PostgresqlJsonType>::ReadOnlyIds) -> std::option::Option<postgresql_crud::Value<<Self::Element as postgresql_crud::PostgresqlJsonType>::ReadInner>> {
        Some(postgresql_crud::Value {
            value: DoggieAsNotNullJsonbObjectReadInner {
                field_807: match <VecOfCatWithIdAsNotNullArrayOfNotNullJsonbObjectWithId as postgresql_crud::tests::PostgresqlJsonTypeTestCases>::read_only_ids_to_option_value_read_inner(value.0.value.field_807) {
                    Some(value) => Some(value),
                    None => Some(postgresql_crud::Value {
                        value: <VecOfCatWithIdAsNotNullArrayOfNotNullJsonbObjectWithId as postgresql_crud::PostgresqlJsonType>::into_inner(<<VecOfCatWithIdAsNotNullArrayOfNotNullJsonbObjectWithId as postgresql_crud::PostgresqlJsonType>::Read as postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement>::default_but_option_is_always_some_and_vec_always_contains_one_element()),
                    }),
                },
            },
        })
    }
    fn update_to_read_only_ids(value: &<Self::Element as postgresql_crud::PostgresqlJsonType>::Update) -> <Self::Element as postgresql_crud::PostgresqlJsonType>::ReadOnlyIds {
        let mut field_807 = None;
        for element in value.0.to_vec() {
            match element {
                DoggieAsNotNullJsonbObjectUpdateElement::Field807(value) => {
                    field_807 = Some(<VecOfCatWithIdAsNotNullArrayOfNotNullJsonbObjectWithId as postgresql_crud::tests::PostgresqlJsonTypeTestCases>::update_to_read_only_ids(&value.value));
                }
            }
        }
        DoggieAsNotNullJsonbObjectReadOnlyIds(postgresql_crud::Value {
            value: DoggieAsNotNullJsonbObjectReadOnlyIdsHandle { field_807: field_807.expect("expect 106f16f2-ae03-48b3-9239-f4b1b60746d5") },
        })
    }
}
#[cfg(test)]
impl postgresql_crud::tests::PostgresqlJsonTypeTestCases for DoggieWithIdAsNotNullJsonbObjectWithId {
    type Element = Self;
    fn test_cases(read_only_ids: &<Self::Element as postgresql_crud::PostgresqlJsonType>::ReadOnlyIds) -> std::vec::Vec<std::vec::Vec<<Self::Element as postgresql_crud::PostgresqlJsonType>::ReadInner>> {
        vec![vec![DoggieWithIdAsNotNullJsonbObjectWithIdReadInner {
            id: Some(postgresql_crud::Value { value: read_only_ids.0.value.id.0.value.clone() }),
            field_807: <VecOfCatWithIdAsNotNullArrayOfNotNullJsonbObjectWithId as postgresql_crud::tests::PostgresqlJsonTypeTestCases>::read_only_ids_to_option_value_read_inner(read_only_ids.0.value.field_807.clone()),
        }]]
    }
    fn read_new_or_try_new_unwraped_for_test(value: DoggieWithIdAsNotNullJsonbObjectWithIdReadInner) -> <Self::Element as postgresql_crud::PostgresqlJsonType>::Read {
        DoggieWithIdAsNotNullJsonbObjectWithIdRead {
            id: match value.id {
                Some(value) => Some(postgresql_crud::Value {
                    value: <postgresql_crud::postgresql_json_type::UuidUuidAsNotNullJsonbString as postgresql_crud::tests::PostgresqlJsonTypeTestCases>::read_new_or_try_new_unwraped_for_test(value.value),
                }),
                None => None,
            },
            field_807: match value.field_807 {
                Some(value) => Some(postgresql_crud::Value {
                    value: <VecOfCatWithIdAsNotNullArrayOfNotNullJsonbObjectWithId as postgresql_crud::tests::PostgresqlJsonTypeTestCases>::read_new_or_try_new_unwraped_for_test(value.value),
                }),
                None => None,
            },
        }
    }
    fn update_new_or_try_new_unwraped_for_test(value: DoggieWithIdAsNotNullJsonbObjectWithIdReadInner) -> <Self::Element as postgresql_crud::PostgresqlJsonType>::Update {
        todo!()
    }
    fn read_only_ids_to_option_value_read_inner(value: <Self::Element as postgresql_crud::PostgresqlJsonType>::ReadOnlyIds) -> std::option::Option<postgresql_crud::Value<<Self::Element as postgresql_crud::PostgresqlJsonType>::ReadInner>> {
        Some(postgresql_crud::Value {
            value: DoggieWithIdAsNotNullJsonbObjectWithIdReadInner {
                id: match <postgresql_crud::postgresql_json_type::UuidUuidAsNotNullJsonbString as postgresql_crud::tests::PostgresqlJsonTypeTestCases>::read_only_ids_to_option_value_read_inner(value.0.value.id) {
                    Some(value) => Some(value),
                    None => Some(postgresql_crud::Value {
                        value: <postgresql_crud::postgresql_json_type::UuidUuidAsNotNullJsonbString as postgresql_crud::PostgresqlJsonType>::into_inner(<<postgresql_crud::postgresql_json_type::UuidUuidAsNotNullJsonbString as postgresql_crud::PostgresqlJsonType>::Read as postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement>::default_but_option_is_always_some_and_vec_always_contains_one_element()),
                    }),
                },
                field_807: match <VecOfCatWithIdAsNotNullArrayOfNotNullJsonbObjectWithId as postgresql_crud::tests::PostgresqlJsonTypeTestCases>::read_only_ids_to_option_value_read_inner(value.0.value.field_807) {
                    Some(value) => Some(value),
                    None => Some(postgresql_crud::Value {
                        value: <VecOfCatWithIdAsNotNullArrayOfNotNullJsonbObjectWithId as postgresql_crud::PostgresqlJsonType>::into_inner(<<VecOfCatWithIdAsNotNullArrayOfNotNullJsonbObjectWithId as postgresql_crud::PostgresqlJsonType>::Read as postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement>::default_but_option_is_always_some_and_vec_always_contains_one_element()),
                    }),
                },
            },
        })
    }
    fn update_to_read_only_ids(value: &<Self::Element as postgresql_crud::PostgresqlJsonType>::Update) -> <Self::Element as postgresql_crud::PostgresqlJsonType>::ReadOnlyIds {
        todo!()
    }
}
#[derive(Debug)]
pub struct OptionDoggieAsNullableJsonbObject;
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionDoggieAsNullableJsonbObjectTableTypeDeclaration(std::option::Option<<DoggieAsNotNullJsonbObject as postgresql_crud::PostgresqlJsonType>::TableTypeDeclaration>);
impl OptionDoggieAsNullableJsonbObjectTableTypeDeclaration {
    pub fn new(value: std::option::Option<<DoggieAsNotNullJsonbObject as postgresql_crud::PostgresqlJsonType>::TableTypeDeclaration>) -> Self {
        Self(value)
    }
}
impl postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for OptionDoggieAsNullableJsonbObjectTableTypeDeclaration {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self(Some(postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()))
    }
}
impl OptionDoggieAsNullableJsonbObjectTableTypeDeclaration {
    pub fn create_table_column_query_part(column: &dyn std::fmt::Display, _: std::primitive::bool) -> impl std::fmt::Display {
        format!("{column} jsonb not null check (jsonb_matches_schema('{}', {column}))", serde_json::to_string(&schemars::schema_for!(OptionDoggieAsNullableJsonbObjectTableTypeDeclaration)).unwrap())
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionDoggieAsNullableJsonbObjectCreate(std::option::Option<<DoggieAsNotNullJsonbObject as postgresql_crud::PostgresqlJsonType>::Create>);
impl OptionDoggieAsNullableJsonbObjectCreate {
    pub fn new(value: std::option::Option<<DoggieAsNotNullJsonbObject as postgresql_crud::PostgresqlJsonType>::Create>) -> Self {
        Self(value)
    }
}
impl postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for OptionDoggieAsNullableJsonbObjectCreate {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self(Some(postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()))
    }
}
impl std::fmt::Display for OptionDoggieAsNullableJsonbObjectCreate {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{:?}", self)
    }
}
impl error_occurence_lib::ToStdStringString for OptionDoggieAsNullableJsonbObjectCreate {
    fn to_std_string_string(&self) -> std::string::String {
        format!("{self:?}")
    }
}
impl OptionDoggieAsNullableJsonbObjectCreate {
    fn create_query_part(&self, increment: &mut std::primitive::u64) -> Result<std::string::String, postgresql_crud::QueryPartErrorNamed> {
        match &self.0 {
            Some(value) => <DoggieAsNotNullJsonbObject as postgresql_crud::PostgresqlJsonType>::create_query_part(value, increment),
            None => match increment.checked_add(1) {
                Some(value) => {
                    *increment = value;
                    Ok(format!("${increment}"))
                }
                None => Err(postgresql_crud::QueryPartErrorNamed::CheckedAdd { code_occurence: error_occurence_lib::code_occurence!() }),
            },
        }
    }
    fn create_query_bind(self, query: sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments> {
        match self.0 {
            Some(value) => value.create_query_bind(query),
            None => query.bind(sqlx::types::Json(None::<std::option::Option<<DoggieAsNotNullJsonbObject as postgresql_crud::PostgresqlJsonType>::Create>>)),
        }
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionDoggieAsNullableJsonbObjectSelect(std::option::Option<<DoggieAsNotNullJsonbObject as postgresql_crud::PostgresqlJsonType>::Select>);
impl OptionDoggieAsNullableJsonbObjectSelect {
    pub fn new(value: std::option::Option<postgresql_crud::NotEmptyUniqueEnumVec<DoggieAsNotNullJsonbObjectSelectElement>>) -> Self {
        Self(match value {
            Some(value) => Some(<DoggieAsNotNullJsonbObject as postgresql_crud::PostgresqlJsonType>::Select::new(value)),
            None => None,
        })
    }
}
impl OptionDoggieAsNullableJsonbObjectSelect {
    fn select_query_part_postgresql_type(&self, column: &std::primitive::str) -> std::string::String {
        format!("case when jsonb_typeof({column}) = 'null' then 'null'::jsonb else ({}) end", {
            let value = match &self.0 {
                Some(value) => value,
                None => &<<DoggieAsNotNullJsonbObject as postgresql_crud::PostgresqlJsonType>::Select as postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement>::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            };
            value.select_query_part_postgresql_type(column)
        })
    }
}
impl sqlx::Type<sqlx::Postgres> for OptionDoggieAsNullableJsonbObjectSelect {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <sqlx::types::Json<Self> as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> std::primitive::bool {
        <sqlx::types::Json<Self> as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
impl sqlx::Decode<'_, sqlx::Postgres> for OptionDoggieAsNullableJsonbObjectSelect {
    fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        match <sqlx::types::Json<Self> as sqlx::Decode<sqlx::Postgres>>::decode(value) {
            Ok(value) => Ok(value.0),
            Err(error) => Err(error),
        }
    }
}
impl postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for OptionDoggieAsNullableJsonbObjectSelect {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self(Some(postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()))
    }
}
pub type OptionDoggieAsNullableJsonbObjectWhereElement = postgresql_crud::NullableJsonObjectPostgresqlTypeWhereFilter<<DoggieAsNotNullJsonbObject as postgresql_crud::PostgresqlJsonType>::WhereElement>;
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionDoggieAsNullableJsonbObjectRead(std::option::Option<<DoggieAsNotNullJsonbObject as postgresql_crud::PostgresqlJsonType>::Read>);
impl OptionDoggieAsNullableJsonbObjectRead {
    fn into_inner(self) -> OptionDoggieAsNullableJsonbObjectReadInner {
        match self.0 {
            Some(value) => Some(<DoggieAsNotNullJsonbObject as postgresql_crud::PostgresqlJsonType>::into_inner(value)),
            None => None,
        }
    }
}
impl OptionDoggieAsNullableJsonbObjectRead {
    pub fn new(value: std::option::Option<<DoggieAsNotNullJsonbObject as postgresql_crud::PostgresqlJsonType>::Read>) -> Self {
        Self(value)
    }
}
impl postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for OptionDoggieAsNullableJsonbObjectRead {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self(Some(postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()))
    }
}
impl sqlx::Type<sqlx::Postgres> for OptionDoggieAsNullableJsonbObjectRead {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <sqlx::types::Json<Self> as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> std::primitive::bool {
        <sqlx::types::Json<Self> as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
impl sqlx::Decode<'_, sqlx::Postgres> for OptionDoggieAsNullableJsonbObjectRead {
    fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        match <sqlx::types::Json<Self> as sqlx::Decode<sqlx::Postgres>>::decode(value) {
            Ok(value) => Ok(value.0),
            Err(error) => Err(error),
        }
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct OptionDoggieAsNullableJsonbObjectReadOnlyIds(postgresql_crud::Value<std::option::Option<DoggieAsNotNullJsonbObjectReadOnlyIds>>);
impl sqlx::Decode<'_, sqlx::Postgres> for OptionDoggieAsNullableJsonbObjectReadOnlyIds {
    fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        match <sqlx::types::Json<Self> as sqlx::Decode<sqlx::Postgres>>::decode(value) {
            Ok(value) => Ok(value.0),
            Err(error) => Err(error),
        }
    }
}
impl sqlx::Type<sqlx::Postgres> for OptionDoggieAsNullableJsonbObjectReadOnlyIds {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <sqlx::types::Json<Self> as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> std::primitive::bool {
        <sqlx::types::Json<Self> as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
pub type OptionDoggieAsNullableJsonbObjectReadInner = std::option::Option<<DoggieAsNotNullJsonbObject as postgresql_crud::PostgresqlJsonType>::ReadInner>;
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionDoggieAsNullableJsonbObjectUpdate(std::option::Option<<DoggieAsNotNullJsonbObject as postgresql_crud::PostgresqlJsonType>::Update>);
impl OptionDoggieAsNullableJsonbObjectUpdate {
    pub fn new(value: std::option::Option<<DoggieAsNotNullJsonbObject as postgresql_crud::PostgresqlJsonType>::Update>) -> Self {
        Self(value)
    }
}
impl postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for OptionDoggieAsNullableJsonbObjectUpdate {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self(Some(postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()))
    }
}
impl postgresql_crud::PostgresqlJsonType for OptionDoggieAsNullableJsonbObject {
    type TableTypeDeclaration = OptionDoggieAsNullableJsonbObjectTableTypeDeclaration;
    type Create = OptionDoggieAsNullableJsonbObjectCreate;
    fn create_query_part(value: &Self::Create, increment: &mut std::primitive::u64) -> Result<std::string::String, postgresql_crud::QueryPartErrorNamed> {
        value.create_query_part(increment)
    }
    fn create_query_bind(value: Self::Create, query: sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments> {
        value.create_query_bind(query)
    }
    type Select = OptionDoggieAsNullableJsonbObjectSelect;
    fn select_query_part(value: &Self::Select, field_ident: &std::primitive::str, column_name_and_maybe_field_getter: &std::primitive::str, column_name_and_maybe_field_getter_for_error_message: &std::primitive::str, is_postgresql_type: std::primitive::bool) -> std::string::String {
        let column_name_and_maybe_field_getter_field_ident = format!("{column_name_and_maybe_field_getter}->'{field_ident}'");
        format!("jsonb_build_object('{field_ident}',jsonb_build_object('value',case when jsonb_typeof({column_name_and_maybe_field_getter_field_ident}) = 'null' then 'null'::jsonb else ({}) end))", {
            let value = match &value.0 {
                Some(value) => value,
                None => &<<DoggieAsNotNullJsonbObject as postgresql_crud::PostgresqlJsonType>::Select as postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement>::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            };
            <DoggieAsNotNullJsonbObject as postgresql_crud::PostgresqlJsonType>::select_query_part(value, field_ident, &column_name_and_maybe_field_getter_field_ident, column_name_and_maybe_field_getter_for_error_message, true)
        })
    }
    type WhereElement = OptionDoggieAsNullableJsonbObjectWhereElement;
    type Read = OptionDoggieAsNullableJsonbObjectRead;
    type ReadOnlyIds = OptionDoggieAsNullableJsonbObjectReadOnlyIds;
    fn select_only_ids_query_part(column_name_and_maybe_field_getter: &std::primitive::str) -> std::string::String {
        format!("jsonb_build_object('value',case when jsonb_typeof({column_name_and_maybe_field_getter})='null' then 'null'::jsonb else {} end)", <DoggieAsNotNullJsonbObject as postgresql_crud::PostgresqlJsonType>::select_only_ids_query_part(column_name_and_maybe_field_getter),)
    }
    type ReadInner = OptionDoggieAsNullableJsonbObjectReadInner;
    fn into_inner(value: Self::Read) -> Self::ReadInner {
        value.into_inner()
    }
    type Update = OptionDoggieAsNullableJsonbObjectUpdate;
    fn update_query_part(value: &Self::Update, jsonb_set_accumulator: &std::primitive::str, jsonb_set_target: &std::primitive::str, jsonb_set_path: &std::primitive::str, increment: &mut std::primitive::u64) -> Result<std::string::String, postgresql_crud::QueryPartErrorNamed> {
        match &value.0 {
            Some(value) => <DoggieAsNotNullJsonbObject as postgresql_crud::PostgresqlJsonType>::update_query_part(value, jsonb_set_accumulator, jsonb_set_target, jsonb_set_path, increment),
            None => match increment.checked_add(1) {
                Some(value) => {
                    *increment = value;
                    Ok(format!("jsonb_set({jsonb_set_accumulator},'{{{jsonb_set_path}}}',${increment})"))
                }
                None => Err(postgresql_crud::QueryPartErrorNamed::CheckedAdd { code_occurence: error_occurence_lib::code_occurence!() }),
            },
        }
    }
    fn update_query_bind(value: Self::Update, query: sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments> {
        match value.0 {
            Some(value) => <DoggieAsNotNullJsonbObject as postgresql_crud::PostgresqlJsonType>::update_query_bind(value, query),
            None => query.bind(sqlx::types::Json(<OptionDoggieAsNullableJsonbObject as postgresql_crud::PostgresqlJsonType>::Update::new(None))),
        }
    }
    fn select_only_updated_ids_query_part(value: &Self::Update, field_ident: &std::primitive::str, column_name_and_maybe_field_getter: &std::primitive::str, increment: &mut std::primitive::u64) -> Result<std::string::String, postgresql_crud::QueryPartErrorNamed> {
        println!("@@@7");
        match &value.0 {
            Some(value) => {
                let mut acc = std::string::String::default();
                for element in value.0.to_vec() {
                    match &element {
                        DoggieAsNotNullJsonbObjectUpdateElement::Field807(value) => match <VecOfCatWithIdAsNotNullArrayOfNotNullJsonbObjectWithId as postgresql_crud::PostgresqlJsonType>::select_only_updated_ids_query_part(&value.value, &"field_807", &format!("{column_name_and_maybe_field_getter}->'{field_ident}'"), increment) {
                            Ok(value) => {
                                acc.push_str(&value);
                            }
                            Err(error) => {
                                return Err(error);
                            }
                        },
                    }
                }
                let _ = acc.pop();
                Ok(format!("'{field_ident}',jsonb_build_object('value',jsonb_build_object('value',jsonb_build_object({acc}))),"))
            }
            None => Ok(format!("'{field_ident}',jsonb_build_object('value','null'::jsonb),")),
        }
    }
}
impl postgresql_crud::PostgresqlType for OptionDoggieAsNullableJsonbObject {
    type TableTypeDeclaration = OptionDoggieAsNullableJsonbObjectTableTypeDeclaration;
    type Create = OptionDoggieAsNullableJsonbObjectCreate;
    fn create_query_part(value: &Self::Create, increment: &mut std::primitive::u64) -> Result<std::string::String, postgresql_crud::QueryPartErrorNamed> {
        value.create_query_part(increment)
    }
    fn create_query_bind(value: Self::Create, query: sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments> {
        value.create_query_bind(query)
    }
    type Select = OptionDoggieAsNullableJsonbObjectSelect;
    fn select_query_part(value: &Self::Select, column: &std::primitive::str) -> std::string::String {
        format!("{} as {column}", value.select_query_part_postgresql_type(column))
    }
    type WhereElement = OptionDoggieAsNullableJsonbObjectWhereElement;
    type Read = OptionDoggieAsNullableJsonbObjectRead;
    fn normalize(value: Self::Read) -> Self::Read {
        value
    }
    type ReadOnlyIds = OptionDoggieAsNullableJsonbObjectReadOnlyIds;
    fn select_only_ids_query_part(column: &std::primitive::str, is_primary_key: std::primitive::bool) -> std::string::String {
        format!("{} as {column},", <OptionDoggieAsNullableJsonbObject as postgresql_crud::PostgresqlJsonType>::select_only_ids_query_part(&column))
    }
    type ReadInner = OptionDoggieAsNullableJsonbObjectReadInner;
    fn into_inner(value: Self::Read) -> Self::ReadInner {
        value.into_inner()
    }
    type Update = OptionDoggieAsNullableJsonbObjectUpdate;
    fn update_query_part(value: &Self::Update, jsonb_set_accumulator: &std::primitive::str, jsonb_set_target: &std::primitive::str, jsonb_set_path: &std::primitive::str, increment: &mut std::primitive::u64) -> Result<std::string::String, postgresql_crud::QueryPartErrorNamed> {
        match &value.0 {
            Some(value) => <DoggieAsNotNullJsonbObject as postgresql_crud::PostgresqlType>::update_query_part(value, jsonb_set_accumulator, jsonb_set_target, jsonb_set_path, increment),
            None => match increment.checked_add(1) {
                Some(value) => {
                    *increment = value;
                    Ok(format!("${increment}"))
                }
                None => Err(postgresql_crud::QueryPartErrorNamed::CheckedAdd { code_occurence: error_occurence_lib::code_occurence!() }),
            },
        }
    }
    fn update_query_bind<'a>(value: Self::Update, query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
        match value.0 {
            Some(value) => <DoggieAsNotNullJsonbObject as postgresql_crud::PostgresqlType>::update_query_bind(value, query),
            None => query.bind(sqlx::types::Json(<OptionDoggieAsNullableJsonbObject as postgresql_crud::PostgresqlJsonType>::Update::new(None))),
        }
    }
    fn select_only_updated_ids_query_part(value: &Self::Update, column: &std::primitive::str, increment: &mut std::primitive::u64, is_primary_key: std::primitive::bool) -> Result<std::string::String, postgresql_crud::QueryPartErrorNamed> {
        match &value.0 {
            Some(value) => {
                let mut acc = std::string::String::default();
                for element in value.0.to_vec() {
                    match &element {
                        DoggieAsNotNullJsonbObjectUpdateElement::Field807(value) => match <VecOfCatWithIdAsNotNullArrayOfNotNullJsonbObjectWithId as postgresql_crud::PostgresqlJsonType>::select_only_updated_ids_query_part(&value.value, &"field_807", &format!("{column}"), increment) {
                            Ok(value) => {
                                acc.push_str(&value);
                            }
                            Err(error) => {
                                return Err(error);
                            }
                        },
                    }
                }
                let _ = acc.pop();
                Ok(format!("jsonb_build_object('value',jsonb_build_object('value',jsonb_build_object({acc}))) as {column},"))
            }
            None => Ok(format!("jsonb_build_object('value','null'::jsonb) as {column},")),
        }
    }
}
#[cfg(test)]
impl postgresql_crud::tests::PostgresqlTypeTestCases for OptionDoggieAsNullableJsonbObject {
    type Element = Self;
    fn test_cases(read_only_ids: &<Self::Element as postgresql_crud::PostgresqlType>::ReadOnlyIds) -> std::vec::Vec<std::vec::Vec<<Self::Element as postgresql_crud::PostgresqlType>::ReadInner>> {
        let mut acc = vec![];
        if let Some(value) = &read_only_ids.0.value {
            for element in <DoggieAsNotNullJsonbObject as postgresql_crud::tests::PostgresqlJsonTypeTestCases>::test_cases(&value) {
                for current_element in element {
                    acc.push(Some(current_element));
                }
            }
        }
        acc.push(None);
        vec![acc]
    }
    fn read_new_or_try_new_unwraped_for_test(value: OptionDoggieAsNullableJsonbObjectReadInner) -> <Self::Element as postgresql_crud::PostgresqlType>::Read {
        <Self::Element as postgresql_crud::PostgresqlType>::Read::new(match value {
            Some(value) => Some(<DoggieAsNotNullJsonbObject as postgresql_crud::tests::PostgresqlJsonTypeTestCases>::read_new_or_try_new_unwraped_for_test(value)),
            None => None,
        })
    }
    fn update_new_or_try_new_unwraped_for_test(value: OptionDoggieAsNullableJsonbObjectReadInner) -> <Self::Element as postgresql_crud::PostgresqlType>::Update {
        <Self::Element as postgresql_crud::PostgresqlType>::Update::new(match value {
            Some(value) => Some(<DoggieAsNotNullJsonbObject as postgresql_crud::tests::PostgresqlTypeTestCases>::update_new_or_try_new_unwraped_for_test(value)),
            None => None,
        })
    }
    fn read_only_ids_to_option_value_read_inner(value: <Self::Element as postgresql_crud::PostgresqlType>::ReadOnlyIds) -> std::option::Option<postgresql_crud::Value<<Self::Element as postgresql_crud::PostgresqlType>::ReadInner>> {
        todo!()
    }
    fn update_to_read_only_ids(value: &<Self::Element as postgresql_crud::PostgresqlType>::Update) -> <Self::Element as postgresql_crud::PostgresqlType>::ReadOnlyIds {
        OptionDoggieAsNullableJsonbObjectReadOnlyIds(postgresql_crud::Value {
            value: match &value.0 {
                Some(value) => Some(<DoggieAsNotNullJsonbObject as postgresql_crud::tests::PostgresqlJsonTypeTestCases>::update_to_read_only_ids(&value)),
                None => None,
            },
        })
    }
}
#[cfg(test)]
impl postgresql_crud::tests::PostgresqlJsonTypeTestCases for OptionDoggieAsNullableJsonbObject {
    type Element = Self;
    fn test_cases(read_only_ids: &<Self::Element as postgresql_crud::PostgresqlJsonType>::ReadOnlyIds) -> std::vec::Vec<std::vec::Vec<<Self::Element as postgresql_crud::PostgresqlJsonType>::ReadInner>> {
        let mut acc = vec![];
        if let Some(value) = &read_only_ids.0.value {
            for element in <DoggieAsNotNullJsonbObject as postgresql_crud::tests::PostgresqlJsonTypeTestCases>::test_cases(&value) {
                for current_element in element {
                    acc.push(Some(current_element));
                }
            }
        }
        acc.push(None);
        vec![acc]
    }
    fn read_new_or_try_new_unwraped_for_test(value: OptionDoggieAsNullableJsonbObjectReadInner) -> <Self::Element as postgresql_crud::PostgresqlJsonType>::Read {
        <Self::Element as postgresql_crud::PostgresqlType>::Read::new(match value {
            Some(value) => Some(<DoggieAsNotNullJsonbObject as postgresql_crud::tests::PostgresqlJsonTypeTestCases>::read_new_or_try_new_unwraped_for_test(value)),
            None => None,
        })
    }
    fn update_new_or_try_new_unwraped_for_test(value: OptionDoggieAsNullableJsonbObjectReadInner) -> <Self::Element as postgresql_crud::PostgresqlJsonType>::Update {
        <Self::Element as postgresql_crud::PostgresqlType>::Update::new(match value {
            Some(value) => Some(<DoggieAsNotNullJsonbObject as postgresql_crud::tests::PostgresqlTypeTestCases>::update_new_or_try_new_unwraped_for_test(value)),
            None => None,
        })
    }
    fn read_only_ids_to_option_value_read_inner(value: <Self::Element as postgresql_crud::PostgresqlJsonType>::ReadOnlyIds) -> std::option::Option<postgresql_crud::Value<<Self::Element as postgresql_crud::PostgresqlJsonType>::ReadInner>> {
        Some(postgresql_crud::Value {
            value: match value.0.value {
                Some(value) => match <DoggieAsNotNullJsonbObject as postgresql_crud::tests::PostgresqlJsonTypeTestCases>::read_only_ids_to_option_value_read_inner(value) {
                    Some(value) => Some(value.value),
                    None => None,
                },
                None => None,
            },
        })
    }
    fn update_to_read_only_ids(value: &<Self::Element as postgresql_crud::PostgresqlJsonType>::Update) -> <Self::Element as postgresql_crud::PostgresqlJsonType>::ReadOnlyIds {
        OptionDoggieAsNullableJsonbObjectReadOnlyIds(postgresql_crud::Value {
            value: match &value.0 {
                Some(value) => Some(<DoggieAsNotNullJsonbObject as postgresql_crud::tests::PostgresqlJsonTypeTestCases>::update_to_read_only_ids(&value)),
                None => None,
            },
        })
    }
}

///////////
#[derive(Debug)]
pub struct CatAsNotNullJsonbObject;
#[derive(Debug)]
pub struct CatWithIdAsNotNullJsonbObjectWithId;
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct CatAsNotNullJsonbObjectTableTypeDeclaration {
    field_444: <postgresql_crud::postgresql_json_type::StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud::PostgresqlJsonType>::TableTypeDeclaration,
}
impl CatAsNotNullJsonbObjectTableTypeDeclaration {
    pub fn new(field_444: <postgresql_crud::postgresql_json_type::StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud::PostgresqlJsonType>::TableTypeDeclaration) -> Self {
        Self { field_444 }
    }
}
impl postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for CatAsNotNullJsonbObjectTableTypeDeclaration {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self {
            field_444: postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct CatWithIdAsNotNullJsonbObjectWithIdTableTypeDeclaration {
    id: <postgresql_crud::postgresql_json_type::UuidUuidAsNotNullJsonbString as postgresql_crud::PostgresqlJsonType>::TableTypeDeclaration,
    field_444: <postgresql_crud::postgresql_json_type::StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud::PostgresqlJsonType>::TableTypeDeclaration,
}
impl CatWithIdAsNotNullJsonbObjectWithIdTableTypeDeclaration {
    pub fn new(id: <postgresql_crud::postgresql_json_type::UuidUuidAsNotNullJsonbString as postgresql_crud::PostgresqlJsonType>::TableTypeDeclaration, field_444: <postgresql_crud::postgresql_json_type::StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud::PostgresqlJsonType>::TableTypeDeclaration) -> Self {
        Self { id, field_444 }
    }
}
impl postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for CatWithIdAsNotNullJsonbObjectWithIdTableTypeDeclaration {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self {
            id: postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            field_444: postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
        }
    }
}
impl CatAsNotNullJsonbObjectTableTypeDeclaration {
    pub fn create_table_column_query_part(column: &dyn std::fmt::Display, _: std::primitive::bool) -> impl std::fmt::Display {
        format!("{column} jsonb not null check (jsonb_matches_schema('{}', {column}))", serde_json::to_string(&schemars::schema_for!(CatAsNotNullJsonbObjectTableTypeDeclaration)).unwrap())
    }
}
impl CatWithIdAsNotNullJsonbObjectWithIdTableTypeDeclaration {
    pub fn create_table_column_query_part(column: &dyn std::fmt::Display, _: std::primitive::bool) -> impl std::fmt::Display {
        format!("{column} jsonb not null check (jsonb_matches_schema('{}', {column}))", serde_json::to_string(&schemars::schema_for!(CatWithIdAsNotNullJsonbObjectWithIdTableTypeDeclaration)).unwrap())
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct CatAsNotNullJsonbObjectCreate {
    field_444: <postgresql_crud::postgresql_json_type::StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud::PostgresqlJsonType>::Create,
}
impl CatAsNotNullJsonbObjectCreate {
    pub fn new(field_444: <postgresql_crud::postgresql_json_type::StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud::PostgresqlJsonType>::Create) -> Self {
        Self { field_444 }
    }
}
impl postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for CatAsNotNullJsonbObjectCreate {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self {
            field_444: postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct CatWithIdAsNotNullJsonbObjectWithIdCreate {
    field_444: <postgresql_crud::postgresql_json_type::StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud::PostgresqlJsonType>::Create,
}
impl CatWithIdAsNotNullJsonbObjectWithIdCreate {
    pub fn new(field_444: <postgresql_crud::postgresql_json_type::StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud::PostgresqlJsonType>::Create) -> Self {
        Self { field_444 }
    }
}
impl postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for CatWithIdAsNotNullJsonbObjectWithIdCreate {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self {
            field_444: postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
        }
    }
}
impl std::fmt::Display for CatAsNotNullJsonbObjectCreate {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{:?}", self)
    }
}
impl error_occurence_lib::ToStdStringString for CatAsNotNullJsonbObjectCreate {
    fn to_std_string_string(&self) -> std::string::String {
        format!("{self:?}")
    }
}
impl CatAsNotNullJsonbObjectCreate {
    fn create_query_part(&self, increment: &mut std::primitive::u64) -> Result<std::string::String, postgresql_crud::QueryPartErrorNamed> {
        let mut increments = std::string::String::from("");
        match <postgresql_crud::postgresql_json_type::StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud::PostgresqlJsonType>::create_query_part(&self.field_444, increment) {
            Ok(value) => {
                increments.push_str(&postgresql_crud::wrap_into_jsonb_build_object("field_444", &value));
            }
            Err(error) => {
                return Err(error);
            }
        }
        let _ = increments.pop();
        let _ = increments.pop();
        Ok(format!("{increments}"))
    }
    fn create_query_bind(self, mut query: sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments> {
        query = <postgresql_crud::postgresql_json_type::StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud::PostgresqlJsonType>::create_query_bind(self.field_444, query);
        query
    }
}
impl std::fmt::Display for CatWithIdAsNotNullJsonbObjectWithIdCreate {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{:?}", self)
    }
}
impl error_occurence_lib::ToStdStringString for CatWithIdAsNotNullJsonbObjectWithIdCreate {
    fn to_std_string_string(&self) -> std::string::String {
        format!("{self:?}")
    }
}
impl CatWithIdAsNotNullJsonbObjectWithIdCreate {
    fn create_query_part(&self, increment: &mut std::primitive::u64) -> Result<std::string::String, postgresql_crud::QueryPartErrorNamed> {
        let mut increments = std::string::String::from("");
        match <postgresql_crud::postgresql_json_type::StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud::PostgresqlJsonType>::create_query_part(&self.field_444, increment) {
            Ok(value) => {
                increments.push_str(&postgresql_crud::wrap_into_jsonb_build_object("field_444", &value));
            }
            Err(error) => {
                return Err(error);
            }
        }
        let _ = increments.pop();
        let _ = increments.pop();
        Ok(format!("jsonb_build_object('id', to_jsonb(gen_random_uuid()))||{increments}"))
    }
    fn create_query_bind(self, mut query: sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments> {
        query = <postgresql_crud::postgresql_json_type::StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud::PostgresqlJsonType>::create_query_bind(self.field_444, query);
        query
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct CatAsNotNullJsonbObjectSelect(postgresql_crud::NotEmptyUniqueEnumVec<CatAsNotNullJsonbObjectSelectElement>);
impl CatAsNotNullJsonbObjectSelect {
    pub fn new(value: postgresql_crud::NotEmptyUniqueEnumVec<CatAsNotNullJsonbObjectSelectElement>) -> Self {
        Self(value)
    }
}
impl CatAsNotNullJsonbObjectSelect {
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
                    CatAsNotNullJsonbObjectSelectElement::Field444(value) => <postgresql_crud::postgresql_json_type::StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud::PostgresqlJsonType>::select_query_part(&value, "field_444", &column_name_and_maybe_field_getter_field_ident, &column_name_and_maybe_field_getter_for_error_message_field_ident, false,),
                }
            ));
        }
        let _ = acc.pop();
        let _ = acc.pop();
        if is_postgresql_type { format!("{acc}") } else { format!("jsonb_build_object('{field_ident}', jsonb_build_object('value',{acc}))") }
    }
}
impl sqlx::Type<sqlx::Postgres> for CatAsNotNullJsonbObjectSelect {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <sqlx::types::Json<Self> as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> std::primitive::bool {
        <sqlx::types::Json<Self> as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
impl sqlx::Decode<'_, sqlx::Postgres> for CatAsNotNullJsonbObjectSelect {
    fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        match <sqlx::types::Json<Self> as sqlx::Decode<sqlx::Postgres>>::decode(value) {
            Ok(value) => Ok(value.0),
            Err(error) => Err(error),
        }
    }
}
impl postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for CatAsNotNullJsonbObjectSelect {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self(postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element())
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub enum CatAsNotNullJsonbObjectSelectElement {
    #[serde(rename(serialize = "field_444", deserialize = "field_444"))]
    Field444(<postgresql_crud::postgresql_json_type::StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud::PostgresqlJsonType>::Select),
}
impl error_occurence_lib::ToStdStringString for CatAsNotNullJsonbObjectSelectElement {
    fn to_std_string_string(&self) -> std::string::String {
        format!("{self:?}")
    }
}
impl postgresql_crud::AllEnumVariantsArrayDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for CatAsNotNullJsonbObjectSelectElement {
    fn all_enum_variants_array_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> std::vec::Vec<Self> {
        vec![CatAsNotNullJsonbObjectSelectElement::Field444(postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element())]
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct CatWithIdAsNotNullJsonbObjectWithIdSelect(postgresql_crud::NotEmptyUniqueEnumVec<CatWithIdAsNotNullJsonbObjectWithIdSelectElement>);
impl CatWithIdAsNotNullJsonbObjectWithIdSelect {
    pub fn new(value: postgresql_crud::NotEmptyUniqueEnumVec<CatWithIdAsNotNullJsonbObjectWithIdSelectElement>) -> Self {
        Self(value)
    }
}
impl sqlx::Type<sqlx::Postgres> for CatWithIdAsNotNullJsonbObjectWithIdSelect {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <sqlx::types::Json<Self> as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> std::primitive::bool {
        <sqlx::types::Json<Self> as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
impl sqlx::Decode<'_, sqlx::Postgres> for CatWithIdAsNotNullJsonbObjectWithIdSelect {
    fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        match <sqlx::types::Json<Self> as sqlx::Decode<sqlx::Postgres>>::decode(value) {
            Ok(value) => Ok(value.0),
            Err(error) => Err(error),
        }
    }
}
impl postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for CatWithIdAsNotNullJsonbObjectWithIdSelect {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self(postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element())
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub enum CatWithIdAsNotNullJsonbObjectWithIdSelectElement {
    #[serde(rename(serialize = "id", deserialize = "id"))]
    Id(<postgresql_crud::postgresql_json_type::UuidUuidAsNotNullJsonbString as postgresql_crud::PostgresqlJsonType>::Select),
    #[serde(rename(serialize = "field_444", deserialize = "field_444"))]
    Field444(<postgresql_crud::postgresql_json_type::StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud::PostgresqlJsonType>::Select),
}
impl error_occurence_lib::ToStdStringString for CatWithIdAsNotNullJsonbObjectWithIdSelectElement {
    fn to_std_string_string(&self) -> std::string::String {
        format!("{self:?}")
    }
}
impl postgresql_crud::AllEnumVariantsArrayDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for CatWithIdAsNotNullJsonbObjectWithIdSelectElement {
    fn all_enum_variants_array_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> std::vec::Vec<Self> {
        vec![
            CatWithIdAsNotNullJsonbObjectWithIdSelectElement::Id(postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            CatWithIdAsNotNullJsonbObjectWithIdSelectElement::Field444(postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
        ]
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub enum CatAsNotNullJsonbObjectWhereElement {
    Field444(postgresql_crud::PostgresqlTypeWhere<<postgresql_crud::postgresql_json_type::StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud::PostgresqlJsonType>::WhereElement>),
}
impl<'a> postgresql_crud::PostgresqlTypeWhereFilter<'a> for CatAsNotNullJsonbObjectWhereElement {
    fn query_part(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, postgresql_crud::QueryPartErrorNamed> {
        match &self {
            Self::Field444(value) => postgresql_crud::PostgresqlTypeWhereFilter::query_part(value, increment, &format!("{column}->'field_444'"), is_need_to_add_logical_operator),
        }
    }
    fn query_bind(self, query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
        match self {
            Self::Field444(value) => postgresql_crud::PostgresqlTypeWhereFilter::query_bind(value, query),
        }
    }
}
impl error_occurence_lib::ToStdStringString for CatAsNotNullJsonbObjectWhereElement {
    fn to_std_string_string(&self) -> std::string::String {
        format!("{self:?}")
    }
}
impl postgresql_crud::AllEnumVariantsArrayDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for CatAsNotNullJsonbObjectWhereElement {
    fn all_enum_variants_array_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> std::vec::Vec<Self> {
        vec![Self::Field444(postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element())]
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub enum CatWithIdAsNotNullJsonbObjectWithIdWhereElement {
    Id(postgresql_crud::PostgresqlTypeWhere<<postgresql_crud::postgresql_json_type::UuidUuidAsNotNullJsonbString as postgresql_crud::PostgresqlJsonType>::WhereElement>),
    Field444(postgresql_crud::PostgresqlTypeWhere<<postgresql_crud::postgresql_json_type::StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud::PostgresqlJsonType>::WhereElement>),
}
impl<'a> postgresql_crud::PostgresqlTypeWhereFilter<'a> for CatWithIdAsNotNullJsonbObjectWithIdWhereElement {
    fn query_part(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, postgresql_crud::QueryPartErrorNamed> {
        match &self {
            Self::Id(value) => postgresql_crud::PostgresqlTypeWhereFilter::query_part(value, increment, &format!("{column}->'id'"), is_need_to_add_logical_operator),
            Self::Field444(value) => postgresql_crud::PostgresqlTypeWhereFilter::query_part(value, increment, &format!("{column}->'field_444'"), is_need_to_add_logical_operator),
        }
    }
    fn query_bind(self, query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
        match self {
            Self::Id(value) => postgresql_crud::PostgresqlTypeWhereFilter::query_bind(value, query),
            Self::Field444(value) => postgresql_crud::PostgresqlTypeWhereFilter::query_bind(value, query),
        }
    }
}
impl error_occurence_lib::ToStdStringString for CatWithIdAsNotNullJsonbObjectWithIdWhereElement {
    fn to_std_string_string(&self) -> std::string::String {
        format!("{self:?}")
    }
}
impl postgresql_crud::AllEnumVariantsArrayDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for CatWithIdAsNotNullJsonbObjectWithIdWhereElement {
    fn all_enum_variants_array_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> std::vec::Vec<Self> {
        vec![
            Self::Id(postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            Self::Field444(postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
        ]
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct CatAsNotNullJsonbObjectRead {
    #[serde(skip_serializing_if = "Option::is_none")]
    field_444: std::option::Option<postgresql_crud::Value<<postgresql_crud::postgresql_json_type::StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud::PostgresqlJsonType>::Read>>,
}
impl CatAsNotNullJsonbObjectRead {
    fn into_inner(self) -> CatAsNotNullJsonbObjectReadInner {
        CatAsNotNullJsonbObjectReadInner {
            field_444: match self.field_444 {
                Some(value) => Some(postgresql_crud::Value {
                    value: <postgresql_crud::postgresql_json_type::StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud::PostgresqlJsonType>::into_inner(value.value),
                }),
                None => None,
            },
        }
    }
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum CatAsNotNullJsonbObjectReadTryFromErrorNamed {
    AllFieldsAreNone { code_occurence: error_occurence_lib::code_occurence::CodeOccurence },
}
impl CatAsNotNullJsonbObjectRead {
    pub fn try_new(field_444: std::option::Option<postgresql_crud::Value<<postgresql_crud::postgresql_json_type::StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud::PostgresqlJsonType>::Read>>) -> Result<Self, CatAsNotNullJsonbObjectReadTryFromErrorNamed> {
        if let None = &field_444 {
            return Err(CatAsNotNullJsonbObjectReadTryFromErrorNamed::AllFieldsAreNone { code_occurence: error_occurence_lib::code_occurence!() });
        }
        Ok(Self { field_444 })
    }
}
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for CatAsNotNullJsonbObjectRead {
        fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            #[doc(hidden)]
            enum __Field {
                __field0,
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
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
                fn visit_str<__E>(self, __value: &str) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        "field_444" => serde::__private::Ok(__Field::__field0),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
                fn visit_bytes<__E>(self, __value: &[u8]) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        b"field_444" => serde::__private::Ok(__Field::__field0),
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
                marker: _serde::__private::PhantomData<CatAsNotNullJsonbObjectRead>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            #[automatically_derived]
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = CatAsNotNullJsonbObjectRead;
                fn expecting(&self, __formatter: &mut _serde::__private::Formatter<'_>) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(__formatter, "struct CatAsNotNullJsonbObjectRead")
                }
                #[inline]
                fn visit_seq<__A>(self, mut __seq: __A) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::SeqAccess<'de>,
                {
                    let __field0 = match serde::de::SeqAccess::next_element::<std::option::Option<postgresql_crud::Value<<postgresql_crud::postgresql_json_type::StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud::PostgresqlJsonType>::Read>>>(&mut __seq)? {
                        serde::__private::Some(__value) => __value,
                        serde::__private::None => {
                            return serde::__private::Err(serde::de::Error::invalid_length(0usize, &"struct CatAsNotNullJsonbObjectRead with 1 elements"));
                        }
                    };
                    match CatAsNotNullJsonbObjectRead::try_new(__field0) {
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
                    let mut __field0: serde::__private::Option<std::option::Option<postgresql_crud::Value<<postgresql_crud::postgresql_json_type::StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud::PostgresqlJsonType>::Read>>> = serde::__private::None;
                    while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<__Field>(&mut __map)? {
                        match __key {
                            __Field::__field0 => {
                                if serde::__private::Option::is_some(&__field0) {
                                    return serde::__private::Err(<__A::Error as serde::de::Error>::duplicate_field("field_444"));
                                }
                                __field0 = serde::__private::Some(serde::de::MapAccess::next_value::<std::option::Option<postgresql_crud::Value<<postgresql_crud::postgresql_json_type::StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud::PostgresqlJsonType>::Read>>>(&mut __map)?);
                            }
                            _ => {
                                let _ = _serde::de::MapAccess::next_value::<_serde::de::IgnoredAny>(&mut __map)?;
                            }
                        }
                    }
                    let __field0 = match __field0 {
                        serde::__private::Some(__field0) => __field0,
                        serde::__private::None => serde::__private::de::missing_field("field_444")?,
                    };
                    match CatAsNotNullJsonbObjectRead::try_new(__field0) {
                        Ok(value) => serde::__private::Ok(value),
                        Err(error) => {
                            return Err(serde::de::Error::custom(format!("{error:?}")));
                        }
                    }
                }
            }
            #[doc(hidden)]
            const FIELDS: &'static [&'static str] = &["field_444"];
            _serde::Deserializer::deserialize_struct(
                __deserializer,
                "CatAsNotNullJsonbObjectRead",
                FIELDS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<CatAsNotNullJsonbObjectRead>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
impl postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for CatAsNotNullJsonbObjectRead {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self {
            field_444: Some(postgresql_crud::Value {
                value: postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            }),
        }
    }
}
impl sqlx::Type<sqlx::Postgres> for CatAsNotNullJsonbObjectRead {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <sqlx::types::Json<Self> as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> std::primitive::bool {
        <sqlx::types::Json<Self> as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
impl sqlx::Decode<'_, sqlx::Postgres> for CatAsNotNullJsonbObjectRead {
    fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        match <sqlx::types::Json<Self> as sqlx::Decode<sqlx::Postgres>>::decode(value) {
            Ok(value) => Ok(value.0),
            Err(error) => Err(error),
        }
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct CatWithIdAsNotNullJsonbObjectWithIdRead {
    #[serde(skip_serializing_if = "Option::is_none")]
    id: std::option::Option<postgresql_crud::Value<<postgresql_crud::postgresql_json_type::UuidUuidAsNotNullJsonbString as postgresql_crud::PostgresqlJsonType>::Read>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    field_444: std::option::Option<postgresql_crud::Value<<postgresql_crud::postgresql_json_type::StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud::PostgresqlJsonType>::Read>>,
}
impl CatWithIdAsNotNullJsonbObjectWithIdRead {
    fn into_inner(self) -> CatWithIdAsNotNullJsonbObjectWithIdReadInner {
        CatWithIdAsNotNullJsonbObjectWithIdReadInner {
            id: match self.id {
                Some(value) => Some(postgresql_crud::Value {
                    value: <postgresql_crud::postgresql_json_type::UuidUuidAsNotNullJsonbString as postgresql_crud::PostgresqlJsonType>::into_inner(value.value),
                }),
                None => None,
            },
            field_444: match self.field_444 {
                Some(value) => Some(postgresql_crud::Value {
                    value: <postgresql_crud::postgresql_json_type::StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud::PostgresqlJsonType>::into_inner(value.value),
                }),
                None => None,
            },
        }
    }
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum CatWithIdAsNotNullJsonbObjectWithIdReadTryFromErrorNamed {
    AllFieldsAreNone { code_occurence: error_occurence_lib::code_occurence::CodeOccurence },
}
impl CatWithIdAsNotNullJsonbObjectWithIdRead {
    pub fn try_new(id: std::option::Option<postgresql_crud::Value<<postgresql_crud::postgresql_json_type::UuidUuidAsNotNullJsonbString as postgresql_crud::PostgresqlJsonType>::Read>>, field_444: std::option::Option<postgresql_crud::Value<<postgresql_crud::postgresql_json_type::StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud::PostgresqlJsonType>::Read>>) -> Result<Self, CatWithIdAsNotNullJsonbObjectWithIdReadTryFromErrorNamed> {
        if let (None, None) = (&id, &field_444) {
            return Err(CatWithIdAsNotNullJsonbObjectWithIdReadTryFromErrorNamed::AllFieldsAreNone { code_occurence: error_occurence_lib::code_occurence!() });
        }
        Ok(Self { id, field_444 })
    }
}
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for CatWithIdAsNotNullJsonbObjectWithIdRead {
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
                        "id" => serde::__private::Ok(__Field::__field0),
                        "field_444" => serde::__private::Ok(__Field::__field1),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
                fn visit_bytes<__E>(self, __value: &[u8]) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        b"id" => serde::__private::Ok(__Field::__field0),
                        b"field_444" => serde::__private::Ok(__Field::__field1),
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
                marker: _serde::__private::PhantomData<CatWithIdAsNotNullJsonbObjectWithIdRead>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            #[automatically_derived]
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = CatWithIdAsNotNullJsonbObjectWithIdRead;
                fn expecting(&self, __formatter: &mut _serde::__private::Formatter<'_>) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(__formatter, "struct CatWithIdAsNotNullJsonbObjectWithIdRead")
                }
                #[inline]
                fn visit_seq<__A>(self, mut __seq: __A) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::SeqAccess<'de>,
                {
                    let __field0 = match serde::de::SeqAccess::next_element::<std::option::Option<postgresql_crud::Value<<postgresql_crud::postgresql_json_type::UuidUuidAsNotNullJsonbString as postgresql_crud::PostgresqlJsonType>::Read>>>(&mut __seq)? {
                        serde::__private::Some(__value) => __value,
                        serde::__private::None => {
                            return serde::__private::Err(serde::de::Error::invalid_length(0usize, &"struct CatWithIdAsNotNullJsonbObjectWithIdRead with 2 elements"));
                        }
                    };
                    let __field1 = match serde::de::SeqAccess::next_element::<std::option::Option<postgresql_crud::Value<<postgresql_crud::postgresql_json_type::StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud::PostgresqlJsonType>::Read>>>(&mut __seq)? {
                        serde::__private::Some(__value) => __value,
                        serde::__private::None => {
                            return serde::__private::Err(serde::de::Error::invalid_length(0usize, &"struct CatWithIdAsNotNullJsonbObjectWithIdRead with 2 elements"));
                        }
                    };
                    match CatWithIdAsNotNullJsonbObjectWithIdRead::try_new(__field0, __field1) {
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
                    let mut __field0: serde::__private::Option<std::option::Option<postgresql_crud::Value<<postgresql_crud::postgresql_json_type::UuidUuidAsNotNullJsonbString as postgresql_crud::PostgresqlJsonType>::Read>>> = serde::__private::None;
                    let mut __field1: serde::__private::Option<std::option::Option<postgresql_crud::Value<<postgresql_crud::postgresql_json_type::StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud::PostgresqlJsonType>::Read>>> = serde::__private::None;
                    while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<__Field>(&mut __map)? {
                        match __key {
                            __Field::__field0 => {
                                if serde::__private::Option::is_some(&__field0) {
                                    return serde::__private::Err(<__A::Error as serde::de::Error>::duplicate_field("id"));
                                }
                                __field0 = serde::__private::Some(serde::de::MapAccess::next_value::<std::option::Option<postgresql_crud::Value<<postgresql_crud::postgresql_json_type::UuidUuidAsNotNullJsonbString as postgresql_crud::PostgresqlJsonType>::Read>>>(&mut __map)?);
                            }
                            __Field::__field1 => {
                                if serde::__private::Option::is_some(&__field1) {
                                    return serde::__private::Err(<__A::Error as serde::de::Error>::duplicate_field("field_444"));
                                }
                                __field1 = serde::__private::Some(serde::de::MapAccess::next_value::<std::option::Option<postgresql_crud::Value<<postgresql_crud::postgresql_json_type::StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud::PostgresqlJsonType>::Read>>>(&mut __map)?);
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
                        serde::__private::None => serde::__private::de::missing_field("field_444")?,
                    };
                    match CatWithIdAsNotNullJsonbObjectWithIdRead::try_new(__field0, __field1) {
                        Ok(value) => serde::__private::Ok(value),
                        Err(error) => {
                            return Err(serde::de::Error::custom(format!("{error:?}")));
                        }
                    }
                }
            }
            #[doc(hidden)]
            const FIELDS: &'static [&'static str] = &["id", "field_444"];
            _serde::Deserializer::deserialize_struct(
                __deserializer,
                "CatWithIdAsNotNullJsonbObjectWithIdRead",
                FIELDS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<CatWithIdAsNotNullJsonbObjectWithIdRead>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
impl postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for CatWithIdAsNotNullJsonbObjectWithIdRead {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self {
            id: Some(postgresql_crud::Value {
                value: postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            }),
            field_444: Some(postgresql_crud::Value {
                value: postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            }),
        }
    }
}
impl sqlx::Type<sqlx::Postgres> for CatWithIdAsNotNullJsonbObjectWithIdRead {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <sqlx::types::Json<Self> as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> std::primitive::bool {
        <sqlx::types::Json<Self> as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
impl sqlx::Decode<'_, sqlx::Postgres> for CatWithIdAsNotNullJsonbObjectWithIdRead {
    fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        match <sqlx::types::Json<Self> as sqlx::Decode<sqlx::Postgres>>::decode(value) {
            Ok(value) => Ok(value.0),
            Err(error) => Err(error),
        }
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct CatAsNotNullJsonbObjectReadOnlyIdsHandle {
    field_444: <postgresql_crud::postgresql_json_type::StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud::PostgresqlJsonType>::ReadOnlyIds,
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct CatAsNotNullJsonbObjectReadOnlyIds(postgresql_crud::Value<CatAsNotNullJsonbObjectReadOnlyIdsHandle>);
impl sqlx::Decode<'_, sqlx::Postgres> for CatAsNotNullJsonbObjectReadOnlyIds {
    fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        match <sqlx::types::Json<Self> as sqlx::Decode<sqlx::Postgres>>::decode(value) {
            Ok(value) => Ok(value.0),
            Err(error) => Err(error),
        }
    }
}
impl sqlx::Type<sqlx::Postgres> for CatAsNotNullJsonbObjectReadOnlyIds {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <sqlx::types::Json<Self> as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> std::primitive::bool {
        <sqlx::types::Json<Self> as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct CatWithIdAsNotNullJsonbObjectWithIdReadOnlyIdsHandle {
    id: <postgresql_crud::postgresql_json_type::UuidUuidAsNotNullJsonbString as postgresql_crud::PostgresqlJsonType>::ReadOnlyIds,
    field_444: <postgresql_crud::postgresql_json_type::StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud::PostgresqlJsonType>::ReadOnlyIds,
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct CatWithIdAsNotNullJsonbObjectWithIdReadOnlyIds(pub postgresql_crud::Value<CatWithIdAsNotNullJsonbObjectWithIdReadOnlyIdsHandle>);
impl sqlx::Decode<'_, sqlx::Postgres> for CatWithIdAsNotNullJsonbObjectWithIdReadOnlyIds {
    fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        match <sqlx::types::Json<Self> as sqlx::Decode<sqlx::Postgres>>::decode(value) {
            Ok(value) => Ok(value.0),
            Err(error) => Err(error),
        }
    }
}
impl sqlx::Type<sqlx::Postgres> for CatWithIdAsNotNullJsonbObjectWithIdReadOnlyIds {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <sqlx::types::Json<Self> as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> std::primitive::bool {
        <sqlx::types::Json<Self> as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct CatAsNotNullJsonbObjectReadInner {
    field_444: std::option::Option<postgresql_crud::Value<<postgresql_crud::postgresql_json_type::StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud::PostgresqlJsonType>::ReadInner>>,
}
#[derive(Debug, Clone, PartialEq)]
pub struct CatWithIdAsNotNullJsonbObjectWithIdReadInner {
    id: std::option::Option<postgresql_crud::Value<<postgresql_crud::postgresql_json_type::UuidUuidAsNotNullJsonbString as postgresql_crud::PostgresqlJsonType>::ReadInner>>,
    field_444: std::option::Option<postgresql_crud::Value<<postgresql_crud::postgresql_json_type::StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud::PostgresqlJsonType>::ReadInner>>,
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct CatAsNotNullJsonbObjectUpdate(postgresql_crud::NotEmptyUniqueEnumVec<CatAsNotNullJsonbObjectUpdateElement>);
impl CatAsNotNullJsonbObjectUpdate {
    pub fn new(value: postgresql_crud::NotEmptyUniqueEnumVec<CatAsNotNullJsonbObjectUpdateElement>) -> Self {
        Self(value)
    }
}
impl postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for CatAsNotNullJsonbObjectUpdate {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self(postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element())
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub enum CatAsNotNullJsonbObjectUpdateElement {
    #[serde(rename(serialize = "field_444", deserialize = "field_444"))]
    Field444(postgresql_crud::Value<<postgresql_crud::postgresql_json_type::StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud::PostgresqlJsonType>::Update>),
}
impl postgresql_crud::AllEnumVariantsArrayDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for CatAsNotNullJsonbObjectUpdateElement {
    fn all_enum_variants_array_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> std::vec::Vec<Self> {
        vec![CatAsNotNullJsonbObjectUpdateElement::Field444(postgresql_crud::Value {
            value: postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
        })]
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct CatWithIdAsNotNullJsonbObjectWithIdUpdateElement {
    id: <postgresql_crud::postgresql_json_type::UuidUuidAsNotNullJsonbString as postgresql_crud::PostgresqlJsonType>::Update,
    fields: <CatAsNotNullJsonbObject as postgresql_crud::PostgresqlJsonType>::Update,
}
impl CatWithIdAsNotNullJsonbObjectWithIdUpdateElement {
    pub fn new(id: <postgresql_crud::postgresql_json_type::UuidUuidAsNotNullJsonbString as postgresql_crud::PostgresqlJsonType>::Update, fields: <CatAsNotNullJsonbObject as postgresql_crud::PostgresqlJsonType>::Update) -> Self {
        Self { id, fields }
    }
}
impl postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for CatWithIdAsNotNullJsonbObjectWithIdUpdateElement {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self {
            id: postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            fields: postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct CatWithIdAsNotNullJsonbObjectWithIdUpdateHandle(postgresql_crud::UniqueVec<CatWithIdAsNotNullJsonbObjectWithIdUpdateElement>);
impl postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for CatWithIdAsNotNullJsonbObjectWithIdUpdateHandle {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self(postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element())
    }
}
impl CatWithIdAsNotNullJsonbObjectWithIdUpdateHandle {
    pub fn new(value: postgresql_crud::UniqueVec<CatWithIdAsNotNullJsonbObjectWithIdUpdateElement>) -> Self {
        Self(value)
    }
}
impl std::default::Default for CatWithIdAsNotNullJsonbObjectWithIdUpdateHandle {
    fn default() -> Self {
        Self(postgresql_crud::UniqueVec::default())
    }
}
impl postgresql_crud::PostgresqlJsonType for CatAsNotNullJsonbObject {
    type TableTypeDeclaration = CatAsNotNullJsonbObjectTableTypeDeclaration;
    type Create = CatAsNotNullJsonbObjectCreate;
    fn create_query_part(value: &Self::Create, increment: &mut std::primitive::u64) -> Result<std::string::String, postgresql_crud::QueryPartErrorNamed> {
        value.create_query_part(increment)
    }
    fn create_query_bind(value: Self::Create, query: sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments> {
        value.create_query_bind(query)
    }
    type Select = CatAsNotNullJsonbObjectSelect;
    fn select_query_part(value: &Self::Select, field_ident: &std::primitive::str, column_name_and_maybe_field_getter: &std::primitive::str, column_name_and_maybe_field_getter_for_error_message: &std::primitive::str, is_postgresql_type: std::primitive::bool) -> std::string::String {
        let mut acc = std::string::String::default();
        let column_name_and_maybe_field_getter_field_ident = if is_postgresql_type { column_name_and_maybe_field_getter.to_string() } else { format!("{column_name_and_maybe_field_getter}->'{field_ident}'") };
        let column_name_and_maybe_field_getter_for_error_message_field_ident = format!("{column_name_and_maybe_field_getter_for_error_message}.{field_ident}");
        for element in value.0.to_vec() {
            acc.push_str(&format!(
                "{}||",
                match element {
                    CatAsNotNullJsonbObjectSelectElement::Field444(value) => <postgresql_crud::postgresql_json_type::StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud::PostgresqlJsonType>::select_query_part(&value, "field_444", &column_name_and_maybe_field_getter_field_ident, &column_name_and_maybe_field_getter_for_error_message_field_ident, false,),
                }
            ));
        }
        let _ = acc.pop();
        let _ = acc.pop();
        if is_postgresql_type { format!("{acc}") } else { format!("jsonb_build_object('{field_ident}', jsonb_build_object('value',{acc}))") }
    }
    type WhereElement = CatAsNotNullJsonbObjectWhereElement;
    type Read = CatAsNotNullJsonbObjectRead;
    type ReadOnlyIds = CatAsNotNullJsonbObjectReadOnlyIds;
    fn select_only_ids_query_part(column_name_and_maybe_field_getter: &std::primitive::str) -> std::string::String {
        format!("jsonb_build_object('value',jsonb_build_object('field_444',{}))", <postgresql_crud::postgresql_json_type::StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud::PostgresqlJsonType>::select_only_ids_query_part(&format!("{column_name_and_maybe_field_getter}->'field_444'")))
    }
    type ReadInner = CatAsNotNullJsonbObjectReadInner;
    fn into_inner(value: Self::Read) -> Self::ReadInner {
        value.into_inner()
    }
    type Update = CatAsNotNullJsonbObjectUpdate;
    fn update_query_part(value: &Self::Update, jsonb_set_accumulator: &std::primitive::str, jsonb_set_target: &std::primitive::str, jsonb_set_path: &std::primitive::str, increment: &mut std::primitive::u64) -> Result<std::string::String, postgresql_crud::QueryPartErrorNamed> {
        let mut std_option_option_object_acc = format!("case when jsonb_typeof({jsonb_set_target}) = 'object' then ({jsonb_set_target})::jsonb else '{{}}'::jsonb end");
        let generate_jsonb_set_target = |value: &std::primitive::str| format!("{jsonb_set_target}->'{value}'");
        for element in value.0.to_vec() {
            match element {
                CatAsNotNullJsonbObjectUpdateElement::Field444(value) => match <postgresql_crud::postgresql_json_type::StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud::PostgresqlJsonType>::update_query_part(&value.value, &std_option_option_object_acc, &generate_jsonb_set_target("field_444"), "field_444", increment) {
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
    fn update_query_bind(value: Self::Update, mut query: sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments> {
        for element in value.0.into_vec() {
            match element {
                CatAsNotNullJsonbObjectUpdateElement::Field444(value) => {
                    query = <postgresql_crud::postgresql_json_type::StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud::PostgresqlJsonType>::update_query_bind(value.value, query);
                }
            }
        }
        query
    }
    fn select_only_updated_ids_query_part(value: &Self::Update, field_ident: &std::primitive::str, column_name_and_maybe_field_getter: &std::primitive::str, increment: &mut std::primitive::u64) -> Result<std::string::String, postgresql_crud::QueryPartErrorNamed> {
        let mut acc = std::string::String::default();
        for element in value.0.to_vec() {
            match &element {
                CatAsNotNullJsonbObjectUpdateElement::Field444(value) => match <postgresql_crud::postgresql_json_type::StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud::PostgresqlJsonType>::select_only_updated_ids_query_part(&value.value, &"field_444", &format!("{column_name_and_maybe_field_getter}->'{field_ident}'"), increment) {
                    Ok(value) => {
                        acc.push_str(&value);
                    }
                    Err(error) => {
                        return Err(error);
                    }
                },
            }
        }
        let _ = acc.pop();
        Ok(format!("'{field_ident}',jsonb_build_object('value',jsonb_build_object({acc})),"))
    }
}
impl postgresql_crud::PostgresqlType for CatAsNotNullJsonbObject {
    type TableTypeDeclaration = CatAsNotNullJsonbObjectTableTypeDeclaration;
    type Create = CatAsNotNullJsonbObjectCreate;
    fn create_query_part(value: &Self::Create, increment: &mut std::primitive::u64) -> Result<std::string::String, postgresql_crud::QueryPartErrorNamed> {
        value.create_query_part(increment)
    }
    fn create_query_bind(value: Self::Create, query: sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments> {
        value.create_query_bind(query)
    }
    type Select = CatAsNotNullJsonbObjectSelect;
    fn select_query_part(value: &Self::Select, column: &std::primitive::str) -> std::string::String {
        format!("{} as {column}", value.select_query_part_postgresql_type(column))
    }
    type WhereElement = CatAsNotNullJsonbObjectWhereElement;
    type Read = CatAsNotNullJsonbObjectRead;
    fn normalize(value: Self::Read) -> Self::Read {
        value
    }
    type ReadOnlyIds = CatAsNotNullJsonbObjectReadOnlyIds;
    fn select_only_ids_query_part(column: &std::primitive::str, is_primary_key: std::primitive::bool) -> std::string::String {
        format!("{} as {column},", <CatAsNotNullJsonbObject as postgresql_crud::PostgresqlJsonType>::select_only_ids_query_part(&column))
    }
    type ReadInner = CatAsNotNullJsonbObjectReadInner;
    fn into_inner(value: Self::Read) -> Self::ReadInner {
        value.into_inner()
    }
    type Update = CatAsNotNullJsonbObjectUpdate;
    fn update_query_part(value: &Self::Update, jsonb_set_accumulator: &std::primitive::str, jsonb_set_target: &std::primitive::str, jsonb_set_path: &std::primitive::str, increment: &mut std::primitive::u64) -> Result<std::string::String, postgresql_crud::QueryPartErrorNamed> {
        let mut std_option_option_object_acc = format!("case when jsonb_typeof({jsonb_set_target}) = 'object' then ({jsonb_set_target})::jsonb else '{{}}'::jsonb end");
        let generate_jsonb_set_target = |value: &std::primitive::str| format!("{jsonb_set_target}->'{value}'");
        for element in value.0.to_vec() {
            match element {
                CatAsNotNullJsonbObjectUpdateElement::Field444(value) => match <postgresql_crud::postgresql_json_type::StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud::PostgresqlJsonType>::update_query_part(&value.value, &std_option_option_object_acc, &generate_jsonb_set_target("field_444"), "field_444", increment) {
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
    fn update_query_bind<'a>(value: Self::Update, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
        for element in value.0.into_vec() {
            match element {
                CatAsNotNullJsonbObjectUpdateElement::Field444(value) => {
                    query = <postgresql_crud::postgresql_json_type::StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud::PostgresqlJsonType>::update_query_bind(value.value, query);
                }
            }
        }
        query
    }
    fn select_only_updated_ids_query_part(value: &Self::Update, column: &std::primitive::str, increment: &mut std::primitive::u64, is_primary_key: std::primitive::bool) -> Result<std::string::String, postgresql_crud::QueryPartErrorNamed> {
        let mut acc = std::string::String::default();
        for element in value.0.to_vec() {
            match &element {
                CatAsNotNullJsonbObjectUpdateElement::Field444(value) => match <postgresql_crud::postgresql_json_type::StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud::PostgresqlJsonType>::select_only_updated_ids_query_part(&value.value, &"field_444", &column, increment) {
                    Ok(value) => {
                        acc.push_str(&value);
                    }
                    Err(error) => {
                        return Err(error);
                    }
                },
            }
        }
        let _ = acc.pop();
        Ok(format!("jsonb_build_object('value',jsonb_build_object({acc})) as {column},"))
    }
}
impl postgresql_crud::PostgresqlJsonType for CatWithIdAsNotNullJsonbObjectWithId {
    type TableTypeDeclaration = CatWithIdAsNotNullJsonbObjectWithIdTableTypeDeclaration;
    type Create = CatWithIdAsNotNullJsonbObjectWithIdCreate;
    fn create_query_part(value: &Self::Create, increment: &mut std::primitive::u64) -> Result<std::string::String, postgresql_crud::QueryPartErrorNamed> {
        value.create_query_part(increment)
    }
    fn create_query_bind(value: Self::Create, query: sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments> {
        value.create_query_bind(query)
    }
    type Select = CatWithIdAsNotNullJsonbObjectWithIdSelect;
    fn select_query_part(value: &Self::Select, field_ident: &std::primitive::str, column_name_and_maybe_field_getter: &std::primitive::str, column_name_and_maybe_field_getter_for_error_message: &std::primitive::str, is_postgresql_type: std::primitive::bool) -> std::string::String {
        let mut acc = std::string::String::default();
        let column_name_and_maybe_field_getter_field_ident = if is_postgresql_type { column_name_and_maybe_field_getter.to_string() } else { format!("{column_name_and_maybe_field_getter}->'{field_ident}'") };
        let column_name_and_maybe_field_getter_for_error_message_field_ident = format!("{column_name_and_maybe_field_getter_for_error_message}.{field_ident}");
        for element in value.0.to_vec() {
            acc.push_str(&format!(
                "{}||",
                match element {
                    CatWithIdAsNotNullJsonbObjectWithIdSelectElement::Id(value) => <postgresql_crud::postgresql_json_type::UuidUuidAsNotNullJsonbString as postgresql_crud::PostgresqlJsonType>::select_query_part(&value, "id", &column_name_and_maybe_field_getter_field_ident, &column_name_and_maybe_field_getter_for_error_message_field_ident, false,),
                    CatWithIdAsNotNullJsonbObjectWithIdSelectElement::Field444(value) => <postgresql_crud::postgresql_json_type::StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud::PostgresqlJsonType>::select_query_part(&value, "field_444", &column_name_and_maybe_field_getter_field_ident, &column_name_and_maybe_field_getter_for_error_message_field_ident, false,),
                }
            ));
        }
        let _ = acc.pop();
        let _ = acc.pop();
        if is_postgresql_type { format!("{acc}") } else { format!("jsonb_build_object('{field_ident}', jsonb_build_object('value',{acc}))") }
    }
    type WhereElement = CatWithIdAsNotNullJsonbObjectWithIdWhereElement;
    type Read = CatWithIdAsNotNullJsonbObjectWithIdRead;
    type ReadOnlyIds = CatWithIdAsNotNullJsonbObjectWithIdReadOnlyIds;
    fn select_only_ids_query_part(column_name_and_maybe_field_getter: &std::primitive::str) -> std::string::String {
        format!(
            "jsonb_build_object('value',jsonb_build_object('id',{},'field_444',{}))",
            <postgresql_crud::postgresql_json_type::UuidUuidAsNotNullJsonbString as postgresql_crud::PostgresqlJsonType>::select_only_ids_query_part(&format!("{column_name_and_maybe_field_getter}->'id'")),
            <postgresql_crud::postgresql_json_type::StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud::PostgresqlJsonType>::select_only_ids_query_part(&format!("{column_name_and_maybe_field_getter}->'field_444'"))
        )
    }
    type ReadInner = CatWithIdAsNotNullJsonbObjectWithIdReadInner;
    fn into_inner(value: Self::Read) -> Self::ReadInner {
        value.into_inner()
    }
    type Update = CatWithIdAsNotNullJsonbObjectWithIdUpdateHandle;
    fn update_query_part(value: &Self::Update, jsonb_set_accumulator: &std::primitive::str, jsonb_set_target: &std::primitive::str, jsonb_set_path: &std::primitive::str, increment: &mut std::primitive::u64) -> Result<std::string::String, postgresql_crud::QueryPartErrorNamed> {
        todo!()
    }
    fn update_query_bind(value: Self::Update, query: sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments> {
        todo!()
    }
    fn select_only_updated_ids_query_part(value: &Self::Update, field_ident: &std::primitive::str, column_name_and_maybe_field_getter: &std::primitive::str, increment: &mut std::primitive::u64) -> Result<std::string::String, postgresql_crud::QueryPartErrorNamed> {
        println!("@@@11");
        let mut acc = std::string::String::new();
        for element in value.0.to_vec() {
            acc.push_str(&format!("jsonb_build_object('id',jsonb_build_object('value','{}'),{})||", element.id.get_inner(), {
                let mut acc = std::string::String::new();
                for element in element.fields.0.to_vec() {
                    match &element {
                        CatAsNotNullJsonbObjectUpdateElement::Field444(value) => match <postgresql_crud::postgresql_json_type::StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud::PostgresqlJsonType>::select_only_updated_ids_query_part(&value.value, &"field_444", &column_name_and_maybe_field_getter, increment) {
                            Ok(value) => {
                                acc.push_str(&value);
                            }
                            Err(error) => {
                                return Err(error);
                            }
                        },
                    }
                }
                let _ = acc.pop();
                acc
            }));
        }
        let _ = acc.pop();
        let _ = acc.pop();
        Ok(format!("jsonb_build_object('value',{acc})"))
    }
}
#[cfg(test)]
impl postgresql_crud::tests::PostgresqlTypeTestCases for CatAsNotNullJsonbObject {
    type Element = Self;
    fn test_cases(read_only_ids: &<Self::Element as postgresql_crud::PostgresqlType>::ReadOnlyIds) -> std::vec::Vec<std::vec::Vec<<Self::Element as postgresql_crud::PostgresqlType>::ReadInner>> {
        let mut acc = vec![];
        for element in <postgresql_crud::postgresql_json_type::StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud::tests::PostgresqlJsonTypeTestCases>::test_cases(&read_only_ids.0.value.field_444) {
            for current_element in element {
                acc.push(CatAsNotNullJsonbObjectReadInner { field_444: Some(postgresql_crud::Value { value: current_element }) });
            }
        }
        vec![acc]
    }
    fn read_new_or_try_new_unwraped_for_test(value: CatAsNotNullJsonbObjectReadInner) -> <Self::Element as postgresql_crud::PostgresqlType>::Read {
        <Self::Element as postgresql_crud::PostgresqlType>::Read::try_new(match value.field_444 {
            Some(value) => Some(postgresql_crud::Value {
                value: <postgresql_crud::postgresql_json_type::StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud::tests::PostgresqlJsonTypeTestCases>::read_new_or_try_new_unwraped_for_test(value.value),
            }),
            None => None,
        })
        .unwrap()
    }
    fn update_new_or_try_new_unwraped_for_test(value: CatAsNotNullJsonbObjectReadInner) -> <Self::Element as postgresql_crud::PostgresqlType>::Update {
        <Self::Element as postgresql_crud::PostgresqlType>::Update::new(
            postgresql_crud::NotEmptyUniqueEnumVec::try_new({
                let mut acc = vec![];
                if let Some(value) = value.field_444 {
                    acc.push(CatAsNotNullJsonbObjectUpdateElement::Field444(postgresql_crud::Value {
                        value: <postgresql_crud::postgresql_json_type::StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud::tests::PostgresqlJsonTypeTestCases>::update_new_or_try_new_unwraped_for_test(value.value),
                    }));
                }
                acc
            })
            .unwrap(),
        )
    }
    fn read_only_ids_to_option_value_read_inner(value: <Self::Element as postgresql_crud::PostgresqlType>::ReadOnlyIds) -> std::option::Option<postgresql_crud::Value<<Self::Element as postgresql_crud::PostgresqlType>::ReadInner>> {
        todo!()
    }
    fn update_to_read_only_ids(value: &<Self::Element as postgresql_crud::PostgresqlType>::Update) -> <Self::Element as postgresql_crud::PostgresqlType>::ReadOnlyIds {
        let mut field_444 = None;
        for element in value.0.to_vec() {
            match element {
                CatAsNotNullJsonbObjectUpdateElement::Field444(value) => {
                    field_444 = Some(<postgresql_crud::postgresql_json_type::StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud::tests::PostgresqlJsonTypeTestCases>::update_to_read_only_ids(&value.value));
                }
            }
        }
        CatAsNotNullJsonbObjectReadOnlyIds(postgresql_crud::Value {
            value: CatAsNotNullJsonbObjectReadOnlyIdsHandle { field_444: field_444.expect("expect 106f16f2-ae03-48b3-9239-f4b1b60746d5") },
        })
    }
}
#[cfg(test)]
impl postgresql_crud::tests::PostgresqlJsonTypeTestCases for CatAsNotNullJsonbObject {
    type Element = Self;
    fn test_cases(read_only_ids: &<Self::Element as postgresql_crud::PostgresqlJsonType>::ReadOnlyIds) -> std::vec::Vec<std::vec::Vec<<Self::Element as postgresql_crud::PostgresqlJsonType>::ReadInner>> {
        let mut acc = vec![];
        for element in <postgresql_crud::postgresql_json_type::StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud::tests::PostgresqlJsonTypeTestCases>::test_cases(&read_only_ids.0.value.field_444) {
            for current_element in element {
                acc.push(CatAsNotNullJsonbObjectReadInner { field_444: Some(postgresql_crud::Value { value: current_element }) });
            }
        }
        vec![acc]
    }
    fn read_new_or_try_new_unwraped_for_test(value: CatAsNotNullJsonbObjectReadInner) -> <Self::Element as postgresql_crud::PostgresqlJsonType>::Read {
        <Self::Element as postgresql_crud::PostgresqlType>::Read::try_new(match value.field_444 {
            Some(value) => Some(postgresql_crud::Value {
                value: <postgresql_crud::postgresql_json_type::StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud::tests::PostgresqlJsonTypeTestCases>::read_new_or_try_new_unwraped_for_test(value.value),
            }),
            None => None,
        })
        .unwrap()
    }
    fn update_new_or_try_new_unwraped_for_test(value: CatAsNotNullJsonbObjectReadInner) -> <Self::Element as postgresql_crud::PostgresqlJsonType>::Update {
        <Self::Element as postgresql_crud::PostgresqlType>::Update::new(
            postgresql_crud::NotEmptyUniqueEnumVec::try_new({
                let mut acc = vec![];
                if let Some(value) = value.field_444 {
                    acc.push(CatAsNotNullJsonbObjectUpdateElement::Field444(postgresql_crud::Value {
                        value: <postgresql_crud::postgresql_json_type::StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud::tests::PostgresqlJsonTypeTestCases>::update_new_or_try_new_unwraped_for_test(value.value),
                    }));
                }
                acc
            })
            .unwrap(),
        )
    }
    fn read_only_ids_to_option_value_read_inner(value: <Self::Element as postgresql_crud::PostgresqlJsonType>::ReadOnlyIds) -> std::option::Option<postgresql_crud::Value<<Self::Element as postgresql_crud::PostgresqlJsonType>::ReadInner>> {
        Some(postgresql_crud::Value {
            value: CatAsNotNullJsonbObjectReadInner {
                field_444: match <postgresql_crud::postgresql_json_type::StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud::tests::PostgresqlJsonTypeTestCases>::read_only_ids_to_option_value_read_inner(value.0.value.field_444) {
                    Some(value) => Some(value),
                    None => Some(postgresql_crud::Value {
                        value: <postgresql_crud::postgresql_json_type::StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud::PostgresqlJsonType>::into_inner(<<postgresql_crud::postgresql_json_type::StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud::PostgresqlJsonType>::Read as postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement>::default_but_option_is_always_some_and_vec_always_contains_one_element()),
                    }),
                },
            },
        })
    }
    fn update_to_read_only_ids(value: &<Self::Element as postgresql_crud::PostgresqlJsonType>::Update) -> <Self::Element as postgresql_crud::PostgresqlJsonType>::ReadOnlyIds {
        let mut field_444 = None;
        for element in value.0.to_vec() {
            match element {
                CatAsNotNullJsonbObjectUpdateElement::Field444(value) => {
                    field_444 = Some(<postgresql_crud::postgresql_json_type::StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud::tests::PostgresqlJsonTypeTestCases>::update_to_read_only_ids(&value.value));
                }
            }
        }
        CatAsNotNullJsonbObjectReadOnlyIds(postgresql_crud::Value {
            value: CatAsNotNullJsonbObjectReadOnlyIdsHandle { field_444: field_444.expect("expect 106f16f2-ae03-48b3-9239-f4b1b60746d5") },
        })
    }
}
#[cfg(test)]
impl postgresql_crud::tests::PostgresqlJsonTypeTestCases for CatWithIdAsNotNullJsonbObjectWithId {
    type Element = Self;
    fn test_cases(read_only_ids: &<Self::Element as postgresql_crud::PostgresqlJsonType>::ReadOnlyIds) -> std::vec::Vec<std::vec::Vec<<Self::Element as postgresql_crud::PostgresqlJsonType>::ReadInner>> {
        vec![vec![CatWithIdAsNotNullJsonbObjectWithIdReadInner {
            id: Some(postgresql_crud::Value { value: read_only_ids.0.value.id.0.value.clone() }),
            field_444: <postgresql_crud::postgresql_json_type::StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud::tests::PostgresqlJsonTypeTestCases>::read_only_ids_to_option_value_read_inner(read_only_ids.0.value.field_444.clone()),
        }]]
    }
    fn read_new_or_try_new_unwraped_for_test(value: CatWithIdAsNotNullJsonbObjectWithIdReadInner) -> <Self::Element as postgresql_crud::PostgresqlJsonType>::Read {
        CatWithIdAsNotNullJsonbObjectWithIdRead {
            id: match value.id {
                Some(value) => Some(postgresql_crud::Value {
                    value: <postgresql_crud::postgresql_json_type::UuidUuidAsNotNullJsonbString as postgresql_crud::tests::PostgresqlJsonTypeTestCases>::read_new_or_try_new_unwraped_for_test(value.value),
                }),
                None => None,
            },
            field_444: match value.field_444 {
                Some(value) => Some(postgresql_crud::Value {
                    value: <postgresql_crud::postgresql_json_type::StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud::tests::PostgresqlJsonTypeTestCases>::read_new_or_try_new_unwraped_for_test(value.value),
                }),
                None => None,
            },
        }
    }
    fn update_new_or_try_new_unwraped_for_test(value: CatWithIdAsNotNullJsonbObjectWithIdReadInner) -> <Self::Element as postgresql_crud::PostgresqlJsonType>::Update {
        todo!()
    }
    fn read_only_ids_to_option_value_read_inner(value: <Self::Element as postgresql_crud::PostgresqlJsonType>::ReadOnlyIds) -> std::option::Option<postgresql_crud::Value<<Self::Element as postgresql_crud::PostgresqlJsonType>::ReadInner>> {
        Some(postgresql_crud::Value {
            value: CatWithIdAsNotNullJsonbObjectWithIdReadInner {
                id: match <postgresql_crud::postgresql_json_type::UuidUuidAsNotNullJsonbString as postgresql_crud::tests::PostgresqlJsonTypeTestCases>::read_only_ids_to_option_value_read_inner(value.0.value.id) {
                    Some(value) => Some(value),
                    None => Some(postgresql_crud::Value {
                        value: <postgresql_crud::postgresql_json_type::UuidUuidAsNotNullJsonbString as postgresql_crud::PostgresqlJsonType>::into_inner(<<postgresql_crud::postgresql_json_type::UuidUuidAsNotNullJsonbString as postgresql_crud::PostgresqlJsonType>::Read as postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement>::default_but_option_is_always_some_and_vec_always_contains_one_element()),
                    }),
                },
                field_444: match <postgresql_crud::postgresql_json_type::StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud::tests::PostgresqlJsonTypeTestCases>::read_only_ids_to_option_value_read_inner(value.0.value.field_444) {
                    Some(value) => Some(value),
                    None => Some(postgresql_crud::Value {
                        value: <postgresql_crud::postgresql_json_type::StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud::PostgresqlJsonType>::into_inner(<<postgresql_crud::postgresql_json_type::StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud::PostgresqlJsonType>::Read as postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement>::default_but_option_is_always_some_and_vec_always_contains_one_element()),
                    }),
                },
            },
        })
    }
    fn update_to_read_only_ids(value: &<Self::Element as postgresql_crud::PostgresqlJsonType>::Update) -> <Self::Element as postgresql_crud::PostgresqlJsonType>::ReadOnlyIds {
        todo!()
    }
}
#[derive(Debug)]
pub struct OptionCatAsNullableJsonbObject;
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionCatAsNullableJsonbObjectTableTypeDeclaration(std::option::Option<<CatAsNotNullJsonbObject as postgresql_crud::PostgresqlJsonType>::TableTypeDeclaration>);
impl OptionCatAsNullableJsonbObjectTableTypeDeclaration {
    pub fn new(value: std::option::Option<<CatAsNotNullJsonbObject as postgresql_crud::PostgresqlJsonType>::TableTypeDeclaration>) -> Self {
        Self(value)
    }
}
impl postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for OptionCatAsNullableJsonbObjectTableTypeDeclaration {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self(Some(postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()))
    }
}
impl OptionCatAsNullableJsonbObjectTableTypeDeclaration {
    pub fn create_table_column_query_part(column: &dyn std::fmt::Display, _: std::primitive::bool) -> impl std::fmt::Display {
        format!("{column} jsonb not null check (jsonb_matches_schema('{}', {column}))", serde_json::to_string(&schemars::schema_for!(OptionCatAsNullableJsonbObjectTableTypeDeclaration)).unwrap())
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionCatAsNullableJsonbObjectCreate(std::option::Option<<CatAsNotNullJsonbObject as postgresql_crud::PostgresqlJsonType>::Create>);
impl OptionCatAsNullableJsonbObjectCreate {
    pub fn new(value: std::option::Option<<CatAsNotNullJsonbObject as postgresql_crud::PostgresqlJsonType>::Create>) -> Self {
        Self(value)
    }
}
impl postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for OptionCatAsNullableJsonbObjectCreate {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self(Some(postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()))
    }
}
impl std::fmt::Display for OptionCatAsNullableJsonbObjectCreate {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{:?}", self)
    }
}
impl error_occurence_lib::ToStdStringString for OptionCatAsNullableJsonbObjectCreate {
    fn to_std_string_string(&self) -> std::string::String {
        format!("{self:?}")
    }
}
impl OptionCatAsNullableJsonbObjectCreate {
    fn create_query_part(&self, increment: &mut std::primitive::u64) -> Result<std::string::String, postgresql_crud::QueryPartErrorNamed> {
        match &self.0 {
            Some(value) => <CatAsNotNullJsonbObject as postgresql_crud::PostgresqlJsonType>::create_query_part(value, increment),
            None => match increment.checked_add(1) {
                Some(value) => {
                    *increment = value;
                    Ok(format!("${increment}"))
                }
                None => Err(postgresql_crud::QueryPartErrorNamed::CheckedAdd { code_occurence: error_occurence_lib::code_occurence!() }),
            },
        }
    }
    fn create_query_bind(self, query: sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments> {
        match self.0 {
            Some(value) => value.create_query_bind(query),
            None => query.bind(sqlx::types::Json(None::<std::option::Option<<CatAsNotNullJsonbObject as postgresql_crud::PostgresqlJsonType>::Create>>)),
        }
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionCatAsNullableJsonbObjectSelect(std::option::Option<<CatAsNotNullJsonbObject as postgresql_crud::PostgresqlJsonType>::Select>);
impl OptionCatAsNullableJsonbObjectSelect {
    pub fn new(value: std::option::Option<postgresql_crud::NotEmptyUniqueEnumVec<CatAsNotNullJsonbObjectSelectElement>>) -> Self {
        Self(match value {
            Some(value) => Some(<CatAsNotNullJsonbObject as postgresql_crud::PostgresqlJsonType>::Select::new(value)),
            None => None,
        })
    }
}
impl OptionCatAsNullableJsonbObjectSelect {
    fn select_query_part_postgresql_type(&self, column: &std::primitive::str) -> std::string::String {
        format!("case when jsonb_typeof({column}) = 'null' then 'null'::jsonb else ({}) end", {
            let value = match &self.0 {
                Some(value) => value,
                None => &<<CatAsNotNullJsonbObject as postgresql_crud::PostgresqlJsonType>::Select as postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement>::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            };
            value.select_query_part_postgresql_type(column)
        })
    }
}
impl sqlx::Type<sqlx::Postgres> for OptionCatAsNullableJsonbObjectSelect {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <sqlx::types::Json<Self> as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> std::primitive::bool {
        <sqlx::types::Json<Self> as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
impl sqlx::Decode<'_, sqlx::Postgres> for OptionCatAsNullableJsonbObjectSelect {
    fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        match <sqlx::types::Json<Self> as sqlx::Decode<sqlx::Postgres>>::decode(value) {
            Ok(value) => Ok(value.0),
            Err(error) => Err(error),
        }
    }
}
impl postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for OptionCatAsNullableJsonbObjectSelect {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self(Some(postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()))
    }
}
pub type OptionCatAsNullableJsonbObjectWhereElement = postgresql_crud::NullableJsonObjectPostgresqlTypeWhereFilter<<CatAsNotNullJsonbObject as postgresql_crud::PostgresqlJsonType>::WhereElement>;
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionCatAsNullableJsonbObjectRead(std::option::Option<<CatAsNotNullJsonbObject as postgresql_crud::PostgresqlJsonType>::Read>);
impl OptionCatAsNullableJsonbObjectRead {
    fn into_inner(self) -> OptionCatAsNullableJsonbObjectReadInner {
        match self.0 {
            Some(value) => Some(<CatAsNotNullJsonbObject as postgresql_crud::PostgresqlJsonType>::into_inner(value)),
            None => None,
        }
    }
}
impl OptionCatAsNullableJsonbObjectRead {
    pub fn new(value: std::option::Option<<CatAsNotNullJsonbObject as postgresql_crud::PostgresqlJsonType>::Read>) -> Self {
        Self(value)
    }
}
impl postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for OptionCatAsNullableJsonbObjectRead {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self(Some(postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()))
    }
}
impl sqlx::Type<sqlx::Postgres> for OptionCatAsNullableJsonbObjectRead {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <sqlx::types::Json<Self> as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> std::primitive::bool {
        <sqlx::types::Json<Self> as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
impl sqlx::Decode<'_, sqlx::Postgres> for OptionCatAsNullableJsonbObjectRead {
    fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        match <sqlx::types::Json<Self> as sqlx::Decode<sqlx::Postgres>>::decode(value) {
            Ok(value) => Ok(value.0),
            Err(error) => Err(error),
        }
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct OptionCatAsNullableJsonbObjectReadOnlyIds(postgresql_crud::Value<std::option::Option<CatAsNotNullJsonbObjectReadOnlyIds>>);
impl sqlx::Decode<'_, sqlx::Postgres> for OptionCatAsNullableJsonbObjectReadOnlyIds {
    fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        match <sqlx::types::Json<Self> as sqlx::Decode<sqlx::Postgres>>::decode(value) {
            Ok(value) => Ok(value.0),
            Err(error) => Err(error),
        }
    }
}
impl sqlx::Type<sqlx::Postgres> for OptionCatAsNullableJsonbObjectReadOnlyIds {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <sqlx::types::Json<Self> as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> std::primitive::bool {
        <sqlx::types::Json<Self> as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
pub type OptionCatAsNullableJsonbObjectReadInner = std::option::Option<<CatAsNotNullJsonbObject as postgresql_crud::PostgresqlJsonType>::ReadInner>;
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct OptionCatAsNullableJsonbObjectUpdate(std::option::Option<<CatAsNotNullJsonbObject as postgresql_crud::PostgresqlJsonType>::Update>);
impl OptionCatAsNullableJsonbObjectUpdate {
    pub fn new(value: std::option::Option<<CatAsNotNullJsonbObject as postgresql_crud::PostgresqlJsonType>::Update>) -> Self {
        Self(value)
    }
}
impl postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for OptionCatAsNullableJsonbObjectUpdate {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self(Some(postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()))
    }
}
impl postgresql_crud::PostgresqlJsonType for OptionCatAsNullableJsonbObject {
    type TableTypeDeclaration = OptionCatAsNullableJsonbObjectTableTypeDeclaration;
    type Create = OptionCatAsNullableJsonbObjectCreate;
    fn create_query_part(value: &Self::Create, increment: &mut std::primitive::u64) -> Result<std::string::String, postgresql_crud::QueryPartErrorNamed> {
        value.create_query_part(increment)
    }
    fn create_query_bind(value: Self::Create, query: sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments> {
        value.create_query_bind(query)
    }
    type Select = OptionCatAsNullableJsonbObjectSelect;
    fn select_query_part(value: &Self::Select, field_ident: &std::primitive::str, column_name_and_maybe_field_getter: &std::primitive::str, column_name_and_maybe_field_getter_for_error_message: &std::primitive::str, is_postgresql_type: std::primitive::bool) -> std::string::String {
        let column_name_and_maybe_field_getter_field_ident = format!("{column_name_and_maybe_field_getter}->'{field_ident}'");
        format!("jsonb_build_object('{field_ident}',jsonb_build_object('value',case when jsonb_typeof({column_name_and_maybe_field_getter_field_ident}) = 'null' then 'null'::jsonb else ({}) end))", {
            let value = match &value.0 {
                Some(value) => value,
                None => &<<CatAsNotNullJsonbObject as postgresql_crud::PostgresqlJsonType>::Select as postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement>::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            };
            <CatAsNotNullJsonbObject as postgresql_crud::PostgresqlJsonType>::select_query_part(value, field_ident, &column_name_and_maybe_field_getter_field_ident, column_name_and_maybe_field_getter_for_error_message, true)
        })
    }
    type WhereElement = OptionCatAsNullableJsonbObjectWhereElement;
    type Read = OptionCatAsNullableJsonbObjectRead;
    type ReadOnlyIds = OptionCatAsNullableJsonbObjectReadOnlyIds;
    fn select_only_ids_query_part(column_name_and_maybe_field_getter: &std::primitive::str) -> std::string::String {
        format!("jsonb_build_object('value',case when jsonb_typeof({column_name_and_maybe_field_getter})='null' then 'null'::jsonb else {} end)", <CatAsNotNullJsonbObject as postgresql_crud::PostgresqlJsonType>::select_only_ids_query_part(column_name_and_maybe_field_getter),)
    }
    type ReadInner = OptionCatAsNullableJsonbObjectReadInner;
    fn into_inner(value: Self::Read) -> Self::ReadInner {
        value.into_inner()
    }
    type Update = OptionCatAsNullableJsonbObjectUpdate;
    fn update_query_part(value: &Self::Update, jsonb_set_accumulator: &std::primitive::str, jsonb_set_target: &std::primitive::str, jsonb_set_path: &std::primitive::str, increment: &mut std::primitive::u64) -> Result<std::string::String, postgresql_crud::QueryPartErrorNamed> {
        match &value.0 {
            Some(value) => <CatAsNotNullJsonbObject as postgresql_crud::PostgresqlJsonType>::update_query_part(value, jsonb_set_accumulator, jsonb_set_target, jsonb_set_path, increment),
            None => match increment.checked_add(1) {
                Some(value) => {
                    *increment = value;
                    Ok(format!("jsonb_set({jsonb_set_accumulator},'{{{jsonb_set_path}}}',${increment})"))
                }
                None => Err(postgresql_crud::QueryPartErrorNamed::CheckedAdd { code_occurence: error_occurence_lib::code_occurence!() }),
            },
        }
    }
    fn update_query_bind(value: Self::Update, query: sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments> {
        match value.0 {
            Some(value) => <CatAsNotNullJsonbObject as postgresql_crud::PostgresqlJsonType>::update_query_bind(value, query),
            None => query.bind(sqlx::types::Json(<OptionCatAsNullableJsonbObject as postgresql_crud::PostgresqlJsonType>::Update::new(None))),
        }
    }
    fn select_only_updated_ids_query_part(value: &Self::Update, field_ident: &std::primitive::str, column_name_and_maybe_field_getter: &std::primitive::str, increment: &mut std::primitive::u64) -> Result<std::string::String, postgresql_crud::QueryPartErrorNamed> {
        match &value.0 {
            Some(value) => {
                let mut acc = std::string::String::default();
                for element in value.0.to_vec() {
                    match &element {
                        CatAsNotNullJsonbObjectUpdateElement::Field444(value) => match <postgresql_crud::postgresql_json_type::StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud::PostgresqlJsonType>::select_only_updated_ids_query_part(&value.value, &"field_444", &format!("{column_name_and_maybe_field_getter}->'{field_ident}'"), increment) {
                            Ok(value) => {
                                acc.push_str(&value);
                            }
                            Err(error) => {
                                return Err(error);
                            }
                        },
                    }
                }
                let _ = acc.pop();
                Ok(format!("'{field_ident}',jsonb_build_object('value',jsonb_build_object('value',jsonb_build_object({acc}))),"))
            }
            None => Ok(format!("'{field_ident}',jsonb_build_object('value','null'::jsonb),")),
        }
    }
}
impl postgresql_crud::PostgresqlType for OptionCatAsNullableJsonbObject {
    type TableTypeDeclaration = OptionCatAsNullableJsonbObjectTableTypeDeclaration;
    type Create = OptionCatAsNullableJsonbObjectCreate;
    fn create_query_part(value: &Self::Create, increment: &mut std::primitive::u64) -> Result<std::string::String, postgresql_crud::QueryPartErrorNamed> {
        value.create_query_part(increment)
    }
    fn create_query_bind(value: Self::Create, query: sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments> {
        value.create_query_bind(query)
    }
    type Select = OptionCatAsNullableJsonbObjectSelect;
    fn select_query_part(value: &Self::Select, column: &std::primitive::str) -> std::string::String {
        format!("{} as {column}", value.select_query_part_postgresql_type(column))
    }
    type WhereElement = OptionCatAsNullableJsonbObjectWhereElement;
    type Read = OptionCatAsNullableJsonbObjectRead;
    fn normalize(value: Self::Read) -> Self::Read {
        value
    }
    type ReadOnlyIds = OptionCatAsNullableJsonbObjectReadOnlyIds;
    fn select_only_ids_query_part(column: &std::primitive::str, is_primary_key: std::primitive::bool) -> std::string::String {
        format!("{} as {column},", <OptionCatAsNullableJsonbObject as postgresql_crud::PostgresqlJsonType>::select_only_ids_query_part(&column))
    }
    type ReadInner = OptionCatAsNullableJsonbObjectReadInner;
    fn into_inner(value: Self::Read) -> Self::ReadInner {
        value.into_inner()
    }
    type Update = OptionCatAsNullableJsonbObjectUpdate;
    fn update_query_part(value: &Self::Update, jsonb_set_accumulator: &std::primitive::str, jsonb_set_target: &std::primitive::str, jsonb_set_path: &std::primitive::str, increment: &mut std::primitive::u64) -> Result<std::string::String, postgresql_crud::QueryPartErrorNamed> {
        match &value.0 {
            Some(value) => <CatAsNotNullJsonbObject as postgresql_crud::PostgresqlType>::update_query_part(value, jsonb_set_accumulator, jsonb_set_target, jsonb_set_path, increment),
            None => match increment.checked_add(1) {
                Some(value) => {
                    *increment = value;
                    Ok(format!("${increment}"))
                }
                None => Err(postgresql_crud::QueryPartErrorNamed::CheckedAdd { code_occurence: error_occurence_lib::code_occurence!() }),
            },
        }
    }
    fn update_query_bind<'a>(value: Self::Update, query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
        match value.0 {
            Some(value) => <CatAsNotNullJsonbObject as postgresql_crud::PostgresqlType>::update_query_bind(value, query),
            None => query.bind(sqlx::types::Json(<OptionCatAsNullableJsonbObject as postgresql_crud::PostgresqlJsonType>::Update::new(None))),
        }
    }
    fn select_only_updated_ids_query_part(value: &Self::Update, column: &std::primitive::str, increment: &mut std::primitive::u64, is_primary_key: std::primitive::bool) -> Result<std::string::String, postgresql_crud::QueryPartErrorNamed> {
        match &value.0 {
            Some(value) => {
                let mut acc = std::string::String::default();
                for element in value.0.to_vec() {
                    match &element {
                        CatAsNotNullJsonbObjectUpdateElement::Field444(value) => match <postgresql_crud::postgresql_json_type::StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud::PostgresqlJsonType>::select_only_updated_ids_query_part(&value.value, &"field_444", &format!("{column}"), increment) {
                            Ok(value) => {
                                acc.push_str(&value);
                            }
                            Err(error) => {
                                return Err(error);
                            }
                        },
                    }
                }
                let _ = acc.pop();
                Ok(format!("jsonb_build_object('value',jsonb_build_object('value',jsonb_build_object({acc}))) as {column},"))
            }
            None => Ok(format!("jsonb_build_object('value','null'::jsonb) as {column},")),
        }
    }
}
#[cfg(test)]
impl postgresql_crud::tests::PostgresqlTypeTestCases for OptionCatAsNullableJsonbObject {
    type Element = Self;
    fn test_cases(read_only_ids: &<Self::Element as postgresql_crud::PostgresqlType>::ReadOnlyIds) -> std::vec::Vec<std::vec::Vec<<Self::Element as postgresql_crud::PostgresqlType>::ReadInner>> {
        let mut acc = vec![];
        if let Some(value) = &read_only_ids.0.value {
            for element in <CatAsNotNullJsonbObject as postgresql_crud::tests::PostgresqlJsonTypeTestCases>::test_cases(&value) {
                for current_element in element {
                    acc.push(Some(current_element));
                }
            }
        }
        acc.push(None);
        vec![acc]
    }
    fn read_new_or_try_new_unwraped_for_test(value: OptionCatAsNullableJsonbObjectReadInner) -> <Self::Element as postgresql_crud::PostgresqlType>::Read {
        <Self::Element as postgresql_crud::PostgresqlType>::Read::new(match value {
            Some(value) => Some(<CatAsNotNullJsonbObject as postgresql_crud::tests::PostgresqlJsonTypeTestCases>::read_new_or_try_new_unwraped_for_test(value)),
            None => None,
        })
    }
    fn update_new_or_try_new_unwraped_for_test(value: OptionCatAsNullableJsonbObjectReadInner) -> <Self::Element as postgresql_crud::PostgresqlType>::Update {
        <Self::Element as postgresql_crud::PostgresqlType>::Update::new(match value {
            Some(value) => Some(<CatAsNotNullJsonbObject as postgresql_crud::tests::PostgresqlTypeTestCases>::update_new_or_try_new_unwraped_for_test(value)),
            None => None,
        })
    }
    fn read_only_ids_to_option_value_read_inner(value: <Self::Element as postgresql_crud::PostgresqlType>::ReadOnlyIds) -> std::option::Option<postgresql_crud::Value<<Self::Element as postgresql_crud::PostgresqlType>::ReadInner>> {
        todo!()
    }
    fn update_to_read_only_ids(value: &<Self::Element as postgresql_crud::PostgresqlType>::Update) -> <Self::Element as postgresql_crud::PostgresqlType>::ReadOnlyIds {
        OptionCatAsNullableJsonbObjectReadOnlyIds(postgresql_crud::Value {
            value: match &value.0 {
                Some(value) => Some(<CatAsNotNullJsonbObject as postgresql_crud::tests::PostgresqlJsonTypeTestCases>::update_to_read_only_ids(&value)),
                None => None,
            },
        })
    }
}
#[cfg(test)]
impl postgresql_crud::tests::PostgresqlJsonTypeTestCases for OptionCatAsNullableJsonbObject {
    type Element = Self;
    fn test_cases(read_only_ids: &<Self::Element as postgresql_crud::PostgresqlJsonType>::ReadOnlyIds) -> std::vec::Vec<std::vec::Vec<<Self::Element as postgresql_crud::PostgresqlJsonType>::ReadInner>> {
        let mut acc = vec![];
        if let Some(value) = &read_only_ids.0.value {
            for element in <CatAsNotNullJsonbObject as postgresql_crud::tests::PostgresqlJsonTypeTestCases>::test_cases(&value) {
                for current_element in element {
                    acc.push(Some(current_element));
                }
            }
        }
        acc.push(None);
        vec![acc]
    }
    fn read_new_or_try_new_unwraped_for_test(value: OptionCatAsNullableJsonbObjectReadInner) -> <Self::Element as postgresql_crud::PostgresqlJsonType>::Read {
        <Self::Element as postgresql_crud::PostgresqlType>::Read::new(match value {
            Some(value) => Some(<CatAsNotNullJsonbObject as postgresql_crud::tests::PostgresqlJsonTypeTestCases>::read_new_or_try_new_unwraped_for_test(value)),
            None => None,
        })
    }
    fn update_new_or_try_new_unwraped_for_test(value: OptionCatAsNullableJsonbObjectReadInner) -> <Self::Element as postgresql_crud::PostgresqlJsonType>::Update {
        <Self::Element as postgresql_crud::PostgresqlType>::Update::new(match value {
            Some(value) => Some(<CatAsNotNullJsonbObject as postgresql_crud::tests::PostgresqlTypeTestCases>::update_new_or_try_new_unwraped_for_test(value)),
            None => None,
        })
    }
    fn read_only_ids_to_option_value_read_inner(value: <Self::Element as postgresql_crud::PostgresqlJsonType>::ReadOnlyIds) -> std::option::Option<postgresql_crud::Value<<Self::Element as postgresql_crud::PostgresqlJsonType>::ReadInner>> {
        Some(postgresql_crud::Value {
            value: match value.0.value {
                Some(value) => match <CatAsNotNullJsonbObject as postgresql_crud::tests::PostgresqlJsonTypeTestCases>::read_only_ids_to_option_value_read_inner(value) {
                    Some(value) => Some(value.value),
                    None => None,
                },
                None => None,
            },
        })
    }
    fn update_to_read_only_ids(value: &<Self::Element as postgresql_crud::PostgresqlJsonType>::Update) -> <Self::Element as postgresql_crud::PostgresqlJsonType>::ReadOnlyIds {
        OptionCatAsNullableJsonbObjectReadOnlyIds(postgresql_crud::Value {
            value: match &value.0 {
                Some(value) => Some(<CatAsNotNullJsonbObject as postgresql_crud::tests::PostgresqlJsonTypeTestCases>::update_to_read_only_ids(&value)),
                None => None,
            },
        })
    }
}
#[derive(Debug)]
pub struct VecOfCatWithIdAsNotNullArrayOfNotNullJsonbObjectWithId;
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfCatWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdTableTypeDeclaration(std::vec::Vec<<CatWithIdAsNotNullJsonbObjectWithId as postgresql_crud::PostgresqlJsonType>::TableTypeDeclaration>);
impl VecOfCatWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdTableTypeDeclaration {
    pub fn new(value: std::vec::Vec<<CatWithIdAsNotNullJsonbObjectWithId as postgresql_crud::PostgresqlJsonType>::TableTypeDeclaration>) -> Self {
        Self(value)
    }
}
impl postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for VecOfCatWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdTableTypeDeclaration {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self(vec![postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()])
    }
}
impl VecOfCatWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdTableTypeDeclaration {
    pub fn create_table_column_query_part(column: &dyn std::fmt::Display, _: std::primitive::bool) -> impl std::fmt::Display {
        format!("{column} jsonb not null check (jsonb_matches_schema('{}', {column}))", serde_json::to_string(&schemars::schema_for!(VecOfCatWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdTableTypeDeclaration)).unwrap())
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfCatWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdCreate(std::vec::Vec<<CatWithIdAsNotNullJsonbObjectWithId as postgresql_crud::PostgresqlJsonType>::Create>);
impl VecOfCatWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdCreate {
    pub fn new(value: std::vec::Vec<<CatWithIdAsNotNullJsonbObjectWithId as postgresql_crud::PostgresqlJsonType>::Create>) -> Self {
        Self(value)
    }
}
impl postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for VecOfCatWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdCreate {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self(vec![postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()])
    }
}
impl std::fmt::Display for VecOfCatWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdCreate {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{:?}", self)
    }
}
impl error_occurence_lib::ToStdStringString for VecOfCatWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdCreate {
    fn to_std_string_string(&self) -> std::string::String {
        format!("{self:?}")
    }
}
impl VecOfCatWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdCreate {
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
        Ok(format!("jsonb_build_array({acc})"))
    }
    fn create_query_bind(self, mut query: sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments> {
        for element in self.0 {
            query = element.create_query_bind(query);
        }
        query
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfCatWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdSelect {
    cat_with_id_as_not_null_jsonb_object_with_id_select: <CatWithIdAsNotNullJsonbObjectWithId as postgresql_crud::PostgresqlJsonType>::Select,
    dimension1_pagination: postgresql_crud::PaginationStartsWithZero,
}
impl VecOfCatWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdSelect {
    pub fn new(cat_with_id_as_not_null_jsonb_object_with_id_select: <CatWithIdAsNotNullJsonbObjectWithId as postgresql_crud::PostgresqlJsonType>::Select, dimension1_pagination: postgresql_crud::PaginationStartsWithZero) -> Self {
        Self { cat_with_id_as_not_null_jsonb_object_with_id_select, dimension1_pagination }
    }
}
impl VecOfCatWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdSelect {
    fn select_query_part_postgresql_type(&self, column: &std::primitive::str) -> std::string::String {
        let field_ident = column;
        let column_name_and_maybe_field_getter = column;
        let cat_with_id_as_not_null_jsonb_object_with_id_select = <CatWithIdAsNotNullJsonbObjectWithId as postgresql_crud::PostgresqlJsonType>::select_query_part(&self.cat_with_id_as_not_null_jsonb_object_with_id_select, field_ident, &"value", &"value", true);
        let dimension1_start = self.dimension1_pagination.start();
        let dimension1_end = self.dimension1_pagination.end();
        format!("(case when (jsonb_array_length({column_name_and_maybe_field_getter}) = 0) then '[]'::jsonb else (select jsonb_agg(({cat_with_id_as_not_null_jsonb_object_with_id_select})) from jsonb_array_elements((select {column_name_and_maybe_field_getter})) with ordinality where ordinality between {dimension1_start} and {dimension1_end}) end)")
    }
}
impl sqlx::Type<sqlx::Postgres> for VecOfCatWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdSelect {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <sqlx::types::Json<Self> as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> std::primitive::bool {
        <sqlx::types::Json<Self> as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
impl sqlx::Decode<'_, sqlx::Postgres> for VecOfCatWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdSelect {
    fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        match <sqlx::types::Json<Self> as sqlx::Decode<sqlx::Postgres>>::decode(value) {
            Ok(value) => Ok(value.0),
            Err(error) => Err(error),
        }
    }
}
impl postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for VecOfCatWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdSelect {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self {
            cat_with_id_as_not_null_jsonb_object_with_id_select: postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            dimension1_pagination: postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub enum VecOfCatWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdWhereElement {
    Equal(postgresql_crud::where_element_filters::PostgresqlJsonTypeWhereElementEqual<<VecOfCatWithIdAsNotNullArrayOfNotNullJsonbObjectWithId as postgresql_crud::PostgresqlJsonType>::TableTypeDeclaration>),
    DimensionOneEqual(postgresql_crud::where_element_filters::PostgresqlJsonTypeWhereElementDimensionOneEqual<<CatWithIdAsNotNullJsonbObjectWithId as postgresql_crud::PostgresqlJsonType>::TableTypeDeclaration>),
    LengthEqual(postgresql_crud::where_element_filters::PostgresqlJsonTypeWhereElementLengthEqual),
    LengthMoreThan(postgresql_crud::where_element_filters::PostgresqlJsonTypeWhereElementLengthMoreThan),
    In(postgresql_crud::where_element_filters::PostgresqlJsonTypeWhereElementIn<<VecOfCatWithIdAsNotNullArrayOfNotNullJsonbObjectWithId as postgresql_crud::PostgresqlJsonType>::TableTypeDeclaration>),
    DimensionOneIn(postgresql_crud::where_element_filters::PostgresqlJsonTypeWhereElementDimensionOneIn<<CatWithIdAsNotNullJsonbObjectWithId as postgresql_crud::PostgresqlJsonType>::TableTypeDeclaration>),
    ContainsAllElementsOfArray(postgresql_crud::where_element_filters::PostgresqlJsonTypeWhereElementContainsAllElementsOfArray<<CatWithIdAsNotNullJsonbObjectWithId as postgresql_crud::PostgresqlJsonType>::TableTypeDeclaration>),
    OverlapsWithArray(postgresql_crud::where_element_filters::PostgresqlJsonTypeWhereElementOverlapsWithArray<<CatWithIdAsNotNullJsonbObjectWithId as postgresql_crud::PostgresqlJsonType>::TableTypeDeclaration>),
    ElementId(postgresql_crud::PostgresqlTypeWhere<<postgresql_crud::postgresql_json_type::UuidUuidAsNotNullJsonbString as postgresql_crud::PostgresqlJsonType>::WhereElement>),
    ElementField444(postgresql_crud::PostgresqlTypeWhere<<postgresql_crud::postgresql_json_type::StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud::PostgresqlJsonType>::WhereElement>),
}
impl<'a> postgresql_crud::PostgresqlTypeWhereFilter<'a> for VecOfCatWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdWhereElement {
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
            Self::ElementField444(value) => generate_element_query(value, &"field_444"),
        }
    }
    fn query_bind(self, query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
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
            Self::ElementField444(value) => postgresql_crud::PostgresqlTypeWhereFilter::query_bind(value, query),
        }
    }
}
impl error_occurence_lib::ToStdStringString for VecOfCatWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdWhereElement {
    fn to_std_string_string(&self) -> std::string::String {
        format!("{self:?}")
    }
}
impl postgresql_crud::AllEnumVariantsArrayDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for VecOfCatWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdWhereElement {
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
            Self::ElementField444(postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
        ]
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfCatWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdRead(std::vec::Vec<<CatWithIdAsNotNullJsonbObjectWithId as postgresql_crud::PostgresqlJsonType>::Read>);
impl VecOfCatWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdRead {
    fn into_inner(self) -> VecOfCatWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdReadInner {
        self.0.into_iter().map(|element| <CatWithIdAsNotNullJsonbObjectWithId as postgresql_crud::PostgresqlJsonType>::into_inner(element)).collect()
    }
}
impl VecOfCatWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdRead {
    pub fn new(value: std::vec::Vec<<CatWithIdAsNotNullJsonbObjectWithId as postgresql_crud::PostgresqlJsonType>::Read>) -> Self {
        Self(value)
    }
}
impl postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for VecOfCatWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdRead {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self(vec![postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()])
    }
}
impl sqlx::Type<sqlx::Postgres> for VecOfCatWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdRead {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <sqlx::types::Json<Self> as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> std::primitive::bool {
        <sqlx::types::Json<Self> as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
impl sqlx::Decode<'_, sqlx::Postgres> for VecOfCatWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdRead {
    fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        match <sqlx::types::Json<Self> as sqlx::Decode<sqlx::Postgres>>::decode(value) {
            Ok(value) => Ok(value.0),
            Err(error) => Err(error),
        }
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct VecOfCatWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdReadOnlyIds(postgresql_crud::Value<std::vec::Vec<CatWithIdAsNotNullJsonbObjectWithIdReadOnlyIds>>);
impl sqlx::Decode<'_, sqlx::Postgres> for VecOfCatWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdReadOnlyIds {
    fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        match <sqlx::types::Json<Self> as sqlx::Decode<sqlx::Postgres>>::decode(value) {
            Ok(value) => Ok(value.0),
            Err(error) => Err(error),
        }
    }
}
impl sqlx::Type<sqlx::Postgres> for VecOfCatWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdReadOnlyIds {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <sqlx::types::Json<Self> as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> std::primitive::bool {
        <sqlx::types::Json<Self> as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
pub type VecOfCatWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdReadInner = std::vec::Vec<<CatWithIdAsNotNullJsonbObjectWithId as postgresql_crud::PostgresqlJsonType>::ReadInner>;
#[derive(Debug, Clone, PartialEq, serde :: Serialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfCatWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdUpdate {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    create: std::vec::Vec<<CatWithIdAsNotNullJsonbObjectWithId as postgresql_crud::PostgresqlJsonType>::Create>,
    update: CatWithIdAsNotNullJsonbObjectWithIdUpdateHandle,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    delete: std::vec::Vec<<postgresql_crud::postgresql_json_type::UuidUuidAsNotNullJsonbString as postgresql_crud::PostgresqlJsonType>::Update>,
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum VecOfCatWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdUpdateTryNewErrorNamed {
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
impl VecOfCatWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdUpdate {
    pub fn try_new(create: std::vec::Vec<<CatWithIdAsNotNullJsonbObjectWithId as postgresql_crud::PostgresqlJsonType>::Create>, update: CatWithIdAsNotNullJsonbObjectWithIdUpdateHandle, delete: std::vec::Vec<<postgresql_crud::postgresql_json_type::UuidUuidAsNotNullJsonbString as postgresql_crud::PostgresqlJsonType>::Update>) -> Result<Self, VecOfCatWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdUpdateTryNewErrorNamed> {
        if create.is_empty() && update.0.is_empty() && delete.is_empty() {
            return Err(VecOfCatWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdUpdateTryNewErrorNamed::CreateUpdateDeleteAreEmpty { code_occurence: error_occurence_lib::code_occurence!() });
        }
        {
            let update_acc = {
                let mut update_acc = vec![];
                for element in update.0.to_vec() {
                    let id = &element.id;
                    if update_acc.contains(&id) {
                        return Err(VecOfCatWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdUpdateTryNewErrorNamed::NotUniqueIdInJsonUpdateArray {
                            error: format!("custom serde error deserializing VecOfCatWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdUpdate: not unique id in json update array: {}", id.get_inner()),
                            code_occurence: error_occurence_lib::code_occurence!(),
                        });
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
                        return Err(VecOfCatWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdUpdateTryNewErrorNamed::NotUniqueIdInJsonDeleteArray {
                            error: format!("custom serde error deserializing VecOfCatWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdUpdate: not unique id in json delete array: {}", element.get_inner()),
                            code_occurence: error_occurence_lib::code_occurence!(),
                        });
                    } else {
                        delete_acc.push(element);
                    }
                }
                delete_acc
            };
            for element in update_acc {
                if delete_acc.contains(&&element) {
                    return Err(VecOfCatWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdUpdateTryNewErrorNamed::NotUniqueIdInJsonUpdateAndDeleteArrays {
                        error: format!("custom serde error deserializing VecOfCatWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdUpdate: not unique id in json update and delete arrays: {}", element.get_inner()),
                        code_occurence: error_occurence_lib::code_occurence!(),
                    });
                }
            }
        }
        Ok(Self { create, update, delete })
    }
}
impl<'de> serde::Deserialize<'de> for VecOfCatWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdUpdate {
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
            marker: serde::__private::PhantomData<VecOfCatWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdUpdate>,
            lifetime: serde::__private::PhantomData<&'de ()>,
        }
        impl<'de> serde::de::Visitor<'de> for __Visitor<'de> {
            type Value = VecOfCatWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdUpdate;
            fn expecting(&self, __f: &mut serde::__private::Formatter<'_>) -> serde::__private::fmt::Result {
                serde::__private::Formatter::write_str(__f, "tuple struct VecOfCatWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdUpdate")
            }
            #[inline]
            fn visit_seq<__A>(self, mut __seq: __A) -> serde::__private::Result<Self::Value, __A::Error>
            where
                __A: serde::de::SeqAccess<'de>,
            {
                let __field0 = match serde::de::SeqAccess::next_element::<std::vec::Vec<<CatWithIdAsNotNullJsonbObjectWithId as postgresql_crud::PostgresqlJsonType>::Create>>(&mut __seq)? {
                    serde::__private::Some(__value) => __value,
                    serde::__private::None => {
                        vec![]
                    }
                };
                let __field1 = match serde::de::SeqAccess::next_element::<CatWithIdAsNotNullJsonbObjectWithIdUpdateHandle>(&mut __seq)? {
                    serde::__private::Some(__value) => __value,
                    serde::__private::None => CatWithIdAsNotNullJsonbObjectWithIdUpdateHandle::default(),
                };
                let __field2 = match serde::de::SeqAccess::next_element::<std::vec::Vec<<postgresql_crud::postgresql_json_type::UuidUuidAsNotNullJsonbString as postgresql_crud::PostgresqlJsonType>::Update>>(&mut __seq)? {
                    serde::__private::Some(__value) => __value,
                    serde::__private::None => {
                        vec![]
                    }
                };
                match VecOfCatWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdUpdate::try_new(__field0, __field1, __field2) {
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
                let mut __field0: serde::__private::Option<std::vec::Vec<<CatWithIdAsNotNullJsonbObjectWithId as postgresql_crud::PostgresqlJsonType>::Create>> = serde::__private::None;
                let mut __field1: serde::__private::Option<CatWithIdAsNotNullJsonbObjectWithIdUpdateHandle> = serde::__private::None;
                let mut __field2: serde::__private::Option<std::vec::Vec<<postgresql_crud::postgresql_json_type::UuidUuidAsNotNullJsonbString as postgresql_crud::PostgresqlJsonType>::Update>> = serde::__private::None;
                while let serde::__private::Some(__key) = serde::de::MapAccess::next_key::<__Field>(&mut __map)? {
                    match __key {
                        __Field::__field0 => {
                            if serde::__private::Option::is_some(&__field0) {
                                return serde::__private::Err(<__A::Error as serde::de::Error>::duplicate_field("create"));
                            }
                            __field0 = serde::__private::Some(serde::de::MapAccess::next_value::<std::vec::Vec<<CatWithIdAsNotNullJsonbObjectWithId as postgresql_crud::PostgresqlJsonType>::Create>>(&mut __map)?);
                        }
                        __Field::__field1 => {
                            if serde::__private::Option::is_some(&__field1) {
                                return serde::__private::Err(<__A::Error as serde::de::Error>::duplicate_field("update"));
                            }
                            __field1 = serde::__private::Some(serde::de::MapAccess::next_value::<CatWithIdAsNotNullJsonbObjectWithIdUpdateHandle>(&mut __map)?);
                        }
                        __Field::__field2 => {
                            if serde::__private::Option::is_some(&__field2) {
                                return serde::__private::Err(<__A::Error as serde::de::Error>::duplicate_field("delete"));
                            }
                            __field2 = serde::__private::Some(serde::de::MapAccess::next_value::<std::vec::Vec<<postgresql_crud::postgresql_json_type::UuidUuidAsNotNullJsonbString as postgresql_crud::PostgresqlJsonType>::Update>>(&mut __map)?);
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
                    serde::__private::None => CatWithIdAsNotNullJsonbObjectWithIdUpdateHandle::default(),
                };
                let __field2 = match __field2 {
                    serde::__private::Some(__field2) => __field2,
                    serde::__private::None => {
                        vec![]
                    }
                };
                match VecOfCatWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdUpdate::try_new(__field0, __field1, __field2) {
                    Ok(value) => serde::__private::Ok(value),
                    Err(error) => {
                        return Err(serde::de::Error::custom(format!("{error:?}")));
                    }
                }
            }
        }
        #[doc(hidden)]
        const FIELDS: &'static [&'static str] = &["create", "update", "delete"];
        serde::Deserializer::deserialize_struct(
            __deserializer,
            "VecOfCatWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdUpdate",
            FIELDS,
            __Visitor {
                marker: serde::__private::PhantomData::<VecOfCatWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdUpdate>,
                lifetime: serde::__private::PhantomData,
            },
        )
    }
}
impl postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for VecOfCatWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdUpdate {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self {
            create: vec![postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()],
            update: postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            delete: vec![postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()],
        }
    }
}
impl postgresql_crud::PostgresqlJsonType for VecOfCatWithIdAsNotNullArrayOfNotNullJsonbObjectWithId {
    type TableTypeDeclaration = VecOfCatWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdTableTypeDeclaration;
    type Create = VecOfCatWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdCreate;
    fn create_query_part(value: &Self::Create, increment: &mut std::primitive::u64) -> Result<std::string::String, postgresql_crud::QueryPartErrorNamed> {
        value.create_query_part(increment)
    }
    fn create_query_bind(value: Self::Create, query: sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments> {
        value.create_query_bind(query)
    }
    type Select = VecOfCatWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdSelect;
    fn select_query_part(value: &Self::Select, field_ident: &std::primitive::str, column_name_and_maybe_field_getter: &std::primitive::str, column_name_and_maybe_field_getter_for_error_message: &std::primitive::str, is_postgresql_type: std::primitive::bool) -> std::string::String {
        let cat_with_id_as_not_null_jsonb_object_with_id_select = <CatWithIdAsNotNullJsonbObjectWithId as postgresql_crud::PostgresqlJsonType>::select_query_part(&value.cat_with_id_as_not_null_jsonb_object_with_id_select, field_ident, &"value", &"value", true);
        let dimension1_start = value.dimension1_pagination.start();
        let dimension1_end = value.dimension1_pagination.end();
        format!("jsonb_build_object('{field_ident}',jsonb_build_object('value',case when (jsonb_array_length({column_name_and_maybe_field_getter}->'{field_ident}') = 0) then '[]'::jsonb else (select jsonb_agg(({cat_with_id_as_not_null_jsonb_object_with_id_select})) from jsonb_array_elements((select {column_name_and_maybe_field_getter}->'{field_ident}')) with ordinality where ordinality between {dimension1_start} and {dimension1_end}) end ))")
    }
    type WhereElement = VecOfCatWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdWhereElement;
    type Read = VecOfCatWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdRead;
    type ReadOnlyIds = VecOfCatWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdReadOnlyIds;
    fn select_only_ids_query_part(column_name_and_maybe_field_getter: &std::primitive::str) -> std::string::String {
        format!("jsonb_build_object('value',(select jsonb_agg({}) from jsonb_array_elements({column_name_and_maybe_field_getter}) as elem))", <CatWithIdAsNotNullJsonbObjectWithId as postgresql_crud::PostgresqlJsonType>::select_only_ids_query_part("elem"),)
    }
    type ReadInner = VecOfCatWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdReadInner;
    fn into_inner(value: Self::Read) -> Self::ReadInner {
        value.into_inner()
    }
    type Update = VecOfCatWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdUpdate;
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
                        match <CatAsNotNullJsonbObject as postgresql_crud::PostgresqlJsonType>::update_query_part(&element_handle.fields, &"", &"elem", &"", increment) {
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
        Ok(format!(
            "jsonb_set({jsonb_set_accumulator},'{{{jsonb_set_path}}}',case when jsonb_typeof({jsonb_set_target}) = 'null' then '[]'::jsonb else (select coalesce((select jsonb_agg({update_query_part_acc}) from jsonb_array_elements({jsonb_set_target}) as elem {maybe_where}),'[]'::jsonb)) end {maybe_jsonb_build_array})"
        ))
    }
    fn update_query_bind(value: Self::Update, mut query: sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments> {
        for element in value.update.0.into_vec() {
            query = element.id.query_bind_as_postgresql_text(query);
            query = <CatAsNotNullJsonbObject as postgresql_crud::PostgresqlJsonType>::update_query_bind(element.fields, query);
        }
        for element in value.delete {
            query = element.query_bind_as_postgresql_text(query);
        }
        for element in value.create {
            query = element.create_query_bind(query);
        }
        query
    }
    fn select_only_updated_ids_query_part(value: &Self::Update, field_ident: &std::primitive::str, column_name_and_maybe_field_getter: &std::primitive::str, increment: &mut std::primitive::u64) -> Result<std::string::String, postgresql_crud::QueryPartErrorNamed> {
        println!("@@@14");
        Ok(format!("'{field_ident}',jsonb_build_object('value',(select jsonb_agg({}) from jsonb_array_elements({column_name_and_maybe_field_getter}->'{field_ident}') as elem)),", {
            match <CatWithIdAsNotNullJsonbObjectWithId as postgresql_crud::PostgresqlJsonType>::select_only_updated_ids_query_part(&value.update, &"", &"elem", increment) {
                Ok(value) => value,
                Err(error) => {
                    return Err(error);
                }
            }
        }))
    }
}
impl postgresql_crud::PostgresqlType for VecOfCatWithIdAsNotNullArrayOfNotNullJsonbObjectWithId {
    type TableTypeDeclaration = VecOfCatWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdTableTypeDeclaration;
    type Create = VecOfCatWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdCreate;
    fn create_query_part(value: &Self::Create, increment: &mut std::primitive::u64) -> Result<std::string::String, postgresql_crud::QueryPartErrorNamed> {
        value.create_query_part(increment)
    }
    fn create_query_bind(value: Self::Create, query: sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments> {
        value.create_query_bind(query)
    }
    type Select = VecOfCatWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdSelect;
    fn select_query_part(value: &Self::Select, column: &std::primitive::str) -> std::string::String {
        format!("{} as {column}", value.select_query_part_postgresql_type(column))
    }
    type WhereElement = VecOfCatWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdWhereElement;
    type Read = VecOfCatWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdRead;
    fn normalize(value: Self::Read) -> Self::Read {
        value
    }
    type ReadOnlyIds = VecOfCatWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdReadOnlyIds;
    fn select_only_ids_query_part(column: &std::primitive::str, is_primary_key: std::primitive::bool) -> std::string::String {
        format!("{} as {column},", <VecOfCatWithIdAsNotNullArrayOfNotNullJsonbObjectWithId as postgresql_crud::PostgresqlJsonType>::select_only_ids_query_part(&column))
    }
    type ReadInner = VecOfCatWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdReadInner;
    fn into_inner(value: Self::Read) -> Self::ReadInner {
        value.into_inner()
    }
    type Update = VecOfCatWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdUpdate;
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
                        match <CatAsNotNullJsonbObject as postgresql_crud::PostgresqlJsonType>::update_query_part(&element_handle.fields, &"", &"elem", &"", increment) {
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
        Ok(format!(
            "jsonb_set({jsonb_set_accumulator},'{{{jsonb_set_path}}}',case when jsonb_typeof({jsonb_set_target}) = 'null' then '[]'::jsonb else (select coalesce((select jsonb_agg({update_query_part_acc}) from jsonb_array_elements({jsonb_set_target}) as elem {maybe_where}),'[]'::jsonb)) end {maybe_jsonb_build_array})"
        ))
    }
    fn update_query_bind<'a>(value: Self::Update, mut query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
        for element in value.update.0.into_vec() {
            query = element.id.query_bind_as_postgresql_text(query);
            query = <CatAsNotNullJsonbObject as postgresql_crud::PostgresqlJsonType>::update_query_bind(element.fields, query);
        }
        for element in value.delete {
            query = element.query_bind_as_postgresql_text(query);
        }
        for element in value.create {
            query = element.create_query_bind(query);
        }
        query
    }
    fn select_only_updated_ids_query_part(value: &Self::Update, column: &std::primitive::str, increment: &mut std::primitive::u64, is_primary_key: std::primitive::bool) -> Result<std::string::String, postgresql_crud::QueryPartErrorNamed> {
        Ok(format!("jsonb_build_object('value',(select jsonb_agg({}) from jsonb_array_elements({column}) as elem)::jsonb) as {column},", {
            match <CatWithIdAsNotNullJsonbObjectWithId as postgresql_crud::PostgresqlJsonType>::select_only_updated_ids_query_part(&value.update, &"", &"elem", increment) {
                Ok(value) => value,
                Err(error) => {
                    return Err(error);
                }
            }
        }))
    }
}
#[cfg(test)]
impl postgresql_crud::tests::PostgresqlTypeTestCases for VecOfCatWithIdAsNotNullArrayOfNotNullJsonbObjectWithId {
    type Element = Self;
    fn test_cases(read_only_ids: &<Self::Element as postgresql_crud::PostgresqlType>::ReadOnlyIds) -> std::vec::Vec<std::vec::Vec<<Self::Element as postgresql_crud::PostgresqlType>::ReadInner>> {
        read_only_ids
            .0
            .value
            .iter()
            .map(|element0| {
                let mut acc = vec![];
                for element1 in <postgresql_crud::postgresql_json_type::StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud::tests::PostgresqlJsonTypeTestCases>::test_cases(&element0.0.value.field_444.clone()) {
                    for element2 in element1 {
                        acc.push(vec![CatWithIdAsNotNullJsonbObjectWithIdReadInner {
                            id: Some(postgresql_crud::Value { value: element0.0.value.id.0.value.clone() }),
                            field_444: Some(postgresql_crud::Value { value: element2 }),
                        }]);
                    }
                }
                acc
            })
            .collect()
    }
    fn read_new_or_try_new_unwraped_for_test(value: VecOfCatWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdReadInner) -> <Self::Element as postgresql_crud::PostgresqlType>::Read {
        VecOfCatWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdRead::new({
            let mut acc = vec![];
            for element in value {
                acc.push(<CatWithIdAsNotNullJsonbObjectWithId as postgresql_crud::tests::PostgresqlJsonTypeTestCases>::read_new_or_try_new_unwraped_for_test(element));
            }
            acc
        })
    }
    fn update_new_or_try_new_unwraped_for_test(value: VecOfCatWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdReadInner) -> <Self::Element as postgresql_crud::PostgresqlType>::Update {
        VecOfCatWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdUpdate::try_new(
            vec![],
            CatWithIdAsNotNullJsonbObjectWithIdUpdateHandle::new(
                postgresql_crud::UniqueVec::try_new(
                    value
                        .into_iter()
                        .map(|element| CatWithIdAsNotNullJsonbObjectWithIdUpdateElement {
                            id: postgresql_crud::postgresql_json_type::UuidUuidAsNotNullJsonbStringOrigin::new(element.id.clone().unwrap().value),
                            fields: <CatAsNotNullJsonbObject as postgresql_crud::tests::PostgresqlJsonTypeTestCases>::update_new_or_try_new_unwraped_for_test(CatAsNotNullJsonbObjectReadInner { field_444: element.field_444 }),
                        })
                        .collect(),
                )
                .unwrap(),
            ),
            vec![],
        )
        .unwrap()
    }
    fn read_only_ids_to_option_value_read_inner(value: <Self::Element as postgresql_crud::PostgresqlType>::ReadOnlyIds) -> std::option::Option<postgresql_crud::Value<<Self::Element as postgresql_crud::PostgresqlType>::ReadInner>> {
        todo!()
    }
    fn update_to_read_only_ids(value: &<Self::Element as postgresql_crud::PostgresqlType>::Update) -> <Self::Element as postgresql_crud::PostgresqlType>::ReadOnlyIds {
        VecOfCatWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdReadOnlyIds(postgresql_crud::Value {
            value: value
                .update
                .0
                .to_vec()
                .iter()
                .map(|element| {
                    let mut field_444 = None;
                    for element1 in element.fields.0.to_vec() {
                        match &element1 {
                            CatAsNotNullJsonbObjectUpdateElement::Field444(value) => {
                                field_444 = Some(value.value.clone());
                            }
                        }
                    }
                    CatWithIdAsNotNullJsonbObjectWithIdReadOnlyIds(postgresql_crud::Value {
                        value: CatWithIdAsNotNullJsonbObjectWithIdReadOnlyIdsHandle {
                            id: <postgresql_crud::postgresql_json_type::UuidUuidAsNotNullJsonbString as postgresql_crud::tests::PostgresqlJsonTypeTestCases>::update_to_read_only_ids(&element.id),
                            field_444: <postgresql_crud::postgresql_json_type::StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud::tests::PostgresqlJsonTypeTestCases>::update_to_read_only_ids(&field_444.expect("expect a3ec7cae-94a0-49c1-b5d1-88f7b2a3dd1d")),
                        },
                    })
                })
                .collect(),
        })
    }
}
#[cfg(test)]
impl postgresql_crud::tests::PostgresqlJsonTypeTestCases for VecOfCatWithIdAsNotNullArrayOfNotNullJsonbObjectWithId {
    type Element = Self;
    fn test_cases(read_only_ids: &<Self::Element as postgresql_crud::PostgresqlJsonType>::ReadOnlyIds) -> std::vec::Vec<std::vec::Vec<<Self::Element as postgresql_crud::PostgresqlJsonType>::ReadInner>> {
        read_only_ids
            .0
            .value
            .iter()
            .map(|element0| {
                let mut acc = vec![];
                for element1 in <postgresql_crud::postgresql_json_type::StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud::tests::PostgresqlJsonTypeTestCases>::test_cases(&element0.0.value.field_444.clone()) {
                    for element2 in element1 {
                        acc.push(vec![CatWithIdAsNotNullJsonbObjectWithIdReadInner {
                            id: Some(postgresql_crud::Value { value: element0.0.value.id.0.value.clone() }),
                            field_444: Some(postgresql_crud::Value { value: element2 }),
                        }]);
                    }
                }
                acc
            })
            .collect()
    }
    fn read_new_or_try_new_unwraped_for_test(value: VecOfCatWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdReadInner) -> <Self::Element as postgresql_crud::PostgresqlJsonType>::Read {
        VecOfCatWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdRead::new({
            let mut acc = vec![];
            for element in value {
                acc.push(<CatWithIdAsNotNullJsonbObjectWithId as postgresql_crud::tests::PostgresqlJsonTypeTestCases>::read_new_or_try_new_unwraped_for_test(element));
            }
            acc
        })
    }
    fn update_new_or_try_new_unwraped_for_test(value: VecOfCatWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdReadInner) -> <Self::Element as postgresql_crud::PostgresqlJsonType>::Update {
        VecOfCatWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdUpdate::try_new(
            vec![],
            CatWithIdAsNotNullJsonbObjectWithIdUpdateHandle::new(
                postgresql_crud::UniqueVec::try_new(
                    value
                        .into_iter()
                        .map(|element| CatWithIdAsNotNullJsonbObjectWithIdUpdateElement {
                            id: postgresql_crud::postgresql_json_type::UuidUuidAsNotNullJsonbStringOrigin::new(element.id.clone().unwrap().value),
                            fields: <CatAsNotNullJsonbObject as postgresql_crud::tests::PostgresqlJsonTypeTestCases>::update_new_or_try_new_unwraped_for_test(CatAsNotNullJsonbObjectReadInner { field_444: element.field_444 }),
                        })
                        .collect(),
                )
                .unwrap(),
            ),
            vec![],
        )
        .unwrap()
    }
    fn read_only_ids_to_option_value_read_inner(value: <Self::Element as postgresql_crud::PostgresqlJsonType>::ReadOnlyIds) -> std::option::Option<postgresql_crud::Value<<Self::Element as postgresql_crud::PostgresqlJsonType>::ReadInner>> {
        Some(postgresql_crud::Value {
            value: value.0.value.into_iter().fold(vec![], |mut acc, element| {
                if let Some(value) = <CatWithIdAsNotNullJsonbObjectWithId as postgresql_crud::tests::PostgresqlJsonTypeTestCases>::read_only_ids_to_option_value_read_inner(element) {
                    acc.push(value.value);
                }
                acc
            }),
        })
    }
    fn update_to_read_only_ids(value: &<Self::Element as postgresql_crud::PostgresqlJsonType>::Update) -> <Self::Element as postgresql_crud::PostgresqlJsonType>::ReadOnlyIds {
        VecOfCatWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdReadOnlyIds(postgresql_crud::Value {
            value: value
                .update
                .0
                .to_vec()
                .iter()
                .map(|element| {
                    let mut field_444 = None;
                    for element1 in element.fields.0.to_vec() {
                        match &element1 {
                            CatAsNotNullJsonbObjectUpdateElement::Field444(value) => {
                                field_444 = Some(value.value.clone());
                            }
                        }
                    }
                    CatWithIdAsNotNullJsonbObjectWithIdReadOnlyIds(postgresql_crud::Value {
                        value: CatWithIdAsNotNullJsonbObjectWithIdReadOnlyIdsHandle {
                            id: <postgresql_crud::postgresql_json_type::UuidUuidAsNotNullJsonbString as postgresql_crud::tests::PostgresqlJsonTypeTestCases>::update_to_read_only_ids(&element.id),
                            field_444: <postgresql_crud::postgresql_json_type::StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud::tests::PostgresqlJsonTypeTestCases>::update_to_read_only_ids(&field_444.expect("expect a3ec7cae-94a0-49c1-b5d1-88f7b2a3dd1d")),
                        },
                    })
                })
                .collect(),
        })
    }
}

////////////