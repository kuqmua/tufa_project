#[derive(Debug
    // , postgresql_crud::GeneratePostgresqlCrud
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
// #[postgresql_crud::create_many_additional_route_logic{
//     println!("kekw");
// }]
// #[postgresql_crud::create_one_additional_route_logic{}]
// #[postgresql_crud::read_many_additional_route_logic{}]
// #[postgresql_crud::read_one_additional_route_logic{}]
// #[postgresql_crud::update_many_additional_route_logic{}]
// #[postgresql_crud::update_one_additional_route_logic{}]
// #[postgresql_crud::delete_many_additional_route_logic{}]
// #[postgresql_crud::delete_one_additional_route_logic{}]
// #[postgresql_crud::common_additional_route_logic{
//     // if let Err(error) = postgresql_crud::check_commit::check_commit(
//     //     *app_state.get_enable_api_git_commit_check(),
//     //     &headers,
//     // ) {
//     //     let status_code = postgresql_crud::GetAxumHttpStatusCode::get_axum_http_status_code(&error);
//     //     //todo use reserved work instead of TryCreateManyRouteLogicErrorNamed
//     //     let error = TryCreateManyRouteLogicErrorNamed::CheckCommit {
//     //         check_commit: error,
//     //         code_occurence: error_occurence_lib::code_occurence!(),
//     //     };
//     //     eprintln!("{error}");
//     //     let mut response = axum::response::IntoResponse::into_response(axum::Json(
//     //         TryCreateManyRouteLogicResponseVariants::from(error),
//     //     ));
//     //     *response.status_mut() = status_code;
//     //     return response;
//     // }
// }]
pub struct Example {
    // pub vec: postgresql_crud::postgresql_type::VecOfStdPrimitiveI16AsNotNullArrayOfNotNullInt2,

    // pub column_40d64ae4_a646_4394_bfce_3894bdfced87: postgresql_crud::postgresql_type::StdPrimitiveI16AsNotNullInt2,
    // pub column_ebe9a28a_eef0_4b1d_a706_8a9c363db7ab: postgresql_crud::postgresql_type::OptionStdPrimitiveI16AsNullableInt2,
    // pub column_b225e9b0_80ae_4aea_ab7f_2444d0b3c5fa: postgresql_crud::postgresql_type::StdPrimitiveI32AsNotNullInt4,
    // pub column_fb5fe9b0_1505_4d68_b2e1_2483bc47ab6c: postgresql_crud::postgresql_type::OptionStdPrimitiveI32AsNullableInt4,
    // pub column_579f811f_17de_4989_b1f9_be4b5e060641: postgresql_crud::postgresql_type::StdPrimitiveI64AsNotNullInt8,
    // pub column_55a6dde9_fdae_4025_b9ce_d8fd483a0b86: postgresql_crud::postgresql_type::OptionStdPrimitiveI64AsNullableInt8,
    // pub column_59f8f4f5_e7e6_4ee3_973e_b73f2e415d81: postgresql_crud::postgresql_type::StdPrimitiveF32AsNotNullFloat4,
    // pub column_389cc934_2e11_4e86_8429_ef75934563a7: postgresql_crud::postgresql_type::OptionStdPrimitiveF32AsNullableFloat4,
    // pub column_e8c63c01_7317_4b3a_805f_59df250f3ad0: postgresql_crud::postgresql_type::StdPrimitiveF64AsNotNullFloat8,
    // pub column_0c72cfa5_60ae_4f8f_b517_a68d5fda7eef: postgresql_crud::postgresql_type::OptionStdPrimitiveF64AsNullableFloat8,
    // pub column_8ffe8fce_e270_4b32_8c74_2fad827581b1: postgresql_crud::postgresql_type::SqlxPostgresTypesPgMoneyAsNotNullMoney,
    // pub column_c0956407_a5b3_4b7c_a016_20971edc67d6: postgresql_crud::postgresql_type::OptionSqlxPostgresTypesPgMoneyAsNullableMoney,
    // pub column_2311f761_9b1c_40e8_a9ee_2f990f598c7a: postgresql_crud::postgresql_type::SqlxTypesDecimalAsNotNullNumeric,
    // pub column_521b136c_c5ce_46c8_badb_ce504990ca87: postgresql_crud::postgresql_type::OptionSqlxTypesDecimalAsNullableNumeric,
    // pub column_a8238982_d31d_46ed_aa86_9d2420666769: postgresql_crud::postgresql_type::SqlxTypesBigDecimalAsNotNullNumeric,
    // pub column_77e10de9_11aa_4eaf_aefa_8e97720b615a: postgresql_crud::postgresql_type::OptionSqlxTypesBigDecimalAsNullableNumeric,
    // pub column_39b118e9_44ae_4fa6_8acb_4ea926b2afe4: postgresql_crud::postgresql_type::StdPrimitiveBoolAsNotNullBool,
    // pub column_f3cfeda5_bff1_49b9_adc0_ee99468e116d: postgresql_crud::postgresql_type::OptionStdPrimitiveBoolAsNullableBool,
    // pub column_ab072aa7_6b56_4b1f_9491_7b766f6dbc52: postgresql_crud::postgresql_type::StdStringStringAsNotNullCharN,
    // pub column_b5fde990_7151_4944_95cf_221c59531bbb: postgresql_crud::postgresql_type::OptionStdStringStringAsNullableCharN,
    // pub column_2a975be8_3b64_463c_b449_c7f678aa973f: postgresql_crud::postgresql_type::StdStringStringAsNotNullVarchar,
    // pub column_f94b1781_21c6_414a_a1f0_82a1dc9b03d7: postgresql_crud::postgresql_type::OptionStdStringStringAsNullableVarchar,
    // pub column_7578cdb5_a5b9_484b_ac9f_586fa3bcd2e8: postgresql_crud::postgresql_type::StdStringStringAsNotNullText,
    // pub column_185e3592_8d03_4962_b423_fe22e0b5e28d: postgresql_crud::postgresql_type::OptionStdStringStringAsNullableText,
    // pub column_83c47937_8319_43eb_8c42_72d267380602: postgresql_crud::postgresql_type::StdVecVecStdPrimitiveU8AsNotNullBytea,
    // pub column_6367c2ff_5379_4d67_a49e_1075ff353a25: postgresql_crud::postgresql_type::OptionStdVecVecStdPrimitiveU8AsNullableBytea,
    // pub column_03d92aed_e8d1_434f_a45d_029df9ad1d22: postgresql_crud::postgresql_type::SqlxTypesChronoNaiveTimeAsNotNullTime,
    // pub column_7cd3a1c9_75b1_4227_a16e_8e316311568b: postgresql_crud::postgresql_type::OptionSqlxTypesChronoNaiveTimeAsNullableTime,
    // pub column_0b4ff0bd_ad73_48fd_9ec2_b7bafa908953: postgresql_crud::postgresql_type::SqlxTypesTimeTimeAsNotNullTime,
    // pub column_3c9b4049_bf22_44e2_a9d2_cecf01cc088d: postgresql_crud::postgresql_type::OptionSqlxTypesTimeTimeAsNullableTime,
    // pub column_20555517_a10a_4ddf_a2c3_19898345d836: postgresql_crud::postgresql_type::SqlxPostgresTypesPgIntervalAsNotNullInterval,
    // pub column_e8cd8405_0931_41f2_94cb_9766b405e432: postgresql_crud::postgresql_type::OptionSqlxPostgresTypesPgIntervalAsNullableInterval,
    // pub column_ddb3aed3_bcc2_4c14_b537_4e6249be754c: postgresql_crud::postgresql_type::SqlxTypesTimeDateAsNotNullDate,
    // pub column_ccff97e5_f010_483a_a87d_497f68feb321: postgresql_crud::postgresql_type::OptionSqlxTypesTimeDateAsNullableDate,
    // pub column_e2bd1441_feba_40d8_bf0e_df45f6d99d15: postgresql_crud::postgresql_type::SqlxTypesChronoNaiveDateAsNotNullDate,
    // pub column_752433fa_f3d6_4e0d_81e7_5c5f277da8cd: postgresql_crud::postgresql_type::OptionSqlxTypesChronoNaiveDateAsNullableDate,
    // pub column_e3770284_4ad4_4c87_bbcc_7a7adc738f2f: postgresql_crud::postgresql_type::SqlxTypesChronoNaiveDateTimeAsNotNullTimestamp,
    // pub column_063e648a_c05e_4db6_9278_b3b131e5d9a7: postgresql_crud::postgresql_type::OptionSqlxTypesChronoNaiveDateTimeAsNullableTimestamp,
    // pub column_6325da70_e3e9_4289_a1b0_e39be27a8517: postgresql_crud::postgresql_type::SqlxTypesTimePrimitiveDateTimeAsNotNullTimestamp,
    // pub column_8d1d6adb_45fd_4564_9822_71022d53ec1c: postgresql_crud::postgresql_type::OptionSqlxTypesTimePrimitiveDateTimeAsNullableTimestamp,
    // pub column_e06e3994_bb74_46a7_b914_1205bff2527c: postgresql_crud::postgresql_type::SqlxTypesChronoDateTimeSqlxTypesChronoUtcAsNotNullTimestampTz,
    // pub column_22b1b67c_039a_4020_ba36_63622d02cea6: postgresql_crud::postgresql_type::OptionSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsNullableTimestampTz,
    // pub column_293d0e00_6bd6_4a93_af30_67e9ea1ac7dd: postgresql_crud::postgresql_type::SqlxTypesChronoDateTimeSqlxTypesChronoLocalAsNotNullTimestampTz,
    // pub column_49f974c2_ff30_4ad9_b491_b7e844795808: postgresql_crud::postgresql_type::OptionSqlxTypesChronoDateTimeSqlxTypesChronoLocalAsNullableTimestampTz,
    // pub column_0bb198f5_bf4c_4308_ba13_3f3a18960b25: postgresql_crud::postgresql_type::SqlxTypesUuidUuidAsNotNullUuidInitializedByClient,
    // pub column_0cf8db36_04be_4fc9_bbcc_008321b357c2: postgresql_crud::postgresql_type::OptionSqlxTypesUuidUuidAsNullableUuidInitializedByClient,
    // pub column_1e99c21a_69c5_480b_8282_fe38f7b870d7: postgresql_crud::postgresql_type::SqlxTypesIpnetworkIpNetworkAsNotNullInet,
    // pub column_7c4400a7_c0ce_4470_94f0_bb9dcf3a5786: postgresql_crud::postgresql_type::OptionSqlxTypesIpnetworkIpNetworkAsNullableInet,
    // pub column_be493f9c_ed9d_4471_8382_0eace73cb918: postgresql_crud::postgresql_type::SqlxTypesIpnetworkIpNetworkAsNotNullCidr,
    // pub column_16c4828c_1277_479a_8b32_a556f19947a2: postgresql_crud::postgresql_type::OptionSqlxTypesIpnetworkIpNetworkAsNullableCidr,
    // pub column_dd337b11_4ad8_4c64_b0d6_2e73882ad2d0: postgresql_crud::postgresql_type::SqlxTypesMacAddressMacAddressAsNotNullMacAddr,
    // pub column_d77afa8a_f30f_439a_9339_fdfe730ee62e: postgresql_crud::postgresql_type::OptionSqlxTypesMacAddressMacAddressAsNullableMacAddr,
    // pub column_fc22ca9d_1f57_46b4_922f_5ad659dfe5ff: postgresql_crud::postgresql_type::SqlxTypesBitVecAsNotNullBit,
    // pub column_da0f6a4a_5eb9_46a2_98d1_b7985b537b13: postgresql_crud::postgresql_type::OptionSqlxTypesBitVecAsNullableBit,
    // pub column_e668b6ec_1920_4346_a5a4_1af8e133b09c: postgresql_crud::postgresql_type::SqlxTypesBitVecAsNotNullVarbit,
    // pub column_c4ceaa5a_1c93_4545_921e_9c7e38e65e11: postgresql_crud::postgresql_type::OptionSqlxTypesBitVecAsNullableVarbit,
    // pub column_c203347a_c208_4b2b_8096_364a7fb66685: postgresql_crud::postgresql_type::SqlxPostgresTypesPgRangeStdPrimitiveI32AsNotNullInt4Range,
    // pub column_31f58552_715e_48f4_a7fd_a381af1a813f: postgresql_crud::postgresql_type::OptionSqlxPostgresTypesPgRangeStdPrimitiveI32AsNullableInt4Range,
    // pub column_b471a4b4_a473_4541_ab51_82587e19d464: postgresql_crud::postgresql_type::SqlxPostgresTypesPgRangeStdPrimitiveI64AsNotNullInt8Range,
    // pub column_1aefbea2_7ffa_451c_9966_934f8ebf4f78: postgresql_crud::postgresql_type::OptionSqlxPostgresTypesPgRangeStdPrimitiveI64AsNullableInt8Range,
    // pub column_1ddaf386_9a67_4d7d_b3f8_070c35e249c3: postgresql_crud::postgresql_type::SqlxPostgresTypesPgRangeSqlxTypesDecimalAsNotNullNumRange,
    // pub column_9e4c0068_6974_46de_9dc1_bc95a679d9cb: postgresql_crud::postgresql_type::OptionSqlxPostgresTypesPgRangeSqlxTypesDecimalAsNullableNumRange,
    // pub column_f699bdd4_9380_44d0_b1be_2c792d3ec895: postgresql_crud::postgresql_type::SqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNotNullNumRange,
    // pub column_aa7012af_393b_4736_8b95_725eb6df1ca0: postgresql_crud::postgresql_type::OptionSqlxPostgresTypesPgRangeSqlxTypesBigDecimalAsNullableNumRange,
    // pub column_abb40538_4eea_40ef_87bb_c55623bffd40: postgresql_crud::postgresql_type::SqlxPostgresTypesPgRangeSqlxTypesTimeDateAsNotNullDateRange,
    // pub column_6f8fc5c1_3d38_4ca3_b092_0c7a266aaa8b: postgresql_crud::postgresql_type::OptionSqlxPostgresTypesPgRangeSqlxTypesTimeDateAsNullableDateRange,
    // pub column_5f85250a_f889_4d59_a1fa_12e32542ab04: postgresql_crud::postgresql_type::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsNotNullDateRange,
    // pub column_7051ad39_b56b_432a_9224_d3c46c93d47b: postgresql_crud::postgresql_type::OptionSqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateAsNullableDateRange,
    // pub column_ba0e1014_7604_45c7_a475_b22a968108e8: postgresql_crud::postgresql_type::SqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsNotNullTimestampRange,
    // pub column_4c3cf4d7_121c_4854_aad6_96e5e7820f2e: postgresql_crud::postgresql_type::OptionSqlxPostgresTypesPgRangeSqlxTypesChronoNaiveDateTimeAsNullableTimestampRange,
    // pub column_29ca5058_5ad4_464e_aa1f_9077dbe1eca4: postgresql_crud::postgresql_type::SqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTimeAsNotNullTimestampRange,
    // pub column_12f3648d_b7bc_4f36_a302_291c73783ef2: postgresql_crud::postgresql_type::OptionSqlxPostgresTypesPgRangeSqlxTypesTimePrimitiveDateTimeAsNullableTimestampRange,
    // pub column_a72a15d5_27f0_4a0c_a600_cac0c2d48bd7: postgresql_crud::postgresql_type::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsNotNullTimestampTzRange,
    // pub column_bc3e8a83_9b2b_4709_ae21_f4f79b0dbfd0: postgresql_crud::postgresql_type::OptionSqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoUtcAsNullableTimestampTzRange,
    // pub column_e454c17e_d90e_47fe_a0ef_940d7f5afdae: postgresql_crud::postgresql_type::SqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocalAsNotNullTimestampTzRange,
    // pub column_cc739472_9b4e_432c_a6cc_c803117b0086: postgresql_crud::postgresql_type::OptionSqlxPostgresTypesPgRangeSqlxTypesChronoDateTimeSqlxTypesChronoLocalAsNullableTimestampTzRange,
    // pub column_35ebd1c3_ddb7_44c2_bddf_92ef5296c432: postgresql_crud::postgresql_type::StdPrimitiveI16AsNotNullSmallSerialInitializedByPostgresql,
    // pub column_d712ac90_8af2_4a55_9791_314812be4016: postgresql_crud::postgresql_type::StdPrimitiveI32AsNotNullSerialInitializedByPostgresql,
    // #[generate_postgresql_crud_primary_key]
    pub column_6e88acb0_c566_4fef_8a09_66a41338cf36: postgresql_crud::postgresql_type::StdPrimitiveI64AsNotNullBigSerialInitializedByPostgresql,
    // pub column_8384082c_21de_4a21_95e9_a398644d8ea1: postgresql_crud::postgresql_type::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql,

    // //todo rename like postgresql types: NotNull prefix, not postfix
    pub animal_as_not_null_jsonb_object: AnimalAsNotNullJsonbObject,
    // pub option_animal_as_nullable_jsonb_object: OptionAnimalAsNullableJsonbObject,//todo double nullable problem
    // // pub object_animal_as_jsonb_nullable: OptionObjectAnimalAsJsonbNullable,

    // // pub option_object_animal_as_jsonb_not_null: StdOptionOptionObjectAnimalAsJsonbNotNull,
    // // pub option_object_animal_as_jsonb_nullable: StdOptionOptionObjectAnimalAsJsonbNullable,

    // // pub std_vec_vec_object_with_id_animal_as_jsonb_not_null: StdVecVecObjectWithIdAnimalAsJsonbNotNull,
    // // pub std_vec_vec_object_with_id_animal_as_jsonb_nullable: StdVecVecObjectWithIdAnimalAsJsonbNullable,

    // // pub option_std_vec_vec_object_with_id_animal_as_jsonb_not_null: StdOptionOptionStdVecVecObjectWithIdAnimalAsJsonbNotNull,
    // // pub option_std_vec_vec_object_with_id_animal_as_jsonb_nullable: StdOptionOptionStdVecVecObjectWithIdAnimalAsJsonbNullable,

    // ///////////////////////////
    // // pub vec_std_primitive_i16_as_postgresql_int2_array_not_null: postgresql_crud::postgresql_type::VecStdPrimitiveI16AsInt2ArrayNotNull,
    // //63max
}
//todo enum tree support
//todo generate wrapper type for all possible json type
#[derive(Debug, postgresql_crud::GeneratePostgresqlJsonObjectType)]
#[postgresql_crud::postgresql_json_object_type_pattern{
    {
        "not_null_or_nullable": "NotNull",
        "postgresql_json_object_type_pattern": "Standart",
        "trait_gen": "PostgresqlTypeAndPostgresqlJsonType"
    }
}]
pub struct Animal {
    // pub id: postgresql_crud::postgresql_json_type::Uuid,//todo check length of uuid = 36 // must not be updatable, only readable. postgresql must create it than return object with new ids

    // pub std_vec_vec_uuid_uuid: postgresql_crud::postgresql_json_type::StdVecVecUuidUuid,
    // pub std_vec_vec_std_vec_vec_uuid_uuid: postgresql_crud::postgresql_json_type::StdVecVecStdVecVecUuidUuid,
    //todo check field max length in postgresql
    // pub std_primitive_i8_as_not_null_jsonb_number: postgresql_crud::postgresql_json_type::StdPrimitiveI8AsNotNullJsonbNumber,
    // pub std_primitive_i16: postgresql_crud::postgresql_json_type::StdPrimitiveI16,
    // pub std_primitive_i32: postgresql_crud::postgresql_json_type::StdPrimitiveI32,
    // pub std_primitive_i64: postgresql_crud::postgresql_json_type::StdPrimitiveI64,
    // pub std_primitive_u8: postgresql_crud::postgresql_json_type::StdPrimitiveU8,
    // pub std_primitive_u16: postgresql_crud::postgresql_json_type::StdPrimitiveU16,
    // pub std_primitive_u32: postgresql_crud::postgresql_json_type::StdPrimitiveU32,
    // pub std_primitive_u64: postgresql_crud::postgresql_json_type::StdPrimitiveU64,
    // pub std_primitive_f32: postgresql_crud::postgresql_json_type::StdPrimitiveF32,
    // pub std_primitive_f64: postgresql_crud::postgresql_json_type::StdPrimitiveF64,
    // pub std_primitive_bool: postgresql_crud::postgresql_json_type::StdPrimitiveBool,
    // pub std_string_string: postgresql_crud::postgresql_json_type::StdStringString,
    // pub option_std_primitive_i8_as_nullable_jsonb_number: postgresql_crud::postgresql_json_type::OptionStdPrimitiveI8AsNullableJsonbNumber,
    // pub std_option_option_std_primitive_i16: postgresql_crud::postgresql_json_type::StdOptionOptionStdPrimitiveI16,
    // pub std_option_option_std_primitive_i32: postgresql_crud::postgresql_json_type::StdOptionOptionStdPrimitiveI32,
    // pub std_option_option_std_primitive_i64: postgresql_crud::postgresql_json_type::StdOptionOptionStdPrimitiveI64,
    // pub std_option_option_std_primitive_u8: postgresql_crud::postgresql_json_type::StdOptionOptionStdPrimitiveU8,
    // pub std_option_option_std_primitive_u16: postgresql_crud::postgresql_json_type::StdOptionOptionStdPrimitiveU16,
    // pub std_option_option_std_primitive_u32: postgresql_crud::postgresql_json_type::StdOptionOptionStdPrimitiveU32,
    // pub std_option_option_std_primitive_u64: postgresql_crud::postgresql_json_type::StdOptionOptionStdPrimitiveU64,
    // pub std_option_option_std_primitive_f32: postgresql_crud::postgresql_json_type::StdOptionOptionStdPrimitiveF32,
    // pub std_option_option_std_primitive_f64: postgresql_crud::postgresql_json_type::StdOptionOptionStdPrimitiveF64,
    // pub std_option_option_std_primitive_bool: postgresql_crud::postgresql_json_type::StdOptionOptionStdPrimitiveBool,
    // pub std_option_option_std_string_string: postgresql_crud::postgresql_json_type::StdOptionOptionStdStringString,
    // pub vec_of_std_primitive_i8_as_not_null_array_of_not_null_number: postgresql_crud::postgresql_json_type::VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumber,
    // pub std_vec_vec_std_primitive_i16: postgresql_crud::postgresql_json_type::StdVecVecStdPrimitiveI16,
    // pub std_vec_vec_std_primitive_i32: postgresql_crud::postgresql_json_type::StdVecVecStdPrimitiveI32,
    // pub std_vec_vec_std_primitive_i64: postgresql_crud::postgresql_json_type::StdVecVecStdPrimitiveI64,
    // pub std_vec_vec_std_primitive_u8: postgresql_crud::postgresql_json_type::StdVecVecStdPrimitiveU8,
    // pub std_vec_vec_std_primitive_u16: postgresql_crud::postgresql_json_type::StdVecVecStdPrimitiveU16,
    // pub std_vec_vec_std_primitive_u32: postgresql_crud::postgresql_json_type::StdVecVecStdPrimitiveU32,
    // pub std_vec_vec_std_primitive_u64: postgresql_crud::postgresql_json_type::StdVecVecStdPrimitiveU64,
    // pub std_vec_vec_std_primitive_f32: postgresql_crud::postgresql_json_type::StdVecVecStdPrimitiveF32,
    // pub std_vec_vec_std_primitive_f64: postgresql_crud::postgresql_json_type::StdVecVecStdPrimitiveF64,
    // pub std_vec_vec_std_primitive_bool: postgresql_crud::postgresql_json_type::StdVecVecStdPrimitiveBool,
    // pub std_vec_vec_std_string_string: postgresql_crud::postgresql_json_type::StdVecVecStdStringString,
    // pub option_vec_of_std_primitive_i8_as_nullable_array_of_not_null_number: postgresql_crud::postgresql_json_type::OptionVecOfStdPrimitiveI8AsNullableArrayOfNotNullJsonbNumber,
    // pub std_option_option_std_vec_vec_std_primitive_i16: postgresql_crud::postgresql_json_type::StdOptionOptionStdVecVecStdPrimitiveI16,
    // pub std_option_option_std_vec_vec_std_primitive_i32: postgresql_crud::postgresql_json_type::StdOptionOptionStdVecVecStdPrimitiveI32,
    // pub std_option_option_std_vec_vec_std_primitive_i64: postgresql_crud::postgresql_json_type::StdOptionOptionStdVecVecStdPrimitiveI64,
    // pub std_option_option_std_vec_vec_std_primitive_u8: postgresql_crud::postgresql_json_type::StdOptionOptionStdVecVecStdPrimitiveU8,
    // pub std_option_option_std_vec_vec_std_primitive_u16: postgresql_crud::postgresql_json_type::StdOptionOptionStdVecVecStdPrimitiveU16,
    // pub std_option_option_std_vec_vec_std_primitive_u32: postgresql_crud::postgresql_json_type::StdOptionOptionStdVecVecStdPrimitiveU32,
    // pub std_option_option_std_vec_vec_std_primitive_u64: postgresql_crud::postgresql_json_type::StdOptionOptionStdVecVecStdPrimitiveU64,
    // pub std_option_option_std_vec_vec_std_primitive_f32: postgresql_crud::postgresql_json_type::StdOptionOptionStdVecVecStdPrimitiveF32,
    // pub std_option_option_std_vec_vec_std_primitive_f64: postgresql_crud::postgresql_json_type::StdOptionOptionStdVecVecStdPrimitiveF64,
    // pub std_option_option_std_vec_vec_std_primitive_bool: postgresql_crud::postgresql_json_type::StdOptionOptionStdVecVecStdPrimitiveBool,
    // pub std_option_option_std_vec_vec_std_string_string: postgresql_crud::postgresql_json_type::StdOptionOptionStdVecVecStdStringString,
    // pub vec_of_option_std_primitive_i8_as_not_null_array_of_nullable_jsonb_number: postgresql_crud::postgresql_json_type::VecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableJsonbNumber,
    // pub std_vec_vec_std_option_option_std_primitive_i16: postgresql_crud::postgresql_json_type::StdVecVecStdOptionOptionStdPrimitiveI16,
    // pub std_vec_vec_std_option_option_std_primitive_i32: postgresql_crud::postgresql_json_type::StdVecVecStdOptionOptionStdPrimitiveI32,
    // pub std_vec_vec_std_option_option_std_primitive_i64: postgresql_crud::postgresql_json_type::StdVecVecStdOptionOptionStdPrimitiveI64,
    // pub std_vec_vec_std_option_option_std_primitive_u8: postgresql_crud::postgresql_json_type::StdVecVecStdOptionOptionStdPrimitiveU8,
    // pub std_vec_vec_std_option_option_std_primitive_u16: postgresql_crud::postgresql_json_type::StdVecVecStdOptionOptionStdPrimitiveU16,
    // pub std_vec_vec_std_option_option_std_primitive_u32: postgresql_crud::postgresql_json_type::StdVecVecStdOptionOptionStdPrimitiveU32,
    // pub std_vec_vec_std_option_option_std_primitive_u64: postgresql_crud::postgresql_json_type::StdVecVecStdOptionOptionStdPrimitiveU64,
    // pub std_vec_vec_std_option_option_std_primitive_f32: postgresql_crud::postgresql_json_type::StdVecVecStdOptionOptionStdPrimitiveF32,
    // pub std_vec_vec_std_option_option_std_primitive_f64: postgresql_crud::postgresql_json_type::StdVecVecStdOptionOptionStdPrimitiveF64,
    // pub std_vec_vec_std_option_option_std_primitive_bool: postgresql_crud::postgresql_json_type::StdVecVecStdOptionOptionStdPrimitiveBool,
    // pub std_vec_vec_std_option_option_std_string_string: postgresql_crud::postgresql_json_type::StdVecVecStdOptionOptionStdStringString,
    // pub option_vec_of_option_std_primitive_i8_as_nullable_array_of_nullable_number: postgresql_crud::postgresql_json_type::OptionVecOfOptionStdPrimitiveI8AsNullableArrayOfNullableJsonbNumber,
    // pub std_option_option_std_vec_vec_std_option_option_std_primitive_i16: postgresql_crud::postgresql_json_type::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI16,
    // pub std_option_option_std_vec_vec_std_option_option_std_primitive_i32: postgresql_crud::postgresql_json_type::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI32,
    // pub std_option_option_std_vec_vec_std_option_option_std_primitive_i64: postgresql_crud::postgresql_json_type::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI64,
    // pub std_option_option_std_vec_vec_std_option_option_std_primitive_u8: postgresql_crud::postgresql_json_type::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU8,
    // pub std_option_option_std_vec_vec_std_option_option_std_primitive_u16: postgresql_crud::postgresql_json_type::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU16,
    // pub std_option_option_std_vec_vec_std_option_option_std_primitive_u32: postgresql_crud::postgresql_json_type::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU32,
    // pub std_option_option_std_vec_vec_std_option_option_std_primitive_u64: postgresql_crud::postgresql_json_type::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveU64,
    // pub std_option_option_std_vec_vec_std_option_option_std_primitive_f32: postgresql_crud::postgresql_json_type::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveF32,
    // pub std_option_option_std_vec_vec_std_option_option_std_primitive_f64: postgresql_crud::postgresql_json_type::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveF64,
    // pub std_option_option_std_vec_vec_std_option_option_std_primitive_bool: postgresql_crud::postgresql_json_type::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveBool,
    // pub std_option_option_std_vec_vec_std_option_option_std_string_string: postgresql_crud::postgresql_json_type::StdOptionOptionStdVecVecStdOptionOptionStdStringString,






    // pub column_7bd2f76f_276c_4855_8ee0_4b6ce0ac5015: postgresql_crud::postgresql_json_type::StdPrimitiveI8AsNotNullJsonbNumber,
    // pub column_1495baea_b926_4e20_a223_5b7a1f06c211: postgresql_crud::postgresql_json_type::OptionStdPrimitiveI8AsNullableJsonbNumber,

    // pub column_efd0c71f_6c3c_4409_8419_4f0e1b42e7cf: postgresql_crud::postgresql_json_type::VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumber,
    // pub column_1c62748d_ec9d_4136_8a0d_ae708141edd5: postgresql_crud::postgresql_json_type::VecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableJsonbNumber,

    // pub column_6cf8b1cc_ba6d_4c89_bd4c_91019cce4d67: postgresql_crud::postgresql_json_type::OptionVecOfStdPrimitiveI8AsNullableArrayOfNotNullJsonbNumber,
    // pub column_2d73bb3f_b732_487c_819b_d9cd6dfec894: postgresql_crud::postgresql_json_type::OptionVecOfOptionStdPrimitiveI8AsNullableArrayOfNullableJsonbNumber,

    // pub column_44c2ce88_d547_4049_b726_2db012c19c35: postgresql_crud::postgresql_json_type::VecOfVecOfStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub column_8bba92dc_d58d_480e_9906_9a0f1f89b099: postgresql_crud::postgresql_json_type::VecOfVecOfOptionStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNullableJsonbNumber,

    // pub column_37632743_5dbd_4e31_b01b_d18c0f34c4d9: postgresql_crud::postgresql_json_type::VecOfOptionVecOfStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub column_0513fac3_9889_4b8f_8e3e_f6d43a7395b4: postgresql_crud::postgresql_json_type::VecOfOptionVecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNullableJsonbNumber,

    // pub column_8812301b_9ac9_43bf_b505_07119e24c271: postgresql_crud::postgresql_json_type::OptionVecOfVecOfStdPrimitiveI8AsNullableArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub column_69ef09d3_b32c_4816_b4b5_222d92708776: postgresql_crud::postgresql_json_type::OptionVecOfVecOfOptionStdPrimitiveI8AsNullableArrayOfNotNullArrayOfNullableJsonbNumber,

    // pub column_d98bf62a_1c3c_4897_8c40_a69441d59eac: postgresql_crud::postgresql_json_type::OptionVecOfOptionVecOfStdPrimitiveI8AsNullableArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub column_bb28187d_28b8_420a_910b_a32e6e85ae53: postgresql_crud::postgresql_json_type::OptionVecOfOptionVecOfOptionStdPrimitiveI8AsNullableArrayOfNullableArrayOfNullableJsonbNumber,

    // pub column_77ff96cc_736f_4670_8481_796ce55aabcc: postgresql_crud::postgresql_json_type::VecOfVecOfVecOfStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub column_65901692_843e_42aa_9d5b_e168a0fd840f: postgresql_crud::postgresql_json_type::VecOfVecOfVecOfOptionStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumber,

    // pub column_88863a63_0ff7_4454_9a03_793b56e4e134: postgresql_crud::postgresql_json_type::VecOfVecOfOptionVecOfStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub column_ba1f76ff_128f_4ba5_9d95_06a330cf6af4: postgresql_crud::postgresql_json_type::VecOfVecOfOptionVecOfOptionStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumber,

    // pub column_b4814c28_4646_4b86_b51d_2ba06cfdd3c4: postgresql_crud::postgresql_json_type::VecOfOptionVecOfVecOfStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub column_6c9788b0_c4a0_4c4b_bf45_5a4ccd473ed6: postgresql_crud::postgresql_json_type::VecOfOptionVecOfVecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumber,

    // pub column_ef15f2f6_e098_49e6_82b2_d338bd51c4e4: postgresql_crud::postgresql_json_type::VecOfOptionVecOfOptionVecOfStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub column_f94b5afb_2a63_4308_8ff4_a5ce34baaf4c: postgresql_crud::postgresql_json_type::VecOfOptionVecOfOptionVecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumber,

    // pub column_68fe00c5_c2d4_4aa0_a1b0_c168b0a4cc8a: postgresql_crud::postgresql_json_type::OptionVecOfVecOfVecOfStdPrimitiveI8AsNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub column_7b656ca6_3a32_4105_96a1_d5756973dd0d: postgresql_crud::postgresql_json_type::OptionVecOfVecOfVecOfOptionStdPrimitiveI8AsNullableArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumber,

    // pub column_564cc9e4_3352_46c5_b8b0_a0e3be91dedc: postgresql_crud::postgresql_json_type::OptionVecOfVecOfOptionVecOfStdPrimitiveI8AsNullableArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub column_92988dcf_fde0_46b9_8118_de6525b8d65b: postgresql_crud::postgresql_json_type::OptionVecOfVecOfOptionVecOfOptionStdPrimitiveI8AsNullableArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumber,

    // pub column_b96fb4ef_a529_4a55_98fa_13ab10d3db99: postgresql_crud::postgresql_json_type::OptionVecOfOptionVecOfVecOfStdPrimitiveI8AsNullableArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub column_9d7eb71d_210b_4c02_a549_d938779169cb: postgresql_crud::postgresql_json_type::OptionVecOfOptionVecOfVecOfOptionStdPrimitiveI8AsNullableArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumber,

    // pub column_6394bdd4_46da_4775_a942_b81beee1d819: postgresql_crud::postgresql_json_type::OptionVecOfOptionVecOfOptionVecOfStdPrimitiveI8AsNullableArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub column_7fdf3066_43b0_4133_8dc9_292eb5811e28: postgresql_crud::postgresql_json_type::OptionVecOfOptionVecOfOptionVecOfOptionStdPrimitiveI8AsNullableArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumber,

    // pub column_ec61abdc_69d9_44f1_8259_67274abae152: postgresql_crud::postgresql_json_type::VecOfVecOfVecOfVecOfStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub column_cd4f2bdf_1087_4dc5_bddf_92b6751f743f: postgresql_crud::postgresql_json_type::VecOfVecOfVecOfVecOfOptionStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumber,

    // pub column_9012e020_2f7e_4104_a603_70e46aa65268: postgresql_crud::postgresql_json_type::VecOfVecOfVecOfOptionVecOfStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub column_cea634f9_75c9_4017_9216_0e8ccc48511e: postgresql_crud::postgresql_json_type::VecOfVecOfVecOfOptionVecOfOptionStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumber,

    // pub column_e7410429_f6ed_4366_aba1_453f7d96509c: postgresql_crud::postgresql_json_type::VecOfVecOfOptionVecOfVecOfStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub column_84da70a9_ec0a_4c67_9a57_01dd7198d174: postgresql_crud::postgresql_json_type::VecOfVecOfOptionVecOfVecOfOptionStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumber,

    // pub column_63e4b1cb_5c0d_4a2a_8e6c_f3326f9acff2: postgresql_crud::postgresql_json_type::VecOfVecOfOptionVecOfOptionVecOfStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub column_44d54d68_966e_4a61_b0f3_25f2c9dc98e0: postgresql_crud::postgresql_json_type::VecOfVecOfOptionVecOfOptionVecOfOptionStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumber,

    // pub column_2c8d98ca_7f88_49b5_81cc_22a7d5cade07: postgresql_crud::postgresql_json_type::VecOfOptionVecOfVecOfVecOfStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub column_719c70e2_c904_4b05_872f_4fb0e8f4bf1b: postgresql_crud::postgresql_json_type::VecOfOptionVecOfVecOfVecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumber,

    // pub column_99f6281a_9628_4fe4_aff2_3622a3c14694: postgresql_crud::postgresql_json_type::VecOfOptionVecOfVecOfOptionVecOfStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub column_0071c0e8_f00b_4465_b2c7_323ea8ec0429: postgresql_crud::postgresql_json_type::VecOfOptionVecOfVecOfOptionVecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumber,

    // pub column_2853b725_115a_4e99_990a_68515038aa37: postgresql_crud::postgresql_json_type::VecOfOptionVecOfOptionVecOfVecOfStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub column_28e33a5d_b113_4988_8594_e3ed5ab769dc: postgresql_crud::postgresql_json_type::VecOfOptionVecOfOptionVecOfVecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumber,

    // pub column_5fe3e767_4b10_45a2_9feb_5499ebe66a36: postgresql_crud::postgresql_json_type::VecOfOptionVecOfOptionVecOfOptionVecOfStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub column_f3cb923e_8bb4_4bd6_b45d_5005b2ac771b: postgresql_crud::postgresql_json_type::VecOfOptionVecOfOptionVecOfOptionVecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumber,

    // pub column_48906015_ce5b_424d_bfe2_bf811a9be589: postgresql_crud::postgresql_json_type::OptionVecOfVecOfVecOfVecOfStdPrimitiveI8AsNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub column_891ed950_8152_43b4_9664_13d44d83f6f9: postgresql_crud::postgresql_json_type::OptionVecOfVecOfVecOfVecOfOptionStdPrimitiveI8AsNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumber,

    // pub column_1a9fcb5b_39a3_4937_83a0_5c7b413d40c9: postgresql_crud::postgresql_json_type::OptionVecOfVecOfVecOfOptionVecOfStdPrimitiveI8AsNullableArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub column_ad00bd6c_1760_44b4_b11b_68164c0c9fad: postgresql_crud::postgresql_json_type::OptionVecOfVecOfVecOfOptionVecOfOptionStdPrimitiveI8AsNullableArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumber,

    // pub column_b97eddb4_5c06_452c_96d8_61a58766fcc9: postgresql_crud::postgresql_json_type::OptionVecOfVecOfOptionVecOfVecOfStdPrimitiveI8AsNullableArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub column_9d06f032_3347_4fbe_9bf9_0a9720659191: postgresql_crud::postgresql_json_type::OptionVecOfVecOfOptionVecOfVecOfOptionStdPrimitiveI8AsNullableArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumber,

    // pub column_562313b6_c069_498a_85c8_75f1921d09ca: postgresql_crud::postgresql_json_type::OptionVecOfVecOfOptionVecOfOptionVecOfStdPrimitiveI8AsNullableArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub column_76172c05_0dc2_4951_9a8e_711e05a2bc80: postgresql_crud::postgresql_json_type::OptionVecOfVecOfOptionVecOfOptionVecOfOptionStdPrimitiveI8AsNullableArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumber,

    // pub column_c4689acf_cd75_4390_9864_3fd6ff5de2c6: postgresql_crud::postgresql_json_type::OptionVecOfOptionVecOfVecOfVecOfStdPrimitiveI8AsNullableArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub column_83b934d7_af94_4697_9a92_11da92bde211: postgresql_crud::postgresql_json_type::OptionVecOfOptionVecOfVecOfVecOfOptionStdPrimitiveI8AsNullableArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumber,

    // pub column_ce0a5a10_1426_4dcb_a52b_4f549d24f26a: postgresql_crud::postgresql_json_type::OptionVecOfOptionVecOfVecOfOptionVecOfStdPrimitiveI8AsNullableArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub column_bd461efa_aaa3_4cac_b4f0_862dbb6ee20d: postgresql_crud::postgresql_json_type::OptionVecOfOptionVecOfVecOfOptionVecOfOptionStdPrimitiveI8AsNullableArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumber,

    // pub column_36323f04_d859_45c4_9284_5efcad9033c4: postgresql_crud::postgresql_json_type::OptionVecOfOptionVecOfOptionVecOfVecOfStdPrimitiveI8AsNullableArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub column_37266bc7_1cec_420b_92b8_36d24ab92f43: postgresql_crud::postgresql_json_type::OptionVecOfOptionVecOfOptionVecOfVecOfOptionStdPrimitiveI8AsNullableArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumber,

    // pub column_bc808dfd_4e49_4986_a397_89661b6b07da: postgresql_crud::postgresql_json_type::OptionVecOfOptionVecOfOptionVecOfOptionVecOfStdPrimitiveI8AsNullableArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub column_b41f35cd_638e_4e46_999d_b760403ee543: postgresql_crud::postgresql_json_type::OptionVecOfOptionVecOfOptionVecOfOptionVecOfOptionStdPrimitiveI8AsNullableArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumber,

    // pub object_doggie: ObjectDoggie,
    // pub std_option_option_object: StdOptionOptionObjectDoggie,
    // pub std_vec_vec_object: StdVecVecObjectWithIdDoggie,
    // pub std_option_option_std_vec_vec_object: StdOptionOptionStdVecVecObjectWithIdDoggie,

    // pub std_vec_vec_object_with_id: StdVecVecObjectWithIdDoggie,
    // pub std_option_option_std_vec_vec_object_with_id: StdOptionOptionStdVecVecObjectWithIdDoggie
    pub doggie_as_not_null_jsonb_object: DoggieAsNotNullJsonbObject,
    // pub option_doggie_as_nullable_jsonb_object: OptionDoggieAsNullableJsonbObject,
    // pub vec_of_doggie_with_id_as_not_null_array_of_not_null_jsonb_object_with_id: VecOfDoggieWithIdAsNotNullArrayOfNotNullJsonbObjectWithId,
    // pub option_vec_of_doggie_with_id_as_nullable_array_of_not_null_jsonb_object_with_id: OptionVecOfDoggieWithIdAsNullableArrayOfNotNullJsonbObjectWithId,
}
///////////////////////////////////////////////////
#[cfg(test)]
mod tests {
    #[test]
    fn test_size_of() {
        assert_eq!(std::mem::size_of::<crate::repositories_types::server::routes::api::example::Example>(), 0);
    }
}
//////////////////
#[derive(Debug
    , postgresql_crud::GeneratePostgresqlJsonObjectType
)]
#[postgresql_crud::postgresql_json_object_type_pattern{
    {
        "not_null_or_nullable": "Nullable",
        "postgresql_json_object_type_pattern": "Array",
        "trait_gen": "PostgresqlJsonType"
    }
}]
pub struct Doggie {
    pub column_113f3662_35a2_4a7a_9326_03bbd441815f: postgresql_crud::postgresql_json_type::StdPrimitiveI8AsNotNullJsonbNumber,
    pub column_1761f64d_b930_446b_8422_e4fa6faf8872: postgresql_crud::postgresql_json_type::OptionStdPrimitiveI8AsNullableJsonbNumber,

    pub column_0f498e79_5440_4c9d_90cf_c32f9b7d4005: postgresql_crud::postgresql_json_type::VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumber,
    pub column_17c31340_b040_4ead_8dcf_476451486b4a: postgresql_crud::postgresql_json_type::VecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableJsonbNumber,
    pub column_95fb90c0_96bc_4a73_9e4c_2537bcfe92b6: postgresql_crud::postgresql_json_type::OptionVecOfStdPrimitiveI8AsNullableArrayOfNotNullJsonbNumber,
    pub column_77f1b2d9_19ea_4f29_8252_b1658a701077: postgresql_crud::postgresql_json_type::OptionVecOfOptionStdPrimitiveI8AsNullableArrayOfNullableJsonbNumber,

    // pub column_2ba10b63_f1a5_42f2_93d4_976254a4cfe2: postgresql_crud::postgresql_json_type::VecOfVecOfStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub column_daced04c_dc1b_44b6_a575_761f72133c42: postgresql_crud::postgresql_json_type::VecOfVecOfOptionStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub column_be269b47_360c_476b_8a69_af528dd5b511: postgresql_crud::postgresql_json_type::VecOfOptionVecOfStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub column_f4eb68fc_93da_488e_b516_58d0a6f80b05: postgresql_crud::postgresql_json_type::VecOfOptionVecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNullableJsonbNumber,
    // pub column_f4fe555a_0857_4c1d_ad90_3b2a8d105405: postgresql_crud::postgresql_json_type::OptionVecOfVecOfStdPrimitiveI8AsNullableArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub column_5c5383c1_1e87_465c_979c_d1c5fe549f1c: postgresql_crud::postgresql_json_type::OptionVecOfVecOfOptionStdPrimitiveI8AsNullableArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub column_5c2af801_2b80_47d1_a3f6_ba59c48ecd1d: postgresql_crud::postgresql_json_type::OptionVecOfOptionVecOfStdPrimitiveI8AsNullableArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub column_1b82fc80_8f0d_403c_bb2c_d8ebdd61e872: postgresql_crud::postgresql_json_type::OptionVecOfOptionVecOfOptionStdPrimitiveI8AsNullableArrayOfNullableArrayOfNullableJsonbNumber,
    
    // pub column_69fd7fb1_30db_466d_8f71_eaa255b0b941: postgresql_crud::postgresql_json_type::VecOfVecOfVecOfStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub column_0fef1adb_3945_4910_96cc_f055f1a695de: postgresql_crud::postgresql_json_type::VecOfVecOfVecOfOptionStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub column_24f05254_13a0_489e_a918_6704b76471f4: postgresql_crud::postgresql_json_type::VecOfVecOfOptionVecOfStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub column_bb610be4_b1e6_41cb_9a30_989255d62320: postgresql_crud::postgresql_json_type::VecOfVecOfOptionVecOfOptionStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumber,
    // pub column_e5e1bd2c_00fc_4114_bf8c_3fb11da65320: postgresql_crud::postgresql_json_type::VecOfOptionVecOfVecOfStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub column_2ec75da3_26d9_46b7_a7c6_c5cd7a8465e0: postgresql_crud::postgresql_json_type::VecOfOptionVecOfVecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub column_d92682f6_778e_426c_b8c1_f4a4a83c7f14: postgresql_crud::postgresql_json_type::VecOfOptionVecOfOptionVecOfStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub column_1ee1ded8_c2c2_4d16_bafa_31dced73c970: postgresql_crud::postgresql_json_type::VecOfOptionVecOfOptionVecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumber,
    // pub column_09eda034_4e03_4a63_9c5c_341e887338ff: postgresql_crud::postgresql_json_type::OptionVecOfVecOfVecOfStdPrimitiveI8AsNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub column_60853f6b_05d4_4a69_a3cd_a77371bdae98: postgresql_crud::postgresql_json_type::OptionVecOfVecOfVecOfOptionStdPrimitiveI8AsNullableArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub column_12dc13d8_e63a_4ccb_a683_efab99956ca1: postgresql_crud::postgresql_json_type::OptionVecOfVecOfOptionVecOfStdPrimitiveI8AsNullableArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub column_f76b0f25_b6eb_46ae_858f_6c8a97a145c8: postgresql_crud::postgresql_json_type::OptionVecOfVecOfOptionVecOfOptionStdPrimitiveI8AsNullableArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumber,
    // pub column_4ed85585_ac0b_464a_9033_d39551601320: postgresql_crud::postgresql_json_type::OptionVecOfOptionVecOfVecOfStdPrimitiveI8AsNullableArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub column_ab9fb5ec_488e_4b82_8b7f_2e34c3d0b79a: postgresql_crud::postgresql_json_type::OptionVecOfOptionVecOfVecOfOptionStdPrimitiveI8AsNullableArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub column_f74be829_815a_47c8_96e1_dd98f0061ae9: postgresql_crud::postgresql_json_type::OptionVecOfOptionVecOfOptionVecOfStdPrimitiveI8AsNullableArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub column_52a69e2b_5c7b_4e8c_9de6_0b57269a2658: postgresql_crud::postgresql_json_type::OptionVecOfOptionVecOfOptionVecOfOptionStdPrimitiveI8AsNullableArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumber,
    
    // pub column_32fc16eb_2641_4d65_acec_0c7addbfe338: postgresql_crud::postgresql_json_type::VecOfVecOfVecOfVecOfStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub column_28794bc4_4a2b_4239_b010_6a6736bb0e6c: postgresql_crud::postgresql_json_type::VecOfVecOfVecOfVecOfOptionStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub column_b3126da0_b74b_4d59_bf68_a1086dd1349c: postgresql_crud::postgresql_json_type::VecOfVecOfVecOfOptionVecOfStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub column_c0317d97_87a3_4d0f_8ae3_badc09bdaf6c: postgresql_crud::postgresql_json_type::VecOfVecOfVecOfOptionVecOfOptionStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumber,
    // pub column_941b3317_2463_4bdb_9e54_f0ba197f04e7: postgresql_crud::postgresql_json_type::VecOfVecOfOptionVecOfVecOfStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub column_a9306dea_647e_4445_a691_3abd7dd21156: postgresql_crud::postgresql_json_type::VecOfVecOfOptionVecOfVecOfOptionStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub column_db8ad189_035c_4384_a00b_af8a48c27bd9: postgresql_crud::postgresql_json_type::VecOfVecOfOptionVecOfOptionVecOfStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub column_3c97d254_319a_44e7_b14e_9ca736ab1dfb: postgresql_crud::postgresql_json_type::VecOfVecOfOptionVecOfOptionVecOfOptionStdPrimitiveI8AsNotNullArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumber,
    // pub column_488c16c1_5cbd_43f5_9162_e9c9c38d13ab: postgresql_crud::postgresql_json_type::VecOfOptionVecOfVecOfVecOfStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub column_3014263a_b089_4881_8570_a67d990d4a04: postgresql_crud::postgresql_json_type::VecOfOptionVecOfVecOfVecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub column_37638447_4b88_4f42_a894_850b59010f6e: postgresql_crud::postgresql_json_type::VecOfOptionVecOfVecOfOptionVecOfStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub column_7886edd0_bbd7_4983_914a_3d76ee8ed4c4: postgresql_crud::postgresql_json_type::VecOfOptionVecOfVecOfOptionVecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumber,
    // pub column_fd8d1866_f22c_4c5f_99fc_b57606a3eed7: postgresql_crud::postgresql_json_type::VecOfOptionVecOfOptionVecOfVecOfStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub column_f1ff7f9b_bbc8_40b0_8300_7a3ad6ed3d6e: postgresql_crud::postgresql_json_type::VecOfOptionVecOfOptionVecOfVecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub column_51a164f5_8416_4ce3_85af_84f7be08e0a4: postgresql_crud::postgresql_json_type::VecOfOptionVecOfOptionVecOfOptionVecOfStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub column_30c977b6_9bcb_45e8_8106_b30762471d47: postgresql_crud::postgresql_json_type::VecOfOptionVecOfOptionVecOfOptionVecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumber,
    // pub column_4974d9e8_942a_4b97_a966_d28756982032: postgresql_crud::postgresql_json_type::OptionVecOfVecOfVecOfVecOfStdPrimitiveI8AsNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub column_b7705cc7_42a1_4616_84f5_716f8037c800: postgresql_crud::postgresql_json_type::OptionVecOfVecOfVecOfVecOfOptionStdPrimitiveI8AsNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub column_4b0c5238_f2e7_43b6_a26a_0f3d3aab8245: postgresql_crud::postgresql_json_type::OptionVecOfVecOfVecOfOptionVecOfStdPrimitiveI8AsNullableArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub column_8bf01736_94f4_49dd_81f6_a7fac58f6a43: postgresql_crud::postgresql_json_type::OptionVecOfVecOfVecOfOptionVecOfOptionStdPrimitiveI8AsNullableArrayOfNotNullArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumber,
    // pub column_443e87d1_fa04_4525_a525_cfab8585c80e: postgresql_crud::postgresql_json_type::OptionVecOfVecOfOptionVecOfVecOfStdPrimitiveI8AsNullableArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub column_a48b63c1_063a_414b_9e00_0a7ce218cd73: postgresql_crud::postgresql_json_type::OptionVecOfVecOfOptionVecOfVecOfOptionStdPrimitiveI8AsNullableArrayOfNotNullArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub column_b1e6d9bb_5d19_4f30_a361_c695c825b712: postgresql_crud::postgresql_json_type::OptionVecOfVecOfOptionVecOfOptionVecOfStdPrimitiveI8AsNullableArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub column_805b53d0_6c4a_42d0_9042_cc07f800cff0: postgresql_crud::postgresql_json_type::OptionVecOfVecOfOptionVecOfOptionVecOfOptionStdPrimitiveI8AsNullableArrayOfNotNullArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumber,
    // pub column_f1f19fa5_df71_40fc_83e3_13f08f2242c4: postgresql_crud::postgresql_json_type::OptionVecOfOptionVecOfVecOfVecOfStdPrimitiveI8AsNullableArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub column_645037eb_38a0_4fea_97a6_1b376dd34804: postgresql_crud::postgresql_json_type::OptionVecOfOptionVecOfVecOfVecOfOptionStdPrimitiveI8AsNullableArrayOfNullableArrayOfNotNullArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub column_4e4c0b55_7813_4e5a_b46c_af3b2c2632ae: postgresql_crud::postgresql_json_type::OptionVecOfOptionVecOfVecOfOptionVecOfStdPrimitiveI8AsNullableArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub column_8848ad3b_9bab_4919_b54e_e1e91b7da558: postgresql_crud::postgresql_json_type::OptionVecOfOptionVecOfVecOfOptionVecOfOptionStdPrimitiveI8AsNullableArrayOfNullableArrayOfNotNullArrayOfNullableArrayOfNullableJsonbNumber,
    // pub column_1a1e208f_8929_4466_9879_18f4cf2018f7: postgresql_crud::postgresql_json_type::OptionVecOfOptionVecOfOptionVecOfVecOfStdPrimitiveI8AsNullableArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNotNullJsonbNumber,
    // pub column_043a589a_1100_49d5_8486_4b75c5b5a02e: postgresql_crud::postgresql_json_type::OptionVecOfOptionVecOfOptionVecOfVecOfOptionStdPrimitiveI8AsNullableArrayOfNullableArrayOfNullableArrayOfNotNullArrayOfNullableJsonbNumber,
    // pub column_e76360cf_7598_4d59_a1a0_11400669934f: postgresql_crud::postgresql_json_type::OptionVecOfOptionVecOfOptionVecOfOptionVecOfStdPrimitiveI8AsNullableArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNotNullJsonbNumber,
    // pub column_1565db46_4881_405f_8950_773a346a6848: postgresql_crud::postgresql_json_type::OptionVecOfOptionVecOfOptionVecOfOptionVecOfOptionStdPrimitiveI8AsNullableArrayOfNullableArrayOfNullableArrayOfNullableArrayOfNullableJsonbNumber,
}
////////////////////////////
impl Example {
    pub fn table_name() -> &'static str {
        "example"
    }
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize)]
pub struct ExampleOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column_6e88acb0_c566_4fef_8a09_66a41338cf36: std::option::Option<postgresql_crud::Value<<postgresql_crud::postgresql_type::StdPrimitiveI64AsNotNullBigSerialInitializedByPostgresql as postgresql_crud::PostgresqlType>::Read>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub animal_as_not_null_jsonb_object: std::option::Option<postgresql_crud::Value<<AnimalAsNotNullJsonbObject as postgresql_crud::PostgresqlType>::Read>>,
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize, PartialEq, Clone)]
pub enum ExampleSelect {
    #[serde(rename(serialize = "column_6e88acb0_c566_4fef_8a09_66a41338cf36", deserialize = "column_6e88acb0_c566_4fef_8a09_66a41338cf36"))]
    Column6E88Acb0C5664Fef8A0966A41338Cf36(<postgresql_crud::postgresql_type::StdPrimitiveI64AsNotNullBigSerialInitializedByPostgresql as postgresql_crud::PostgresqlType>::Select),
    #[serde(rename(serialize = "animal_as_not_null_jsonb_object", deserialize = "animal_as_not_null_jsonb_object"))]
    AnimalAsNotNullJsonbObject(<AnimalAsNotNullJsonbObject as postgresql_crud::PostgresqlType>::Select),
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
        vec![
            ExampleSelect::Column6E88Acb0C5664Fef8A0966A41338Cf36(postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            ExampleSelect::AnimalAsNotNullJsonbObject(postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
        ]
    }
}
impl ExampleSelect {
    fn pick_select(&self) -> std::string::String {
        match &self {
            Self::Column6E88Acb0C5664Fef8A0966A41338Cf36(_) => "column_6e88acb0_c566_4fef_8a09_66a41338cf36".to_string(),
            Self::AnimalAsNotNullJsonbObject(_) => "animal_as_not_null_jsonb_object".to_string(),
        }
    }
}
pub const ALLOW_METHODS: [http::Method; 4] = [http::Method::GET, http::Method::POST, http::Method::PATCH, http::Method::DELETE];
#[derive(Debug, Clone, Copy)]
pub struct ExampleColumnReadPermission {
    column_6e88acb0_c566_4fef_8a09_66a41338cf36: std::primitive::bool,
    animal_as_not_null_jsonb_object: std::primitive::bool,
}
pub async fn create_table_if_not_exists(pool: &sqlx::Pool<sqlx::Postgres>) {
    let create_extension_if_not_exists_pg_jsonschema_query_stringified = "create extension if not exists pg_jsonschema";
    println!("{create_extension_if_not_exists_pg_jsonschema_query_stringified}");
    let _ = sqlx::query(create_extension_if_not_exists_pg_jsonschema_query_stringified).execute(pool).await.unwrap();
    let create_extension_if_not_exists_uuid_ossp_query_stringified = "create extension if not exists \"uuid-ossp\"";
    println!("{create_extension_if_not_exists_uuid_ossp_query_stringified}");
    let _ = sqlx::query(create_extension_if_not_exists_uuid_ossp_query_stringified).execute(pool).await.unwrap();
    let create_table_if_not_exists_query_stringified = format!(
        "create table if not exists example ({},{})",
        <postgresql_crud::postgresql_type::StdPrimitiveI64AsNotNullBigSerialInitializedByPostgresql as postgresql_crud::PostgresqlType>::TableTypeDeclaration::create_table_column_query_part(&"column_6e88acb0_c566_4fef_8a09_66a41338cf36", true),
        <AnimalAsNotNullJsonbObject as postgresql_crud::PostgresqlType>::TableTypeDeclaration::create_table_column_query_part(&"animal_as_not_null_jsonb_object", false)
    );
    println!("{create_table_if_not_exists_query_stringified}");
    let _ = sqlx::query(&create_table_if_not_exists_query_stringified).execute(pool).await.unwrap();
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema)]
pub struct CreateManyPayloadElement {
    pub column_6e88acb0_c566_4fef_8a09_66a41338cf36: <postgresql_crud::postgresql_type::StdPrimitiveI64AsNotNullBigSerialInitializedByPostgresql as postgresql_crud::PostgresqlType>::Create,
    pub animal_as_not_null_jsonb_object: <AnimalAsNotNullJsonbObject as postgresql_crud::PostgresqlType>::Create,
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema)]
pub struct CreateManyPayload(pub std::vec::Vec<CreateManyPayloadElement>);
#[derive(Debug)]
pub struct CreateManyParameters {
    pub payload: CreateManyPayload,
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize)]
pub enum TryCreateManyRouteLogicResponseVariants {
    Desirable(std::vec::Vec<<postgresql_crud::postgresql_type::StdPrimitiveI64AsNotNullBigSerialInitializedByPostgresql as postgresql_crud::PostgresqlTypePrimaryKey>::PrimaryKey>),
    CheckBodySize {
        check_body_size: postgresql_crud::check_body_size::CheckBodySizeErrorNamedWithSerializeDeserialize,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Postgresql {
        postgresql: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    SerdeJson {
        serde_json: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    CheckCommit {
        check_commit: postgresql_crud::check_commit::CheckCommitErrorNamedWithSerializeDeserialize,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    CheckedAdd {
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    RowAndRollback {
        row: std::string::String,
        rollback: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    UnexpectedRowsLength {
        expected_length: std::string::String,
        got_length: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    UnexpectedRowsLengthAndRollback {
        expected_length: std::string::String,
        got_length: std::string::String,
        rollback: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
impl std::convert::From<TryCreateManyRouteLogicErrorNamed> for TryCreateManyRouteLogicResponseVariants {
    fn from(value: TryCreateManyRouteLogicErrorNamed) -> Self {
        match value.into_serialize_deserialize_version() {
            TryCreateManyRouteLogicErrorNamedWithSerializeDeserialize::CheckBodySize { check_body_size, code_occurence } => Self::CheckBodySize { check_body_size, code_occurence },
            TryCreateManyRouteLogicErrorNamedWithSerializeDeserialize::Postgresql { postgresql, code_occurence } => Self::Postgresql { postgresql, code_occurence },
            TryCreateManyRouteLogicErrorNamedWithSerializeDeserialize::SerdeJson { serde_json, code_occurence } => Self::SerdeJson { serde_json, code_occurence },
            TryCreateManyRouteLogicErrorNamedWithSerializeDeserialize::CheckCommit { check_commit, code_occurence } => Self::CheckCommit { check_commit, code_occurence },
            TryCreateManyRouteLogicErrorNamedWithSerializeDeserialize::CheckedAdd { code_occurence } => Self::CheckedAdd { code_occurence },
            TryCreateManyRouteLogicErrorNamedWithSerializeDeserialize::RowAndRollback { row, rollback, code_occurence } => Self::RowAndRollback { row, rollback, code_occurence },
            TryCreateManyRouteLogicErrorNamedWithSerializeDeserialize::UnexpectedRowsLength { expected_length, got_length, code_occurence } => Self::UnexpectedRowsLength { expected_length, got_length, code_occurence },
            TryCreateManyRouteLogicErrorNamedWithSerializeDeserialize::UnexpectedRowsLengthAndRollback { expected_length, got_length, rollback, code_occurence } => Self::UnexpectedRowsLengthAndRollback { expected_length, got_length, rollback, code_occurence },
        }
    }
}
#[derive(Debug, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum TryCreateManyRouteLogicErrorNamed {
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
    CheckCommit {
        #[eo_error_occurence]
        check_commit: postgresql_crud::check_commit::CheckCommitErrorNamed,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    CheckedAdd {
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    RowAndRollback {
        #[eo_to_std_string_string]
        row: sqlx::Error,
        #[eo_to_std_string_string]
        rollback: sqlx::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    UnexpectedRowsLength {
        #[eo_to_std_string_string]
        expected_length: std::primitive::usize,
        #[eo_to_std_string_string]
        got_length: std::primitive::usize,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    UnexpectedRowsLengthAndRollback {
        #[eo_to_std_string_string]
        expected_length: std::primitive::usize,
        #[eo_to_std_string_string]
        got_length: std::primitive::usize,
        #[eo_to_std_string_string]
        rollback: sqlx::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
pub async fn try_create_many_route_logic(app_state: axum::extract::State<crate::repositories_types::server::routes::app_state::DynArcCombinationOfAppStateLogicTraits>, request: axum::extract::Request) -> axum::response::Response {
    let (parts, body) = request.into_parts();
    let headers = parts.headers;
    let body_bytes = match postgresql_crud::check_body_size::check_body_size(body, *app_state.get_maximum_size_of_http_body_in_bytes()).await {
        Ok(value) => value,
        Err(error_0) => {
            let error = TryCreateManyRouteLogicErrorNamed::CheckBodySize {
                check_body_size: error_0,
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_owned(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                        line: 2290,
                        column: 264,
                    }),
                ),
            };
            eprintln!("{error}");
            let mut response = axum::response::IntoResponse::into_response(axum::Json(TryCreateManyRouteLogicResponseVariants::from(error)));
            *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
            return response;
        }
    };
    println!("kekw");
    let parameters = CreateManyParameters {
        payload: match serde_json::from_slice::<CreateManyPayload>(&body_bytes) {
            Ok(value) => {
                let value = CreateManyPayload::from(value);
                value
            }
            Err(error_0) => {
                let error = TryCreateManyRouteLogicErrorNamed::SerdeJson {
                    serde_json: error_0,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                            line: 2362,
                            column: 249,
                        }),
                    ),
                };
                eprintln!("{error}");
                let mut response = axum::response::IntoResponse::into_response(axum::Json(TryCreateManyRouteLogicResponseVariants::from(error)));
                *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
                return response;
            }
        },
    };
    let error_0 = parameters.payload.0.len();
    let query_string = {
        let mut increment: std::primitive::u64 = 0;
        let mut values = std::string::String::default();
        for element in &parameters.payload.0 {
            let mut acc = std::string::String::default();
            match <postgresql_crud::postgresql_type::StdPrimitiveI64AsNotNullBigSerialInitializedByPostgresql as postgresql_crud::PostgresqlType>::create_query_part(&element.column_6e88acb0_c566_4fef_8a09_66a41338cf36, &mut increment) {
                Ok(value) => {
                    acc.push_str(&format!("{value},"));
                }
                Err(error_0) => {
                    let error = TryCreateManyRouteLogicErrorNamed::CheckedAdd {
                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                            file!().to_owned(),
                            line!(),
                            column!(),
                            Some(error_occurence_lib::code_occurence::MacroOccurence {
                                file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                                line: 2827,
                                column: 260,
                            }),
                        ),
                    };
                    eprintln!("{error}");
                    let mut response = axum::response::IntoResponse::into_response(axum::Json(TryCreateManyRouteLogicResponseVariants::from(error)));
                    *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
                    return response;
                }
            }
            match <AnimalAsNotNullJsonbObject as postgresql_crud::PostgresqlType>::create_query_part(&element.animal_as_not_null_jsonb_object, &mut increment) {
                Ok(value) => {
                    acc.push_str(&format!("{value},"));
                }
                Err(error_0) => {
                    let error = TryCreateManyRouteLogicErrorNamed::CheckedAdd {
                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                            file!().to_owned(),
                            line!(),
                            column!(),
                            Some(error_occurence_lib::code_occurence::MacroOccurence {
                                file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                                line: 2827,
                                column: 260,
                            }),
                        ),
                    };
                    eprintln!("{error}");
                    let mut response = axum::response::IntoResponse::into_response(axum::Json(TryCreateManyRouteLogicResponseVariants::from(error)));
                    *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
                    return response;
                }
            }
            let _: Option<char> = acc.pop();
            values.push_str(&format!("({acc}),"));
        }
        let _: Option<char> = values.pop();
        format!("insert into example (column_6e88acb0_c566_4fef_8a09_66a41338cf36,animal_as_not_null_jsonb_object) values {values}  returning column_6e88acb0_c566_4fef_8a09_66a41338cf36")
    };
    println!("{}", query_string);
    let binded_query = {
        let mut query = sqlx::query::<sqlx::Postgres>(&query_string);
        for element in parameters.payload.0 {
            query = <postgresql_crud::postgresql_type::StdPrimitiveI64AsNotNullBigSerialInitializedByPostgresql as postgresql_crud::PostgresqlType>::create_query_bind(element.column_6e88acb0_c566_4fef_8a09_66a41338cf36, query);
            query = <AnimalAsNotNullJsonbObject as postgresql_crud::PostgresqlType>::create_query_bind(element.animal_as_not_null_jsonb_object, query);
        }
        query
    };
    let mut pool_connection = match app_state.get_postgres_pool().acquire().await {
        Ok(value) => value,
        Err(error_0) => {
            let error = TryCreateManyRouteLogicErrorNamed::Postgresql {
                postgresql: error_0,
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_owned(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                        line: 2312,
                        column: 253,
                    }),
                ),
            };
            eprintln!("{error}");
            let mut response = axum::response::IntoResponse::into_response(axum::Json(TryCreateManyRouteLogicResponseVariants::from(error)));
            *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
            return response;
        }
    };
    let executor = match sqlx::Acquire::acquire(&mut pool_connection).await {
        Ok(value) => value,
        Err(error_0) => {
            let error = TryCreateManyRouteLogicErrorNamed::Postgresql {
                postgresql: error_0,
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_owned(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                        line: 2312,
                        column: 253,
                    }),
                ),
            };
            eprintln!("{error}");
            let mut response = axum::response::IntoResponse::into_response(axum::Json(TryCreateManyRouteLogicResponseVariants::from(error)));
            *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
            return response;
        }
    };
    let value = {
        let mut executor = match sqlx::Acquire::begin(executor).await {
            Ok(value) => value,
            Err(error_0) => {
                let error = TryCreateManyRouteLogicErrorNamed::Postgresql {
                    postgresql: error_0,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                            line: 2059,
                            column: 245,
                        }),
                    ),
                };
                eprintln!("{error}");
                let mut response = axum::response::IntoResponse::into_response(axum::Json(TryCreateManyRouteLogicResponseVariants::from(error)));
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
                    Some(value) => match sqlx::Row::try_get::<<postgresql_crud::postgresql_type::StdPrimitiveI64AsNotNullBigSerialInitializedByPostgresql as postgresql_crud::PostgresqlTypePrimaryKey>::PrimaryKey, &std::primitive::str>(&value, "column_6e88acb0_c566_4fef_8a09_66a41338cf36") {
                        Ok(value) => Some(value),
                        Err(error_0) => {
                            drop(rows);
                            match executor.rollback().await {
                                Ok(_) => {
                                    let error = TryCreateManyRouteLogicErrorNamed::Postgresql {
                                        postgresql: error_0,
                                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                            file!().to_owned(),
                                            line!(),
                                            column!(),
                                            Some(error_occurence_lib::code_occurence::MacroOccurence {
                                                file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                                                line: 2693,
                                                column: 128,
                                            }),
                                        ),
                                    };
                                    eprintln!("{error}");
                                    let mut response = axum::response::IntoResponse::into_response(axum::Json(TryCreateManyRouteLogicResponseVariants::from(error)));
                                    *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                                    return response;
                                }
                                Err(error_1) => {
                                    let error = TryCreateManyRouteLogicErrorNamed::RowAndRollback {
                                        row: error_0,
                                        rollback: error_1,
                                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                            file!().to_owned(),
                                            line!(),
                                            column!(),
                                            Some(error_occurence_lib::code_occurence::MacroOccurence {
                                                file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                                                line: 2693,
                                                column: 157,
                                            }),
                                        ),
                                    };
                                    eprintln!("{error}");
                                    let mut response = axum::response::IntoResponse::into_response(axum::Json(TryCreateManyRouteLogicResponseVariants::from(error)));
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
                            let error = TryCreateManyRouteLogicErrorNamed::Postgresql {
                                postgresql: error_0,
                                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                    file!().to_owned(),
                                    line!(),
                                    column!(),
                                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                                        file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                                        line: 2695,
                                        column: 124,
                                    }),
                                ),
                            };
                            eprintln!("{error}");
                            let mut response = axum::response::IntoResponse::into_response(axum::Json(TryCreateManyRouteLogicResponseVariants::from(error)));
                            *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                            return response;
                        }
                        Err(error_1) => {
                            let error = TryCreateManyRouteLogicErrorNamed::RowAndRollback {
                                row: error_0,
                                rollback: error_1,
                                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                    file!().to_owned(),
                                    line!(),
                                    column!(),
                                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                                        file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                                        line: 2695,
                                        column: 153,
                                    }),
                                ),
                            };
                            eprintln!("{error}");
                            let mut response = axum::response::IntoResponse::into_response(axum::Json(TryCreateManyRouteLogicResponseVariants::from(error)));
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
        {
            let error_1 = value.len();
            if error_0 != error_1 {
                match executor.rollback().await {
                    Ok(_) => {
                        let error = TryCreateManyRouteLogicErrorNamed::UnexpectedRowsLength {
                            expected_length: error_0,
                            got_length: error_1,
                            code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                file!().to_owned(),
                                line!(),
                                column!(),
                                Some(error_occurence_lib::code_occurence::MacroOccurence {
                                    file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                                    line: 2880,
                                    column: 278,
                                }),
                            ),
                        };
                        eprintln!("{error}");
                        let mut response = axum::response::IntoResponse::into_response(axum::Json(TryCreateManyRouteLogicResponseVariants::from(error)));
                        *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                        return response;
                    }
                    Err(error_2) => {
                        let error = TryCreateManyRouteLogicErrorNamed::UnexpectedRowsLengthAndRollback {
                            expected_length: error_0,
                            got_length: error_1,
                            rollback: error_2,
                            code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                file!().to_owned(),
                                line!(),
                                column!(),
                                Some(error_occurence_lib::code_occurence::MacroOccurence {
                                    file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                                    line: 2882,
                                    column: 193,
                                }),
                            ),
                        };
                        eprintln!("{error}");
                        let mut response = axum::response::IntoResponse::into_response(axum::Json(TryCreateManyRouteLogicResponseVariants::from(error)));
                        *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                        return response;
                    }
                }
            }
        }
        if let Err(error_0) = executor.commit().await {
            let error = TryCreateManyRouteLogicErrorNamed::Postgresql {
                postgresql: error_0,
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_owned(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                        line: 2071,
                        column: 245,
                    }),
                ),
            };
            eprintln!("{error}");
            let mut response = axum::response::IntoResponse::into_response(axum::Json(TryCreateManyRouteLogicResponseVariants::from(error)));
            *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
            return response;
        }
        value
    };
    let mut response = axum::response::IntoResponse::into_response(axum::Json(TryCreateManyRouteLogicResponseVariants::Desirable(value)));
    *response.status_mut() = axum::http::StatusCode::CREATED;
    return response;
}
#[derive(Debug, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum TryCreateManyErrorNamed {
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
    TryCreateManyRouteLogicErrorNamedWithSerializeDeserialize {
        #[eo_to_std_string_string]
        try_create_many_route_logic_error_named_with_serialize_deserialize: TryCreateManyRouteLogicErrorNamedWithSerializeDeserialize,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
pub async fn try_create_many(
    server_location: &std::primitive::str,
    parameters: CreateManyParameters,
) -> Result<std::vec::Vec<<postgresql_crud::postgresql_type::StdPrimitiveI64AsNotNullBigSerialInitializedByPostgresql as postgresql_crud::PostgresqlTypePrimaryKey>::PrimaryKey>, TryCreateManyErrorNamed> {
    let payload = {
        let value = CreateManyPayload::from(parameters.payload);
        match serde_json::to_string(&value) {
            Ok(value) => value,
            Err(error_0) => {
                return Err(TryCreateManyErrorNamed::SerdeJsonToString {
                    serde_json_to_string: error_0,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                            line: 2405,
                            column: 178,
                        }),
                    ),
                });
            }
        }
    };
    let url = format!("{}/example/create_many", server_location,);
    let future = reqwest::Client::new()
        .post(&url)
        .header(&postgresql_crud::CommitSnakeCase.to_string(), git_info::PROJECT_GIT_INFO.commit)
        .header(reqwest::header::CONTENT_TYPE, "application/json")
        .body(payload)
        .send();
    let response = match future.await {
        Ok(value) => value,
        Err(error_0) => {
            return Err(TryCreateManyErrorNamed::Reqwest {
                reqwest: error_0,
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_owned(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                        line: 2457,
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
            return Err(TryCreateManyErrorNamed::FailedToGetResponseText {
                status_code: error_0,
                headers: error_1,
                reqwest: error_2,
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_owned(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                        line: 2474,
                        column: 192,
                    }),
                ),
            });
        }
    };
    let expected_response = match serde_json::from_str::<TryCreateManyRouteLogicResponseVariants>(&error_2) {
        Ok(value) => value,
        Err(error_3) => {
            return Err(TryCreateManyErrorNamed::DeserializeResponse {
                status_code: error_0,
                headers: error_1,
                response_text: error_2,
                serde: error_3,
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_owned(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                        line: 2487,
                        column: 178,
                    }),
                ),
            });
        }
    };
    let try_create_many_route_logic_error_named_with_serialize_deserialize = match expected_response {
        TryCreateManyRouteLogicResponseVariants::Desirable(value) => {
            let value = value;
            return Ok(value);
        }
        TryCreateManyRouteLogicResponseVariants::CheckBodySize { check_body_size, code_occurence } => TryCreateManyRouteLogicErrorNamedWithSerializeDeserialize::CheckBodySize { check_body_size, code_occurence },
        TryCreateManyRouteLogicResponseVariants::Postgresql { postgresql, code_occurence } => TryCreateManyRouteLogicErrorNamedWithSerializeDeserialize::Postgresql { postgresql, code_occurence },
        TryCreateManyRouteLogicResponseVariants::SerdeJson { serde_json, code_occurence } => TryCreateManyRouteLogicErrorNamedWithSerializeDeserialize::SerdeJson { serde_json, code_occurence },
        TryCreateManyRouteLogicResponseVariants::CheckCommit { check_commit, code_occurence } => TryCreateManyRouteLogicErrorNamedWithSerializeDeserialize::CheckCommit { check_commit, code_occurence },
        TryCreateManyRouteLogicResponseVariants::CheckedAdd { code_occurence } => TryCreateManyRouteLogicErrorNamedWithSerializeDeserialize::CheckedAdd { code_occurence },
        TryCreateManyRouteLogicResponseVariants::RowAndRollback { row, rollback, code_occurence } => TryCreateManyRouteLogicErrorNamedWithSerializeDeserialize::RowAndRollback { row, rollback, code_occurence },
        TryCreateManyRouteLogicResponseVariants::UnexpectedRowsLength { expected_length, got_length, code_occurence } => TryCreateManyRouteLogicErrorNamedWithSerializeDeserialize::UnexpectedRowsLength { expected_length, got_length, code_occurence },
        TryCreateManyRouteLogicResponseVariants::UnexpectedRowsLengthAndRollback { expected_length, got_length, rollback, code_occurence } => TryCreateManyRouteLogicErrorNamedWithSerializeDeserialize::UnexpectedRowsLengthAndRollback { expected_length, got_length, rollback, code_occurence },
    };
    Err(TryCreateManyErrorNamed::TryCreateManyRouteLogicErrorNamedWithSerializeDeserialize {
        try_create_many_route_logic_error_named_with_serialize_deserialize,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
            file!().to_owned(),
            line!(),
            column!(),
            Some(error_occurence_lib::code_occurence::MacroOccurence {
                file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                line: 2525,
                column: 223,
            }),
        ),
    })
}
impl postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for CreateManyPayloadElement {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self {
            column_6e88acb0_c566_4fef_8a09_66a41338cf36: postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            animal_as_not_null_jsonb_object: postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
        }
    }
}
impl postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for CreateManyPayload {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self(vec![postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()])
    }
}
pub async fn create_many_payload_example_route_logic() -> axum::response::Response {
    let mut response = axum::response::IntoResponse::into_response(axum::Json(
        <CreateManyPayload as postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement>::default_but_option_is_always_some_and_vec_always_contains_one_element(),
    ));
    *response.status_mut() = axum::http::StatusCode::OK;
    return response;
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema)]
pub struct CreateOnePayload {
    pub column_6e88acb0_c566_4fef_8a09_66a41338cf36: <postgresql_crud::postgresql_type::StdPrimitiveI64AsNotNullBigSerialInitializedByPostgresql as postgresql_crud::PostgresqlType>::Create,
    pub animal_as_not_null_jsonb_object: <AnimalAsNotNullJsonbObject as postgresql_crud::PostgresqlType>::Create,
}
#[derive(Debug)]
pub struct CreateOneParameters {
    pub payload: CreateOnePayload,
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize)]
pub enum TryCreateOneRouteLogicResponseVariants {
    Desirable(<postgresql_crud::postgresql_type::StdPrimitiveI64AsNotNullBigSerialInitializedByPostgresql as postgresql_crud::PostgresqlTypePrimaryKey>::PrimaryKey),
    CheckBodySize {
        check_body_size: postgresql_crud::check_body_size::CheckBodySizeErrorNamedWithSerializeDeserialize,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Postgresql {
        postgresql: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    SerdeJson {
        serde_json: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    CheckCommit {
        check_commit: postgresql_crud::check_commit::CheckCommitErrorNamedWithSerializeDeserialize,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    RowAndRollback {
        row: std::string::String,
        rollback: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    CheckedAdd {
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
impl std::convert::From<TryCreateOneRouteLogicErrorNamed> for TryCreateOneRouteLogicResponseVariants {
    fn from(value: TryCreateOneRouteLogicErrorNamed) -> Self {
        match value.into_serialize_deserialize_version() {
            TryCreateOneRouteLogicErrorNamedWithSerializeDeserialize::CheckBodySize { check_body_size, code_occurence } => Self::CheckBodySize { check_body_size, code_occurence },
            TryCreateOneRouteLogicErrorNamedWithSerializeDeserialize::Postgresql { postgresql, code_occurence } => Self::Postgresql { postgresql, code_occurence },
            TryCreateOneRouteLogicErrorNamedWithSerializeDeserialize::SerdeJson { serde_json, code_occurence } => Self::SerdeJson { serde_json, code_occurence },
            TryCreateOneRouteLogicErrorNamedWithSerializeDeserialize::CheckCommit { check_commit, code_occurence } => Self::CheckCommit { check_commit, code_occurence },
            TryCreateOneRouteLogicErrorNamedWithSerializeDeserialize::RowAndRollback { row, rollback, code_occurence } => Self::RowAndRollback { row, rollback, code_occurence },
            TryCreateOneRouteLogicErrorNamedWithSerializeDeserialize::CheckedAdd { code_occurence } => Self::CheckedAdd { code_occurence },
        }
    }
}
#[derive(Debug, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum TryCreateOneRouteLogicErrorNamed {
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
    CheckedAdd {
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
pub async fn try_create_one_route_logic(app_state: axum::extract::State<crate::repositories_types::server::routes::app_state::DynArcCombinationOfAppStateLogicTraits>, request: axum::extract::Request) -> axum::response::Response {
    let (parts, body) = request.into_parts();
    let headers = parts.headers;
    let body_bytes = match postgresql_crud::check_body_size::check_body_size(body, *app_state.get_maximum_size_of_http_body_in_bytes()).await {
        Ok(value) => value,
        Err(error_0) => {
            let error = TryCreateOneRouteLogicErrorNamed::CheckBodySize {
                check_body_size: error_0,
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_owned(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                        line: 2290,
                        column: 264,
                    }),
                ),
            };
            eprintln!("{error}");
            let mut response = axum::response::IntoResponse::into_response(axum::Json(TryCreateOneRouteLogicResponseVariants::from(error)));
            *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
            return response;
        }
    };
    let parameters = CreateOneParameters {
        payload: match serde_json::from_slice::<CreateOnePayload>(&body_bytes) {
            Ok(value) => {
                let value = CreateOnePayload::from(value);
                value
            }
            Err(error_0) => {
                let error = TryCreateOneRouteLogicErrorNamed::SerdeJson {
                    serde_json: error_0,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                            line: 2362,
                            column: 249,
                        }),
                    ),
                };
                eprintln!("{error}");
                let mut response = axum::response::IntoResponse::into_response(axum::Json(TryCreateOneRouteLogicResponseVariants::from(error)));
                *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
                return response;
            }
        },
    };
    let query_string = {
        let mut increment: std::primitive::u64 = 0;
        format!(
            "insert into example (column_6e88acb0_c566_4fef_8a09_66a41338cf36,animal_as_not_null_jsonb_object) values ({},{})  returning column_6e88acb0_c566_4fef_8a09_66a41338cf36",
            match <postgresql_crud::postgresql_type::StdPrimitiveI64AsNotNullBigSerialInitializedByPostgresql as postgresql_crud::PostgresqlType>::create_query_part(&parameters.payload.column_6e88acb0_c566_4fef_8a09_66a41338cf36, &mut increment) {
                Ok(value) => value,
                Err(error_0) => {
                    let error = TryCreateOneRouteLogicErrorNamed::CheckedAdd {
                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                            file!().to_owned(),
                            line!(),
                            column!(),
                            Some(error_occurence_lib::code_occurence::MacroOccurence {
                                file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                                line: 3075,
                                column: 260,
                            }),
                        ),
                    };
                    eprintln!("{error}");
                    let mut response = axum::response::IntoResponse::into_response(axum::Json(TryCreateOneRouteLogicResponseVariants::from(error)));
                    *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
                    return response;
                }
            },
            match <AnimalAsNotNullJsonbObject as postgresql_crud::PostgresqlType>::create_query_part(&parameters.payload.animal_as_not_null_jsonb_object, &mut increment) {
                Ok(value) => value,
                Err(error_0) => {
                    let error = TryCreateOneRouteLogicErrorNamed::CheckedAdd {
                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                            file!().to_owned(),
                            line!(),
                            column!(),
                            Some(error_occurence_lib::code_occurence::MacroOccurence {
                                file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                                line: 3075,
                                column: 260,
                            }),
                        ),
                    };
                    eprintln!("{error}");
                    let mut response = axum::response::IntoResponse::into_response(axum::Json(TryCreateOneRouteLogicResponseVariants::from(error)));
                    *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
                    return response;
                }
            }
        )
    };
    println!("{}", query_string);
    let binded_query = {
        let mut query = sqlx::query::<sqlx::Postgres>(&query_string);
        query = <postgresql_crud::postgresql_type::StdPrimitiveI64AsNotNullBigSerialInitializedByPostgresql as postgresql_crud::PostgresqlType>::create_query_bind(parameters.payload.column_6e88acb0_c566_4fef_8a09_66a41338cf36, query);
        query = <AnimalAsNotNullJsonbObject as postgresql_crud::PostgresqlType>::create_query_bind(parameters.payload.animal_as_not_null_jsonb_object, query);
        query
    };
    let mut pool_connection = match app_state.get_postgres_pool().acquire().await {
        Ok(value) => value,
        Err(error_0) => {
            let error = TryCreateOneRouteLogicErrorNamed::Postgresql {
                postgresql: error_0,
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_owned(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                        line: 2312,
                        column: 253,
                    }),
                ),
            };
            eprintln!("{error}");
            let mut response = axum::response::IntoResponse::into_response(axum::Json(TryCreateOneRouteLogicResponseVariants::from(error)));
            *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
            return response;
        }
    };
    let executor = match sqlx::Acquire::acquire(&mut pool_connection).await {
        Ok(value) => value,
        Err(error_0) => {
            let error = TryCreateOneRouteLogicErrorNamed::Postgresql {
                postgresql: error_0,
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_owned(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                        line: 2312,
                        column: 253,
                    }),
                ),
            };
            eprintln!("{error}");
            let mut response = axum::response::IntoResponse::into_response(axum::Json(TryCreateOneRouteLogicResponseVariants::from(error)));
            *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
            return response;
        }
    };
    let value = {
        let mut executor = match sqlx::Acquire::begin(executor).await {
            Ok(value) => value,
            Err(error_0) => {
                let error = TryCreateOneRouteLogicErrorNamed::Postgresql {
                    postgresql: error_0,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                            line: 2059,
                            column: 245,
                        }),
                    ),
                };
                eprintln!("{error}");
                let mut response = axum::response::IntoResponse::into_response(axum::Json(TryCreateOneRouteLogicResponseVariants::from(error)));
                *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                return response;
            }
        };
        let value = {
            match binded_query.fetch_one(executor.as_mut()).await {
                Ok(value) => match sqlx::Row::try_get::<<postgresql_crud::postgresql_type::StdPrimitiveI64AsNotNullBigSerialInitializedByPostgresql as postgresql_crud::PostgresqlTypePrimaryKey>::PrimaryKey, &std::primitive::str>(&value, "column_6e88acb0_c566_4fef_8a09_66a41338cf36") {
                    Ok(value) => value,
                    Err(error_0) => match executor.rollback().await {
                        Ok(_) => {
                            let error = TryCreateOneRouteLogicErrorNamed::Postgresql {
                                postgresql: error_0,
                                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                    file!().to_owned(),
                                    line!(),
                                    column!(),
                                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                                        file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                                        line: 2711,
                                        column: 111,
                                    }),
                                ),
                            };
                            eprintln!("{error}");
                            let mut response = axum::response::IntoResponse::into_response(axum::Json(TryCreateOneRouteLogicResponseVariants::from(error)));
                            *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                            return response;
                        }
                        Err(error_1) => {
                            let error = TryCreateOneRouteLogicErrorNamed::RowAndRollback {
                                row: error_0,
                                rollback: error_1,
                                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                    file!().to_owned(),
                                    line!(),
                                    column!(),
                                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                                        file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                                        line: 2711,
                                        column: 140,
                                    }),
                                ),
                            };
                            eprintln!("{error}");
                            let mut response = axum::response::IntoResponse::into_response(axum::Json(TryCreateOneRouteLogicResponseVariants::from(error)));
                            *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                            return response;
                        }
                    },
                },
                Err(error_0) => match executor.rollback().await {
                    Ok(_) => {
                        let error = TryCreateOneRouteLogicErrorNamed::Postgresql {
                            postgresql: error_0,
                            code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                file!().to_owned(),
                                line!(),
                                column!(),
                                Some(error_occurence_lib::code_occurence::MacroOccurence {
                                    file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                                    line: 2713,
                                    column: 107,
                                }),
                            ),
                        };
                        eprintln!("{error}");
                        let mut response = axum::response::IntoResponse::into_response(axum::Json(TryCreateOneRouteLogicResponseVariants::from(error)));
                        *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                        return response;
                    }
                    Err(error_1) => {
                        let error = TryCreateOneRouteLogicErrorNamed::RowAndRollback {
                            row: error_0,
                            rollback: error_1,
                            code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                file!().to_owned(),
                                line!(),
                                column!(),
                                Some(error_occurence_lib::code_occurence::MacroOccurence {
                                    file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                                    line: 2713,
                                    column: 136,
                                }),
                            ),
                        };
                        eprintln!("{error}");
                        let mut response = axum::response::IntoResponse::into_response(axum::Json(TryCreateOneRouteLogicResponseVariants::from(error)));
                        *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                        return response;
                    }
                },
            }
        };
        if let Err(error_0) = executor.commit().await {
            let error = TryCreateOneRouteLogicErrorNamed::Postgresql {
                postgresql: error_0,
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_owned(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                        line: 2071,
                        column: 245,
                    }),
                ),
            };
            eprintln!("{error}");
            let mut response = axum::response::IntoResponse::into_response(axum::Json(TryCreateOneRouteLogicResponseVariants::from(error)));
            *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
            return response;
        }
        value
    };
    let mut response = axum::response::IntoResponse::into_response(axum::Json(TryCreateOneRouteLogicResponseVariants::Desirable(value)));
    *response.status_mut() = axum::http::StatusCode::CREATED;
    return response;
}
#[derive(Debug, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum TryCreateOneErrorNamed {
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
    TryCreateOneRouteLogicErrorNamedWithSerializeDeserialize {
        #[eo_to_std_string_string]
        try_create_one_route_logic_error_named_with_serialize_deserialize: TryCreateOneRouteLogicErrorNamedWithSerializeDeserialize,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
pub async fn try_create_one(server_location: &std::primitive::str, parameters: CreateOneParameters) -> Result<<postgresql_crud::postgresql_type::StdPrimitiveI64AsNotNullBigSerialInitializedByPostgresql as postgresql_crud::PostgresqlTypePrimaryKey>::PrimaryKey, TryCreateOneErrorNamed> {
    let payload = {
        let value = CreateOnePayload::from(parameters.payload);
        match serde_json::to_string(&value) {
            Ok(value) => value,
            Err(error_0) => {
                return Err(TryCreateOneErrorNamed::SerdeJsonToString {
                    serde_json_to_string: error_0,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                            line: 2405,
                            column: 178,
                        }),
                    ),
                });
            }
        }
    };
    let url = format!("{}/example/create_one", server_location,);
    let future = reqwest::Client::new()
        .post(&url)
        .header(&postgresql_crud::CommitSnakeCase.to_string(), git_info::PROJECT_GIT_INFO.commit)
        .header(reqwest::header::CONTENT_TYPE, "application/json")
        .body(payload)
        .send();
    let response = match future.await {
        Ok(value) => value,
        Err(error_0) => {
            return Err(TryCreateOneErrorNamed::Reqwest {
                reqwest: error_0,
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_owned(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                        line: 2457,
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
            return Err(TryCreateOneErrorNamed::FailedToGetResponseText {
                status_code: error_0,
                headers: error_1,
                reqwest: error_2,
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_owned(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                        line: 2474,
                        column: 192,
                    }),
                ),
            });
        }
    };
    let expected_response = match serde_json::from_str::<TryCreateOneRouteLogicResponseVariants>(&error_2) {
        Ok(value) => value,
        Err(error_3) => {
            return Err(TryCreateOneErrorNamed::DeserializeResponse {
                status_code: error_0,
                headers: error_1,
                response_text: error_2,
                serde: error_3,
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_owned(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                        line: 2487,
                        column: 178,
                    }),
                ),
            });
        }
    };
    let try_create_one_route_logic_error_named_with_serialize_deserialize = match expected_response {
        TryCreateOneRouteLogicResponseVariants::Desirable(value) => {
            let value = value;
            return Ok(value);
        }
        TryCreateOneRouteLogicResponseVariants::CheckBodySize { check_body_size, code_occurence } => TryCreateOneRouteLogicErrorNamedWithSerializeDeserialize::CheckBodySize { check_body_size, code_occurence },
        TryCreateOneRouteLogicResponseVariants::Postgresql { postgresql, code_occurence } => TryCreateOneRouteLogicErrorNamedWithSerializeDeserialize::Postgresql { postgresql, code_occurence },
        TryCreateOneRouteLogicResponseVariants::SerdeJson { serde_json, code_occurence } => TryCreateOneRouteLogicErrorNamedWithSerializeDeserialize::SerdeJson { serde_json, code_occurence },
        TryCreateOneRouteLogicResponseVariants::CheckCommit { check_commit, code_occurence } => TryCreateOneRouteLogicErrorNamedWithSerializeDeserialize::CheckCommit { check_commit, code_occurence },
        TryCreateOneRouteLogicResponseVariants::RowAndRollback { row, rollback, code_occurence } => TryCreateOneRouteLogicErrorNamedWithSerializeDeserialize::RowAndRollback { row, rollback, code_occurence },
        TryCreateOneRouteLogicResponseVariants::CheckedAdd { code_occurence } => TryCreateOneRouteLogicErrorNamedWithSerializeDeserialize::CheckedAdd { code_occurence },
    };
    Err(TryCreateOneErrorNamed::TryCreateOneRouteLogicErrorNamedWithSerializeDeserialize {
        try_create_one_route_logic_error_named_with_serialize_deserialize,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
            file!().to_owned(),
            line!(),
            column!(),
            Some(error_occurence_lib::code_occurence::MacroOccurence {
                file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                line: 2525,
                column: 223,
            }),
        ),
    })
}
impl postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for CreateOnePayload {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self {
            column_6e88acb0_c566_4fef_8a09_66a41338cf36: postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            animal_as_not_null_jsonb_object: postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
        }
    }
}
pub async fn create_one_payload_example_route_logic() -> axum::response::Response {
    let mut response = axum::response::IntoResponse::into_response(axum::Json(
        <CreateOnePayload as postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement>::default_but_option_is_always_some_and_vec_always_contains_one_element(),
    ));
    *response.status_mut() = axum::http::StatusCode::OK;
    return response;
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema)]
pub struct ReadManyPayload {
    pub column_6e88acb0_c566_4fef_8a09_66a41338cf36: std::option::Option<postgresql_crud::PostgresqlTypeWhere<<postgresql_crud::postgresql_type::StdPrimitiveI64AsNotNullBigSerialInitializedByPostgresql as postgresql_crud::PostgresqlType>::WhereElement>>,
    pub animal_as_not_null_jsonb_object: std::option::Option<postgresql_crud::PostgresqlTypeWhere<<AnimalAsNotNullJsonbObject as postgresql_crud::PostgresqlType>::WhereElement>>,
    pub select: std::vec::Vec<ExampleSelect>,
    pub order_by: postgresql_crud::OrderBy<ExampleSelect>,
    pub pagination: postgresql_crud::Pagination,
}
#[derive(Debug)]
pub struct ReadManyParameters {
    pub payload: ReadManyPayload,
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize)]
pub enum TryReadManyRouteLogicResponseVariants {
    Desirable(std::vec::Vec<ExampleOptions>),
    CheckBodySize {
        check_body_size: postgresql_crud::check_body_size::CheckBodySizeErrorNamedWithSerializeDeserialize,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Postgresql {
        postgresql: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    SerdeJson {
        serde_json: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    CheckCommit {
        check_commit: postgresql_crud::check_commit::CheckCommitErrorNamedWithSerializeDeserialize,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    CheckedAdd {
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NotUniqueColumn {
        not_unique_column: ExampleSelect,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
impl std::convert::From<TryReadManyRouteLogicErrorNamed> for TryReadManyRouteLogicResponseVariants {
    fn from(value: TryReadManyRouteLogicErrorNamed) -> Self {
        match value.into_serialize_deserialize_version() {
            TryReadManyRouteLogicErrorNamedWithSerializeDeserialize::CheckBodySize { check_body_size, code_occurence } => Self::CheckBodySize { check_body_size, code_occurence },
            TryReadManyRouteLogicErrorNamedWithSerializeDeserialize::Postgresql { postgresql, code_occurence } => Self::Postgresql { postgresql, code_occurence },
            TryReadManyRouteLogicErrorNamedWithSerializeDeserialize::SerdeJson { serde_json, code_occurence } => Self::SerdeJson { serde_json, code_occurence },
            TryReadManyRouteLogicErrorNamedWithSerializeDeserialize::CheckCommit { check_commit, code_occurence } => Self::CheckCommit { check_commit, code_occurence },
            TryReadManyRouteLogicErrorNamedWithSerializeDeserialize::CheckedAdd { code_occurence } => Self::CheckedAdd { code_occurence },
            TryReadManyRouteLogicErrorNamedWithSerializeDeserialize::NotUniqueColumn { not_unique_column, code_occurence } => Self::NotUniqueColumn { not_unique_column, code_occurence },
        }
    }
}
#[derive(Debug, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum TryReadManyRouteLogicErrorNamed {
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
    CheckCommit {
        #[eo_error_occurence]
        check_commit: postgresql_crud::check_commit::CheckCommitErrorNamed,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    CheckedAdd {
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NotUniqueColumn {
        #[eo_to_std_string_string_serialize_deserialize]
        not_unique_column: ExampleSelect,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
pub async fn try_read_many_route_logic(app_state: axum::extract::State<crate::repositories_types::server::routes::app_state::DynArcCombinationOfAppStateLogicTraits>, request: axum::extract::Request) -> axum::response::Response {
    let (parts, body) = request.into_parts();
    let headers = parts.headers;
    let body_bytes = match postgresql_crud::check_body_size::check_body_size(body, *app_state.get_maximum_size_of_http_body_in_bytes()).await {
        Ok(value) => value,
        Err(error_0) => {
            let error = TryReadManyRouteLogicErrorNamed::CheckBodySize {
                check_body_size: error_0,
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_owned(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                        line: 2290,
                        column: 264,
                    }),
                ),
            };
            eprintln!("{error}");
            let mut response = axum::response::IntoResponse::into_response(axum::Json(TryReadManyRouteLogicResponseVariants::from(error)));
            *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
            return response;
        }
    };
    let parameters = ReadManyParameters {
        payload: match serde_json::from_slice::<ReadManyPayload>(&body_bytes) {
            Ok(value) => {
                let value = ReadManyPayload::from(value);
                value
            }
            Err(error_0) => {
                let error = TryReadManyRouteLogicErrorNamed::SerdeJson {
                    serde_json: error_0,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                            line: 2362,
                            column: 249,
                        }),
                    ),
                };
                eprintln!("{error}");
                let mut response = axum::response::IntoResponse::into_response(axum::Json(TryReadManyRouteLogicResponseVariants::from(error)));
                *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
                return response;
            }
        },
    };
    let query_string = {
        format!(
            "select {} from example {}",
            {
                let mut value = std::string::String::default();
                for element in &parameters.payload.select {
                    value.push_str(&match element {
                        ExampleSelect::Column6E88Acb0C5664Fef8A0966A41338Cf36(value) => <postgresql_crud::postgresql_type::StdPrimitiveI64AsNotNullBigSerialInitializedByPostgresql as postgresql_crud::PostgresqlType>::select_query_part(value, "column_6e88acb0_c566_4fef_8a09_66a41338cf36"),
                        ExampleSelect::AnimalAsNotNullJsonbObject(value) => <AnimalAsNotNullJsonbObject as postgresql_crud::PostgresqlType>::select_query_part(value, "animal_as_not_null_jsonb_object"),
                    });
                    value.push_str(",");
                }
                let _: Option<char> = value.pop();
                value
            },
            {
                let mut increment: std::primitive::u64 = 0;
                let mut additional_parameters = std::string::String::from("where");
                let mut is_first_push_to_additional_parameters_already_happend = false;
                if let Some(value) = &parameters.payload.column_6e88acb0_c566_4fef_8a09_66a41338cf36 {
                    match postgresql_crud::PostgresqlTypeWhereFilter::query_part(value, &mut increment, &"column_6e88acb0_c566_4fef_8a09_66a41338cf36", is_first_push_to_additional_parameters_already_happend) {
                        Ok(value) => {
                            additional_parameters.push_str(&value);
                            is_first_push_to_additional_parameters_already_happend = true;
                        }
                        Err(error_0) => {
                            let error = TryReadManyRouteLogicErrorNamed::CheckedAdd {
                                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                    file!().to_owned(),
                                    line!(),
                                    column!(),
                                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                                        file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                                        line: 3268,
                                        column: 260,
                                    }),
                                ),
                            };
                            eprintln!("{error}");
                            let mut response = axum::response::IntoResponse::into_response(axum::Json(TryReadManyRouteLogicResponseVariants::from(error)));
                            *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
                            return response;
                        }
                    }
                }
                if let Some(value) = &parameters.payload.animal_as_not_null_jsonb_object {
                    match postgresql_crud::PostgresqlTypeWhereFilter::query_part(value, &mut increment, &"animal_as_not_null_jsonb_object", is_first_push_to_additional_parameters_already_happend) {
                        Ok(value) => {
                            additional_parameters.push_str(&value);
                            // is_first_push_to_additional_parameters_already_happend = true;
                        }
                        Err(error_0) => {
                            let error = TryReadManyRouteLogicErrorNamed::CheckedAdd {
                                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                    file!().to_owned(),
                                    line!(),
                                    column!(),
                                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                                        file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                                        line: 3268,
                                        column: 260,
                                    }),
                                ),
                            };
                            eprintln!("{error}");
                            let mut response = axum::response::IntoResponse::into_response(axum::Json(TryReadManyRouteLogicResponseVariants::from(error)));
                            *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
                            return response;
                        }
                    }
                }
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
                    additional_parameters.push_str(&format!("{}order by {} {}", prefix, value.column.pick_select(), order,));
                }
                {
                    let prefix = match additional_parameters.is_empty() {
                        true => "",
                        false => " ",
                    };
                    let value = match postgresql_crud::PostgresqlTypeWhereFilter::query_part(&parameters.payload.pagination, &mut increment, &"", std::primitive::bool::default()) {
                        Ok(value) => value,
                        Err(error_0) => {
                            let error = TryReadManyRouteLogicErrorNamed::CheckedAdd {
                                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                    file!().to_owned(),
                                    line!(),
                                    column!(),
                                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                                        file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                                        line: 3301,
                                        column: 256,
                                    }),
                                ),
                            };
                            eprintln!("{error}");
                            let mut response = axum::response::IntoResponse::into_response(axum::Json(TryReadManyRouteLogicResponseVariants::from(error)));
                            *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
                            return response;
                        }
                    };
                    additional_parameters.push_str(&format!("{}{}", prefix, value));
                }
                additional_parameters
            }
        )
    };
    println!("{}", query_string);
    let binded_query = {
        let mut query = sqlx::query::<sqlx::Postgres>(&query_string);
        if let Some(value) = parameters.payload.column_6e88acb0_c566_4fef_8a09_66a41338cf36 {
            query = postgresql_crud::PostgresqlTypeWhereFilter::query_bind(value, query);
        }
        if let Some(value) = parameters.payload.animal_as_not_null_jsonb_object {
            query = postgresql_crud::PostgresqlTypeWhereFilter::query_bind(value, query);
        }
        query = postgresql_crud::PostgresqlTypeWhereFilter::query_bind(parameters.payload.pagination, query);
        query
    };
    let mut pool_connection = match app_state.get_postgres_pool().acquire().await {
        Ok(value) => value,
        Err(error_0) => {
            let error = TryReadManyRouteLogicErrorNamed::Postgresql {
                postgresql: error_0,
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_owned(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                        line: 2312,
                        column: 253,
                    }),
                ),
            };
            eprintln!("{error}");
            let mut response = axum::response::IntoResponse::into_response(axum::Json(TryReadManyRouteLogicResponseVariants::from(error)));
            *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
            return response;
        }
    };
    let executor = match sqlx::Acquire::acquire(&mut pool_connection).await {
        Ok(value) => value,
        Err(error_0) => {
            let error = TryReadManyRouteLogicErrorNamed::Postgresql {
                postgresql: error_0,
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_owned(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                        line: 2312,
                        column: 253,
                    }),
                ),
            };
            eprintln!("{error}");
            let mut response = axum::response::IntoResponse::into_response(axum::Json(TryReadManyRouteLogicResponseVariants::from(error)));
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
                    Some(value) => Some({
                        let mut column_6e88acb0_c566_4fef_8a09_66a41338cf36: std::option::Option<postgresql_crud::Value<<postgresql_crud::postgresql_type::StdPrimitiveI64AsNotNullBigSerialInitializedByPostgresql as postgresql_crud::PostgresqlTypePrimaryKey>::PrimaryKey>> = None;
                        let mut animal_as_not_null_jsonb_object: std::option::Option<postgresql_crud::Value<<AnimalAsNotNullJsonbObject as postgresql_crud::PostgresqlType>::Read>> = None;
                        for element in &parameters.payload.select {
                            match element {
                                ExampleSelect::Column6E88Acb0C5664Fef8A0966A41338Cf36(_) => {
                                    match sqlx::Row::try_get::<<postgresql_crud::postgresql_type::StdPrimitiveI64AsNotNullBigSerialInitializedByPostgresql as postgresql_crud::PostgresqlTypePrimaryKey>::PrimaryKey, &std::primitive::str>(&value, "column_6e88acb0_c566_4fef_8a09_66a41338cf36") {
                                        Ok(value) => {
                                            column_6e88acb0_c566_4fef_8a09_66a41338cf36 = Some(postgresql_crud::Value { value: value });
                                        }
                                        Err(error_0) => {
                                            let error = TryReadManyRouteLogicErrorNamed::Postgresql {
                                                postgresql: error_0,
                                                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                                    file!().to_owned(),
                                                    line!(),
                                                    column!(),
                                                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                                                        file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                                                        line: 1186,
                                                        column: 245,
                                                    }),
                                                ),
                                            };
                                            eprintln!("{error}");
                                            let mut response = axum::response::IntoResponse::into_response(axum::Json(TryReadManyRouteLogicResponseVariants::from(error)));
                                            *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                                            return response;
                                        }
                                    }
                                }
                                ExampleSelect::AnimalAsNotNullJsonbObject(_) => match sqlx::Row::try_get::<<AnimalAsNotNullJsonbObject as postgresql_crud::PostgresqlType>::Read, &std::primitive::str>(&value, "animal_as_not_null_jsonb_object") {
                                    Ok(value) => {
                                        animal_as_not_null_jsonb_object = Some(postgresql_crud::Value { value: value });
                                    }
                                    Err(error_0) => {
                                        let error = TryReadManyRouteLogicErrorNamed::Postgresql {
                                            postgresql: error_0,
                                            code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                                file!().to_owned(),
                                                line!(),
                                                column!(),
                                                Some(error_occurence_lib::code_occurence::MacroOccurence {
                                                    file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                                                    line: 1223,
                                                    column: 249,
                                                }),
                                            ),
                                        };
                                        eprintln!("{error}");
                                        let mut response = axum::response::IntoResponse::into_response(axum::Json(TryReadManyRouteLogicResponseVariants::from(error)));
                                        *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                                        return response;
                                    }
                                },
                            }
                        }
                        ExampleOptions {
                            column_6e88acb0_c566_4fef_8a09_66a41338cf36,
                            animal_as_not_null_jsonb_object,
                        }
                    }),
                    None => None,
                },
                Err(error_0) => {
                    let error = TryReadManyRouteLogicErrorNamed::Postgresql {
                        postgresql: error_0,
                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                            file!().to_owned(),
                            line!(),
                            column!(),
                            Some(error_occurence_lib::code_occurence::MacroOccurence {
                                file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                                line: 3382,
                                column: 169,
                            }),
                        ),
                    };
                    eprintln!("{error}");
                    let mut response = axum::response::IntoResponse::into_response(axum::Json(TryReadManyRouteLogicResponseVariants::from(error)));
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
    let mut response = axum::response::IntoResponse::into_response(axum::Json(TryReadManyRouteLogicResponseVariants::Desirable(value)));
    *response.status_mut() = axum::http::StatusCode::OK;
    return response;
}
#[derive(Debug, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum TryReadManyErrorNamed {
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
    NotUniqueColumn {
        #[eo_to_std_string_string_serialize_deserialize]
        not_unique_column: ExampleSelect,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    TryReadManyRouteLogicErrorNamedWithSerializeDeserialize {
        #[eo_to_std_string_string]
        try_read_many_route_logic_error_named_with_serialize_deserialize: TryReadManyRouteLogicErrorNamedWithSerializeDeserialize,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
pub async fn try_read_many(server_location: &std::primitive::str, parameters: ReadManyParameters) -> Result<std::vec::Vec<ExampleOptions>, TryReadManyErrorNamed> {
    let payload = {
        let value = ReadManyPayload::from(parameters.payload);
        match serde_json::to_string(&value) {
            Ok(value) => value,
            Err(error_0) => {
                return Err(TryReadManyErrorNamed::SerdeJsonToString {
                    serde_json_to_string: error_0,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                            line: 2405,
                            column: 178,
                        }),
                    ),
                });
            }
        }
    };
    let url = format!("{}/example/read_many", server_location,);
    let future = reqwest::Client::new()
        .post(&url)
        .header(&postgresql_crud::CommitSnakeCase.to_string(), git_info::PROJECT_GIT_INFO.commit)
        .header(reqwest::header::CONTENT_TYPE, "application/json")
        .body(payload)
        .send();
    let response = match future.await {
        Ok(value) => value,
        Err(error_0) => {
            return Err(TryReadManyErrorNamed::Reqwest {
                reqwest: error_0,
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_owned(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                        line: 2457,
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
            return Err(TryReadManyErrorNamed::FailedToGetResponseText {
                status_code: error_0,
                headers: error_1,
                reqwest: error_2,
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_owned(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                        line: 2474,
                        column: 192,
                    }),
                ),
            });
        }
    };
    let expected_response = match serde_json::from_str::<TryReadManyRouteLogicResponseVariants>(&error_2) {
        Ok(value) => value,
        Err(error_3) => {
            return Err(TryReadManyErrorNamed::DeserializeResponse {
                status_code: error_0,
                headers: error_1,
                response_text: error_2,
                serde: error_3,
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_owned(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                        line: 2487,
                        column: 178,
                    }),
                ),
            });
        }
    };
    let try_read_many_route_logic_error_named_with_serialize_deserialize = match expected_response {
        TryReadManyRouteLogicResponseVariants::Desirable(value) => {
            let value = value.into_iter().fold(std::vec::Vec::new(), |mut acc, element| {
                acc.push(element);
                acc
            });
            return Ok(value);
        }
        TryReadManyRouteLogicResponseVariants::CheckBodySize { check_body_size, code_occurence } => TryReadManyRouteLogicErrorNamedWithSerializeDeserialize::CheckBodySize { check_body_size, code_occurence },
        TryReadManyRouteLogicResponseVariants::Postgresql { postgresql, code_occurence } => TryReadManyRouteLogicErrorNamedWithSerializeDeserialize::Postgresql { postgresql, code_occurence },
        TryReadManyRouteLogicResponseVariants::SerdeJson { serde_json, code_occurence } => TryReadManyRouteLogicErrorNamedWithSerializeDeserialize::SerdeJson { serde_json, code_occurence },
        TryReadManyRouteLogicResponseVariants::CheckCommit { check_commit, code_occurence } => TryReadManyRouteLogicErrorNamedWithSerializeDeserialize::CheckCommit { check_commit, code_occurence },
        TryReadManyRouteLogicResponseVariants::CheckedAdd { code_occurence } => TryReadManyRouteLogicErrorNamedWithSerializeDeserialize::CheckedAdd { code_occurence },
        TryReadManyRouteLogicResponseVariants::NotUniqueColumn { not_unique_column, code_occurence } => TryReadManyRouteLogicErrorNamedWithSerializeDeserialize::NotUniqueColumn { not_unique_column, code_occurence },
    };
    Err(TryReadManyErrorNamed::TryReadManyRouteLogicErrorNamedWithSerializeDeserialize {
        try_read_many_route_logic_error_named_with_serialize_deserialize,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
            file!().to_owned(),
            line!(),
            column!(),
            Some(error_occurence_lib::code_occurence::MacroOccurence {
                file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                line: 2525,
                column: 223,
            }),
        ),
    })
}
impl postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for ReadManyPayload {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self {
            column_6e88acb0_c566_4fef_8a09_66a41338cf36: Some(postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            animal_as_not_null_jsonb_object: Some(postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            select: postgresql_crud::AllEnumVariantsArrayDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::all_enum_variants_array_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
            order_by: postgresql_crud::OrderBy {
                column: ExampleSelect::Column6E88Acb0C5664Fef8A0966A41338Cf36(postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
                order: Some(postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            },
            pagination: postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
        }
    }
}
pub async fn read_many_payload_example_route_logic() -> axum::response::Response {
    let mut response = axum::response::IntoResponse::into_response(axum::Json(<ReadManyPayload as postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement>::default_but_option_is_always_some_and_vec_always_contains_one_element()));
    *response.status_mut() = axum::http::StatusCode::OK;
    return response;
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema)]
pub struct ReadOnePayload {
    pub column_6e88acb0_c566_4fef_8a09_66a41338cf36: postgresql_crud::postgresql_type::StdPrimitiveI64AsNotNullBigSerialInitializedByPostgresqlRead,
    pub select: std::vec::Vec<ExampleSelect>,
}
#[derive(Debug)]
pub struct ReadOneParameters {
    pub payload: ReadOnePayload,
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize)]
pub enum TryReadOneRouteLogicResponseVariants {
    Desirable(ExampleOptions),
    CheckBodySize {
        check_body_size: postgresql_crud::check_body_size::CheckBodySizeErrorNamedWithSerializeDeserialize,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Postgresql {
        postgresql: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    SerdeJson {
        serde_json: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    CheckCommit {
        check_commit: postgresql_crud::check_commit::CheckCommitErrorNamedWithSerializeDeserialize,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NotUniqueColumn {
        not_unique_column: ExampleSelect,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    CheckedAdd {
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
impl std::convert::From<TryReadOneRouteLogicErrorNamed> for TryReadOneRouteLogicResponseVariants {
    fn from(value: TryReadOneRouteLogicErrorNamed) -> Self {
        match value.into_serialize_deserialize_version() {
            TryReadOneRouteLogicErrorNamedWithSerializeDeserialize::CheckBodySize { check_body_size, code_occurence } => Self::CheckBodySize { check_body_size, code_occurence },
            TryReadOneRouteLogicErrorNamedWithSerializeDeserialize::Postgresql { postgresql, code_occurence } => Self::Postgresql { postgresql, code_occurence },
            TryReadOneRouteLogicErrorNamedWithSerializeDeserialize::SerdeJson { serde_json, code_occurence } => Self::SerdeJson { serde_json, code_occurence },
            TryReadOneRouteLogicErrorNamedWithSerializeDeserialize::CheckCommit { check_commit, code_occurence } => Self::CheckCommit { check_commit, code_occurence },
            TryReadOneRouteLogicErrorNamedWithSerializeDeserialize::NotUniqueColumn { not_unique_column, code_occurence } => Self::NotUniqueColumn { not_unique_column, code_occurence },
            TryReadOneRouteLogicErrorNamedWithSerializeDeserialize::CheckedAdd { code_occurence } => Self::CheckedAdd { code_occurence },
        }
    }
}
#[derive(Debug, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum TryReadOneRouteLogicErrorNamed {
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
    CheckCommit {
        #[eo_error_occurence]
        check_commit: postgresql_crud::check_commit::CheckCommitErrorNamed,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NotUniqueColumn {
        #[eo_to_std_string_string_serialize_deserialize]
        not_unique_column: ExampleSelect,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    CheckedAdd {
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
pub async fn try_read_one_route_logic(app_state: axum::extract::State<crate::repositories_types::server::routes::app_state::DynArcCombinationOfAppStateLogicTraits>, request: axum::extract::Request) -> axum::response::Response {
    let (parts, body) = request.into_parts();
    let headers = parts.headers;
    let body_bytes = match postgresql_crud::check_body_size::check_body_size(body, *app_state.get_maximum_size_of_http_body_in_bytes()).await {
        Ok(value) => value,
        Err(error_0) => {
            let error = TryReadOneRouteLogicErrorNamed::CheckBodySize {
                check_body_size: error_0,
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_owned(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                        line: 2290,
                        column: 264,
                    }),
                ),
            };
            eprintln!("{error}");
            let mut response = axum::response::IntoResponse::into_response(axum::Json(TryReadOneRouteLogicResponseVariants::from(error)));
            *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
            return response;
        }
    };
    let parameters = ReadOneParameters {
        payload: match serde_json::from_slice::<ReadOnePayload>(&body_bytes) {
            Ok(value) => {
                let value = ReadOnePayload::from(value);
                value
            }
            Err(error_0) => {
                let error = TryReadOneRouteLogicErrorNamed::SerdeJson {
                    serde_json: error_0,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                            line: 2362,
                            column: 249,
                        }),
                    ),
                };
                eprintln!("{error}");
                let mut response = axum::response::IntoResponse::into_response(axum::Json(TryReadOneRouteLogicResponseVariants::from(error)));
                *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
                return response;
            }
        },
    };
    let query_string = {
        let mut increment: std::primitive::u64 = 0;
        format!(
            "select {} from example where {}",
            {
                let mut value = std::string::String::default();
                for element in &parameters.payload.select {
                    value.push_str(&match element {
                        ExampleSelect::Column6E88Acb0C5664Fef8A0966A41338Cf36(value) => <postgresql_crud::postgresql_type::StdPrimitiveI64AsNotNullBigSerialInitializedByPostgresql as postgresql_crud::PostgresqlType>::select_query_part(value, "column_6e88acb0_c566_4fef_8a09_66a41338cf36"),
                        ExampleSelect::AnimalAsNotNullJsonbObject(value) => <AnimalAsNotNullJsonbObject as postgresql_crud::PostgresqlType>::select_query_part(value, "animal_as_not_null_jsonb_object"),
                    });
                    value.push_str(",");
                }
                let _: Option<char> = value.pop();
                value
            },
            match postgresql_crud::PostgresqlTypeWhereFilter::query_part(&parameters.payload.column_6e88acb0_c566_4fef_8a09_66a41338cf36, &mut increment, &"column_6e88acb0_c566_4fef_8a09_66a41338cf36", false) {
                Ok(value) => value,
                Err(error_0) => {
                    let error = TryReadOneRouteLogicErrorNamed::CheckedAdd {
                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                            file!().to_owned(),
                            line!(),
                            column!(),
                            Some(error_occurence_lib::code_occurence::MacroOccurence {
                                file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                                line: 3673,
                                column: 256,
                            }),
                        ),
                    };
                    eprintln!("{error}");
                    let mut response = axum::response::IntoResponse::into_response(axum::Json(TryReadOneRouteLogicResponseVariants::from(error)));
                    *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
                    return response;
                }
            }
        )
    };
    println!("{}", query_string);
    let binded_query = {
        let query = sqlx::query::<sqlx::Postgres>(&query_string);
        let query = postgresql_crud::PostgresqlTypeWhereFilter::query_bind(parameters.payload.column_6e88acb0_c566_4fef_8a09_66a41338cf36, query);
        query
    };
    let mut pool_connection = match app_state.get_postgres_pool().acquire().await {
        Ok(value) => value,
        Err(error_0) => {
            let error = TryReadOneRouteLogicErrorNamed::Postgresql {
                postgresql: error_0,
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_owned(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                        line: 2312,
                        column: 253,
                    }),
                ),
            };
            eprintln!("{error}");
            let mut response = axum::response::IntoResponse::into_response(axum::Json(TryReadOneRouteLogicResponseVariants::from(error)));
            *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
            return response;
        }
    };
    let executor = match sqlx::Acquire::acquire(&mut pool_connection).await {
        Ok(value) => value,
        Err(error_0) => {
            let error = TryReadOneRouteLogicErrorNamed::Postgresql {
                postgresql: error_0,
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_owned(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                        line: 2312,
                        column: 253,
                    }),
                ),
            };
            eprintln!("{error}");
            let mut response = axum::response::IntoResponse::into_response(axum::Json(TryReadOneRouteLogicResponseVariants::from(error)));
            *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
            return response;
        }
    };
    let value = {
        let value = {
            match binded_query.fetch_one(executor.as_mut()).await {
                Ok(value) => {
                    let mut column_6e88acb0_c566_4fef_8a09_66a41338cf36: std::option::Option<postgresql_crud::Value<<postgresql_crud::postgresql_type::StdPrimitiveI64AsNotNullBigSerialInitializedByPostgresql as postgresql_crud::PostgresqlTypePrimaryKey>::PrimaryKey>> = None;
                    let mut animal_as_not_null_jsonb_object: std::option::Option<postgresql_crud::Value<<AnimalAsNotNullJsonbObject as postgresql_crud::PostgresqlType>::Read>> = None;
                    for element in &parameters.payload.select {
                        match element {
                            ExampleSelect::Column6E88Acb0C5664Fef8A0966A41338Cf36(_) => {
                                match sqlx::Row::try_get::<<postgresql_crud::postgresql_type::StdPrimitiveI64AsNotNullBigSerialInitializedByPostgresql as postgresql_crud::PostgresqlTypePrimaryKey>::PrimaryKey, &std::primitive::str>(&value, "column_6e88acb0_c566_4fef_8a09_66a41338cf36") {
                                    Ok(value) => {
                                        column_6e88acb0_c566_4fef_8a09_66a41338cf36 = Some(postgresql_crud::Value { value: value });
                                    }
                                    Err(error_0) => {
                                        let error = TryReadOneRouteLogicErrorNamed::Postgresql {
                                            postgresql: error_0,
                                            code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                                file!().to_owned(),
                                                line!(),
                                                column!(),
                                                Some(error_occurence_lib::code_occurence::MacroOccurence {
                                                    file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                                                    line: 1186,
                                                    column: 245,
                                                }),
                                            ),
                                        };
                                        eprintln!("{error}");
                                        let mut response = axum::response::IntoResponse::into_response(axum::Json(TryReadOneRouteLogicResponseVariants::from(error)));
                                        *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                                        return response;
                                    }
                                }
                            }
                            ExampleSelect::AnimalAsNotNullJsonbObject(_) => match sqlx::Row::try_get::<<AnimalAsNotNullJsonbObject as postgresql_crud::PostgresqlType>::Read, &std::primitive::str>(&value, "animal_as_not_null_jsonb_object") {
                                Ok(value) => {
                                    animal_as_not_null_jsonb_object = Some(postgresql_crud::Value { value: value });
                                }
                                Err(error_0) => {
                                    let error = TryReadOneRouteLogicErrorNamed::Postgresql {
                                        postgresql: error_0,
                                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                            file!().to_owned(),
                                            line!(),
                                            column!(),
                                            Some(error_occurence_lib::code_occurence::MacroOccurence {
                                                file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                                                line: 1223,
                                                column: 249,
                                            }),
                                        ),
                                    };
                                    eprintln!("{error}");
                                    let mut response = axum::response::IntoResponse::into_response(axum::Json(TryReadOneRouteLogicResponseVariants::from(error)));
                                    *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                                    return response;
                                }
                            },
                        }
                    }
                    ExampleOptions {
                        column_6e88acb0_c566_4fef_8a09_66a41338cf36,
                        animal_as_not_null_jsonb_object,
                    }
                }
                Err(error_0) => {
                    let error = TryReadOneRouteLogicErrorNamed::Postgresql {
                        postgresql: error_0,
                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                            file!().to_owned(),
                            line!(),
                            column!(),
                            Some(error_occurence_lib::code_occurence::MacroOccurence {
                                file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                                line: 3710,
                                column: 169,
                            }),
                        ),
                    };
                    eprintln!("{error}");
                    let mut response = axum::response::IntoResponse::into_response(axum::Json(TryReadOneRouteLogicResponseVariants::from(error)));
                    *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                    return response;
                }
            }
        };
        value
    };
    let mut response = axum::response::IntoResponse::into_response(axum::Json(TryReadOneRouteLogicResponseVariants::Desirable(value)));
    *response.status_mut() = axum::http::StatusCode::OK;
    return response;
}
#[derive(Debug, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum TryReadOneErrorNamed {
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
    NotUniqueColumn {
        #[eo_to_std_string_string_serialize_deserialize]
        not_unique_column: ExampleSelect,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    TryReadOneRouteLogicErrorNamedWithSerializeDeserialize {
        #[eo_to_std_string_string]
        try_read_one_route_logic_error_named_with_serialize_deserialize: TryReadOneRouteLogicErrorNamedWithSerializeDeserialize,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
pub async fn try_read_one(server_location: &std::primitive::str, parameters: ReadOneParameters) -> Result<ExampleOptions, TryReadOneErrorNamed> {
    let payload = {
        let value = ReadOnePayload::from(parameters.payload);
        match serde_json::to_string(&value) {
            Ok(value) => value,
            Err(error_0) => {
                return Err(TryReadOneErrorNamed::SerdeJsonToString {
                    serde_json_to_string: error_0,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                            line: 2405,
                            column: 178,
                        }),
                    ),
                });
            }
        }
    };
    let url = format!("{}/example/read_one", server_location,);
    let future = reqwest::Client::new()
        .post(&url)
        .header(&postgresql_crud::CommitSnakeCase.to_string(), git_info::PROJECT_GIT_INFO.commit)
        .header(reqwest::header::CONTENT_TYPE, "application/json")
        .body(payload)
        .send();
    let response = match future.await {
        Ok(value) => value,
        Err(error_0) => {
            return Err(TryReadOneErrorNamed::Reqwest {
                reqwest: error_0,
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_owned(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                        line: 2457,
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
            return Err(TryReadOneErrorNamed::FailedToGetResponseText {
                status_code: error_0,
                headers: error_1,
                reqwest: error_2,
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_owned(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                        line: 2474,
                        column: 192,
                    }),
                ),
            });
        }
    };
    let expected_response = match serde_json::from_str::<TryReadOneRouteLogicResponseVariants>(&error_2) {
        Ok(value) => value,
        Err(error_3) => {
            return Err(TryReadOneErrorNamed::DeserializeResponse {
                status_code: error_0,
                headers: error_1,
                response_text: error_2,
                serde: error_3,
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_owned(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                        line: 2487,
                        column: 178,
                    }),
                ),
            });
        }
    };
    let try_read_one_route_logic_error_named_with_serialize_deserialize = match expected_response {
        TryReadOneRouteLogicResponseVariants::Desirable(value) => {
            let value = value;
            return Ok(value);
        }
        TryReadOneRouteLogicResponseVariants::CheckBodySize { check_body_size, code_occurence } => TryReadOneRouteLogicErrorNamedWithSerializeDeserialize::CheckBodySize { check_body_size, code_occurence },
        TryReadOneRouteLogicResponseVariants::Postgresql { postgresql, code_occurence } => TryReadOneRouteLogicErrorNamedWithSerializeDeserialize::Postgresql { postgresql, code_occurence },
        TryReadOneRouteLogicResponseVariants::SerdeJson { serde_json, code_occurence } => TryReadOneRouteLogicErrorNamedWithSerializeDeserialize::SerdeJson { serde_json, code_occurence },
        TryReadOneRouteLogicResponseVariants::CheckCommit { check_commit, code_occurence } => TryReadOneRouteLogicErrorNamedWithSerializeDeserialize::CheckCommit { check_commit, code_occurence },
        TryReadOneRouteLogicResponseVariants::NotUniqueColumn { not_unique_column, code_occurence } => TryReadOneRouteLogicErrorNamedWithSerializeDeserialize::NotUniqueColumn { not_unique_column, code_occurence },
        TryReadOneRouteLogicResponseVariants::CheckedAdd { code_occurence } => TryReadOneRouteLogicErrorNamedWithSerializeDeserialize::CheckedAdd { code_occurence },
    };
    Err(TryReadOneErrorNamed::TryReadOneRouteLogicErrorNamedWithSerializeDeserialize {
        try_read_one_route_logic_error_named_with_serialize_deserialize,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
            file!().to_owned(),
            line!(),
            column!(),
            Some(error_occurence_lib::code_occurence::MacroOccurence {
                file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                line: 2525,
                column: 223,
            }),
        ),
    })
}
impl postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for ReadOnePayload {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self {
            column_6e88acb0_c566_4fef_8a09_66a41338cf36: postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            select: postgresql_crud::AllEnumVariantsArrayDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::all_enum_variants_array_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element(),
        }
    }
}
pub async fn read_one_payload_example_route_logic() -> axum::response::Response {
    let mut response = axum::response::IntoResponse::into_response(axum::Json(<ReadOnePayload as postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement>::default_but_option_is_always_some_and_vec_always_contains_one_element()));
    *response.status_mut() = axum::http::StatusCode::OK;
    return response;
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema)]
pub struct UpdateManyPayloadElement {
    pub column_6e88acb0_c566_4fef_8a09_66a41338cf36: postgresql_crud::postgresql_type::StdPrimitiveI64AsNotNullBigSerialInitializedByPostgresqlUpdate,
    pub animal_as_not_null_jsonb_object: std::option::Option<postgresql_crud::Value<<AnimalAsNotNullJsonbObject as postgresql_crud::PostgresqlType>::Update>>,
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema)]
pub struct UpdateManyPayload(pub std::vec::Vec<UpdateManyPayloadElement>);
#[derive(Debug)]
pub struct UpdateManyParameters {
    pub payload: UpdateManyPayload,
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize)]
pub enum TryUpdateManyRouteLogicResponseVariants {
    Desirable(std::vec::Vec<<postgresql_crud::postgresql_type::StdPrimitiveI64AsNotNullBigSerialInitializedByPostgresql as postgresql_crud::PostgresqlTypePrimaryKey>::PrimaryKey>),
    CheckBodySize {
        check_body_size: postgresql_crud::check_body_size::CheckBodySizeErrorNamedWithSerializeDeserialize,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Postgresql {
        postgresql: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    SerdeJson {
        serde_json: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    CheckCommit {
        check_commit: postgresql_crud::check_commit::CheckCommitErrorNamedWithSerializeDeserialize,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    RowAndRollback {
        row: std::string::String,
        rollback: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NonExistingPrimaryKeys {
        non_existing_primary_keys: std::vec::Vec<std::string::String>,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NonExistingPrimaryKeysAndRollback {
        non_existing_primary_keys: std::vec::Vec<std::string::String>,
        rollback: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NotUniquePrimaryKey {
        not_unique_primary_key: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NoPayloadFieldsPrimaryKey {
        no_payload_fields_primary_key: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    CheckedAdd {
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
impl std::convert::From<TryUpdateManyRouteLogicErrorNamed> for TryUpdateManyRouteLogicResponseVariants {
    fn from(value: TryUpdateManyRouteLogicErrorNamed) -> Self {
        match value.into_serialize_deserialize_version() {
            TryUpdateManyRouteLogicErrorNamedWithSerializeDeserialize::CheckBodySize { check_body_size, code_occurence } => Self::CheckBodySize { check_body_size, code_occurence },
            TryUpdateManyRouteLogicErrorNamedWithSerializeDeserialize::Postgresql { postgresql, code_occurence } => Self::Postgresql { postgresql, code_occurence },
            TryUpdateManyRouteLogicErrorNamedWithSerializeDeserialize::SerdeJson { serde_json, code_occurence } => Self::SerdeJson { serde_json, code_occurence },
            TryUpdateManyRouteLogicErrorNamedWithSerializeDeserialize::CheckCommit { check_commit, code_occurence } => Self::CheckCommit { check_commit, code_occurence },
            TryUpdateManyRouteLogicErrorNamedWithSerializeDeserialize::RowAndRollback { row, rollback, code_occurence } => Self::RowAndRollback { row, rollback, code_occurence },
            TryUpdateManyRouteLogicErrorNamedWithSerializeDeserialize::NonExistingPrimaryKeys { non_existing_primary_keys, code_occurence } => Self::NonExistingPrimaryKeys { non_existing_primary_keys, code_occurence },
            TryUpdateManyRouteLogicErrorNamedWithSerializeDeserialize::NonExistingPrimaryKeysAndRollback { non_existing_primary_keys, rollback, code_occurence } => Self::NonExistingPrimaryKeysAndRollback { non_existing_primary_keys, rollback, code_occurence },
            TryUpdateManyRouteLogicErrorNamedWithSerializeDeserialize::NotUniquePrimaryKey { not_unique_primary_key, code_occurence } => Self::NotUniquePrimaryKey { not_unique_primary_key, code_occurence },
            TryUpdateManyRouteLogicErrorNamedWithSerializeDeserialize::NoPayloadFieldsPrimaryKey { no_payload_fields_primary_key, code_occurence } => Self::NoPayloadFieldsPrimaryKey { no_payload_fields_primary_key, code_occurence },
            TryUpdateManyRouteLogicErrorNamedWithSerializeDeserialize::CheckedAdd { code_occurence } => Self::CheckedAdd { code_occurence },
        }
    }
}
#[derive(Debug, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum TryUpdateManyRouteLogicErrorNamed {
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
    NonExistingPrimaryKeys {
        #[eo_vec_to_std_string_string]
        non_existing_primary_keys: std::vec::Vec<postgresql_crud::postgresql_type::StdPrimitiveI64AsNotNullBigSerialInitializedByPostgresqlRead>,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NonExistingPrimaryKeysAndRollback {
        #[eo_vec_to_std_string_string]
        non_existing_primary_keys: std::vec::Vec<postgresql_crud::postgresql_type::StdPrimitiveI64AsNotNullBigSerialInitializedByPostgresqlRead>,
        #[eo_to_std_string_string]
        rollback: sqlx::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NotUniquePrimaryKey {
        #[eo_to_std_string_string]
        not_unique_primary_key: postgresql_crud::postgresql_type::StdPrimitiveI64AsNotNullBigSerialInitializedByPostgresqlUpdate,//was Read - fix todo
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NoPayloadFieldsPrimaryKey {
        #[eo_to_std_string_string]
        no_payload_fields_primary_key: postgresql_crud::postgresql_type::StdPrimitiveI64AsNotNullBigSerialInitializedByPostgresqlUpdate,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    CheckedAdd {
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
pub async fn try_update_many_route_logic(app_state: axum::extract::State<crate::repositories_types::server::routes::app_state::DynArcCombinationOfAppStateLogicTraits>, request: axum::extract::Request) -> axum::response::Response {
    let (parts, body) = request.into_parts();
    let headers = parts.headers;
    let body_bytes = match postgresql_crud::check_body_size::check_body_size(body, *app_state.get_maximum_size_of_http_body_in_bytes()).await {
        Ok(value) => value,
        Err(error_0) => {
            let error = TryUpdateManyRouteLogicErrorNamed::CheckBodySize {
                check_body_size: error_0,
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_owned(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                        line: 2290,
                        column: 264,
                    }),
                ),
            };
            eprintln!("{error}");
            let mut response = axum::response::IntoResponse::into_response(axum::Json(TryUpdateManyRouteLogicResponseVariants::from(error)));
            *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
            return response;
        }
    };
    let parameters = UpdateManyParameters {
        payload: match serde_json::from_slice::<UpdateManyPayload>(&body_bytes) {
            Ok(value) => {
                let value = UpdateManyPayload::from(value);
                {
                    let mut acc = std::vec::Vec::new();
                    for element in &value.0 {
                        if !acc.contains(&&element.column_6e88acb0_c566_4fef_8a09_66a41338cf36) {
                            acc.push(&element.column_6e88acb0_c566_4fef_8a09_66a41338cf36);
                        } else {
                            let error_0 = element.column_6e88acb0_c566_4fef_8a09_66a41338cf36.clone();
                            let error = TryUpdateManyRouteLogicErrorNamed::NotUniquePrimaryKey {
                                not_unique_primary_key: error_0,
                                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                    file!().to_owned(),
                                    line!(),
                                    column!(),
                                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                                        file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                                        line: 3918,
                                        column: 185,
                                    }),
                                ),
                            };
                            eprintln!("{error}");
                            let mut response = axum::response::IntoResponse::into_response(axum::Json(TryUpdateManyRouteLogicResponseVariants::from(error)));
                            *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
                            return response;
                        }
                    }
                }
                for element in &value.0 {
                    if let None = &element.animal_as_not_null_jsonb_object {
                        let error_0 = element.column_6e88acb0_c566_4fef_8a09_66a41338cf36.clone();
                        let error = TryUpdateManyRouteLogicErrorNamed::NoPayloadFieldsPrimaryKey {
                            no_payload_fields_primary_key: error_0,
                            code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                file!().to_owned(),
                                line!(),
                                column!(),
                                Some(error_occurence_lib::code_occurence::MacroOccurence {
                                    file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                                    line: 1906,
                                    column: 287,
                                }),
                            ),
                        };
                        eprintln!("{error}");
                        let mut response = axum::response::IntoResponse::into_response(axum::Json(TryUpdateManyRouteLogicResponseVariants::from(error)));
                        *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                        return response;
                    }
                }
                value
            }
            Err(error_0) => {
                let error = TryUpdateManyRouteLogicErrorNamed::SerdeJson {
                    serde_json: error_0,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                            line: 2362,
                            column: 249,
                        }),
                    ),
                };
                eprintln!("{error}");
                let mut response = axum::response::IntoResponse::into_response(axum::Json(TryUpdateManyRouteLogicResponseVariants::from(error)));
                *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
                return response;
            }
        },
    };
    // let expected_primary_keys = parameters
    //     .payload
    //     .0
    //     .iter()
    //     .map(|element| element.column_6e88acb0_c566_4fef_8a09_66a41338cf36.clone())
    //     .collect::<std::vec::Vec<<postgresql_crud::postgresql_type::StdPrimitiveI64AsNotNullBigSerialInitializedByPostgresql as postgresql_crud::PostgresqlTypePrimaryKey>::PrimaryKey>>();
    //todo
    let query_string = {
        let mut query = std::string::String::from("update example set ");
        let mut increment: std::primitive::u64 = 0;
        {
            let mut is_animal_as_not_null_jsonb_object_update_exist = false;
            for element in &parameters.payload.0 {
                if element.animal_as_not_null_jsonb_object.is_some() {
                    is_animal_as_not_null_jsonb_object_update_exist = true;
                    break;
                }
            }
            if is_animal_as_not_null_jsonb_object_update_exist {
                let mut acc = std::string::String::from("animal_as_not_null_jsonb_object = case ");
                for element in &parameters.payload.0 {
                    if let Some(value) = &element.animal_as_not_null_jsonb_object {
                        acc.push_str(&format!(
                            "when column_6e88acb0_c566_4fef_8a09_66a41338cf36 = {} then {} ",
                            match <postgresql_crud::postgresql_type::StdPrimitiveI64AsNotNullBigSerialInitializedByPostgresql as postgresql_crud::PostgresqlType>::update_query_part(
                                &element.column_6e88acb0_c566_4fef_8a09_66a41338cf36,
                                &"",
                                &"column_6e88acb0_c566_4fef_8a09_66a41338cf36",
                                &"",
                                &mut increment,
                            ) {
                                Ok(value) => value,
                                Err(error_0) => {
                                    let error = TryUpdateManyRouteLogicErrorNamed::CheckedAdd {
                                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                            file!().to_owned(),
                                            line!(),
                                            column!(),
                                            Some(error_occurence_lib::code_occurence::MacroOccurence {
                                                file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                                                line: 3958,
                                                column: 260,
                                            }),
                                        ),
                                    };
                                    eprintln!("{error}");
                                    let mut response = axum::response::IntoResponse::into_response(axum::Json(TryUpdateManyRouteLogicResponseVariants::from(error)));
                                    *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
                                    return response;
                                }
                            },
                            match <AnimalAsNotNullJsonbObject as postgresql_crud::PostgresqlType>::update_query_part(&value.value, &"", &"animal_as_not_null_jsonb_object", &"", &mut increment,) {
                                Ok(value) => value,
                                Err(error_0) => {
                                    let error = TryUpdateManyRouteLogicErrorNamed::CheckedAdd {
                                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                            file!().to_owned(),
                                            line!(),
                                            column!(),
                                            Some(error_occurence_lib::code_occurence::MacroOccurence {
                                                file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                                                line: 3958,
                                                column: 260,
                                            }),
                                        ),
                                    };
                                    eprintln!("{error}");
                                    let mut response = axum::response::IntoResponse::into_response(axum::Json(TryUpdateManyRouteLogicResponseVariants::from(error)));
                                    *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
                                    return response;
                                }
                            }
                        ));
                    }
                }
                query.push_str(&format!("{}{}", acc, "else animal_as_not_null_jsonb_object end,"));
            }
        }
        let _: Option<char> = query.pop();
        query.push_str(&format!(" where column_6e88acb0_c566_4fef_8a09_66a41338cf36 in ({}) returning column_6e88acb0_c566_4fef_8a09_66a41338cf36;", {
            let mut acc = std::string::String::default();
            for element in &parameters.payload.0 {
                match <postgresql_crud::postgresql_type::StdPrimitiveI64AsNotNullBigSerialInitializedByPostgresql as postgresql_crud::PostgresqlType>::update_query_part(&element.column_6e88acb0_c566_4fef_8a09_66a41338cf36, &"", &"column_6e88acb0_c566_4fef_8a09_66a41338cf36", &"", &mut increment) {
                    Ok(value) => {
                        acc.push_str(&format!("{value},"));
                    }
                    Err(error_0) => {
                        let error = TryUpdateManyRouteLogicErrorNamed::CheckedAdd {
                            code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                file!().to_owned(),
                                line!(),
                                column!(),
                                Some(error_occurence_lib::code_occurence::MacroOccurence {
                                    file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                                    line: 4012,
                                    column: 260,
                                }),
                            ),
                        };
                        eprintln!("{error}");
                        let mut response = axum::response::IntoResponse::into_response(axum::Json(TryUpdateManyRouteLogicResponseVariants::from(error)));
                        *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
                        return response;
                    }
                }
            }
            let _: Option<char> = acc.pop();
            acc
        }));
        query
    };
    println!("{}", query_string);
    let binded_query = {
        let mut query = sqlx::query::<sqlx::Postgres>(&query_string);
        {
            let mut is_animal_as_not_null_jsonb_object_update_exist = false;
            for element in &parameters.payload.0 {
                if element.animal_as_not_null_jsonb_object.is_some() {
                    is_animal_as_not_null_jsonb_object_update_exist = true;
                    break;
                }
            }
            if is_animal_as_not_null_jsonb_object_update_exist {
                for element in &parameters.payload.0 {
                    if let Some(value) = &element.animal_as_not_null_jsonb_object {
                        query = query.bind(element.column_6e88acb0_c566_4fef_8a09_66a41338cf36.clone());
                        query = <AnimalAsNotNullJsonbObject as postgresql_crud::PostgresqlType>::update_query_bind(value.value.clone(), query);
                    }
                }
            }
        }
        {
            for element in &parameters.payload.0 {
                query = <postgresql_crud::postgresql_type::StdPrimitiveI64AsNotNullBigSerialInitializedByPostgresql as postgresql_crud::PostgresqlType>::update_query_bind(element.column_6e88acb0_c566_4fef_8a09_66a41338cf36.clone(), query);
            }
        }
        query
    };
    let mut pool_connection = match app_state.get_postgres_pool().acquire().await {
        Ok(value) => value,
        Err(error_0) => {
            let error = TryUpdateManyRouteLogicErrorNamed::Postgresql {
                postgresql: error_0,
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_owned(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                        line: 2312,
                        column: 253,
                    }),
                ),
            };
            eprintln!("{error}");
            let mut response = axum::response::IntoResponse::into_response(axum::Json(TryUpdateManyRouteLogicResponseVariants::from(error)));
            *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
            return response;
        }
    };
    let executor = match sqlx::Acquire::acquire(&mut pool_connection).await {
        Ok(value) => value,
        Err(error_0) => {
            let error = TryUpdateManyRouteLogicErrorNamed::Postgresql {
                postgresql: error_0,
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_owned(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                        line: 2312,
                        column: 253,
                    }),
                ),
            };
            eprintln!("{error}");
            let mut response = axum::response::IntoResponse::into_response(axum::Json(TryUpdateManyRouteLogicResponseVariants::from(error)));
            *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
            return response;
        }
    };
    let value = {
        let mut executor = match sqlx::Acquire::begin(executor).await {
            Ok(value) => value,
            Err(error_0) => {
                let error = TryUpdateManyRouteLogicErrorNamed::Postgresql {
                    postgresql: error_0,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                            line: 2059,
                            column: 245,
                        }),
                    ),
                };
                eprintln!("{error}");
                let mut response = axum::response::IntoResponse::into_response(axum::Json(TryUpdateManyRouteLogicResponseVariants::from(error)));
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
                    Some(value) => match sqlx::Row::try_get::<<postgresql_crud::postgresql_type::StdPrimitiveI64AsNotNullBigSerialInitializedByPostgresql as postgresql_crud::PostgresqlTypePrimaryKey>::PrimaryKey, &std::primitive::str>(&value, "column_6e88acb0_c566_4fef_8a09_66a41338cf36") {
                        Ok(value) => Some(value),
                        Err(error_0) => {
                            drop(rows);
                            match executor.rollback().await {
                                Ok(_) => {
                                    let error = TryUpdateManyRouteLogicErrorNamed::Postgresql {
                                        postgresql: error_0,
                                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                            file!().to_owned(),
                                            line!(),
                                            column!(),
                                            Some(error_occurence_lib::code_occurence::MacroOccurence {
                                                file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                                                line: 2693,
                                                column: 128,
                                            }),
                                        ),
                                    };
                                    eprintln!("{error}");
                                    let mut response = axum::response::IntoResponse::into_response(axum::Json(TryUpdateManyRouteLogicResponseVariants::from(error)));
                                    *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                                    return response;
                                }
                                Err(error_1) => {
                                    let error = TryUpdateManyRouteLogicErrorNamed::RowAndRollback {
                                        row: error_0,
                                        rollback: error_1,
                                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                            file!().to_owned(),
                                            line!(),
                                            column!(),
                                            Some(error_occurence_lib::code_occurence::MacroOccurence {
                                                file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                                                line: 2693,
                                                column: 157,
                                            }),
                                        ),
                                    };
                                    eprintln!("{error}");
                                    let mut response = axum::response::IntoResponse::into_response(axum::Json(TryUpdateManyRouteLogicResponseVariants::from(error)));
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
                            let error = TryUpdateManyRouteLogicErrorNamed::Postgresql {
                                postgresql: error_0,
                                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                    file!().to_owned(),
                                    line!(),
                                    column!(),
                                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                                        file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                                        line: 2695,
                                        column: 124,
                                    }),
                                ),
                            };
                            eprintln!("{error}");
                            let mut response = axum::response::IntoResponse::into_response(axum::Json(TryUpdateManyRouteLogicResponseVariants::from(error)));
                            *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                            return response;
                        }
                        Err(error_1) => {
                            let error = TryUpdateManyRouteLogicErrorNamed::RowAndRollback {
                                row: error_0,
                                rollback: error_1,
                                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                    file!().to_owned(),
                                    line!(),
                                    column!(),
                                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                                        file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                                        line: 2695,
                                        column: 153,
                                    }),
                                ),
                            };
                            eprintln!("{error}");
                            let mut response = axum::response::IntoResponse::into_response(axum::Json(TryUpdateManyRouteLogicResponseVariants::from(error)));
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
        // {
        //     let error_0 = expected_primary_keys.into_iter().fold(std::vec::Vec::new(), |mut acc, element| {
        //         if let false = value.contains(&element) {
        //             acc.push(element);
        //         }
        //         acc
        //     });
        //     if let false = error_0.is_empty() {
        //         match executor.rollback().await {
        //             Ok(_) => {
        //                 let error = TryUpdateManyRouteLogicErrorNamed::NonExistingPrimaryKeys {
        //                     non_existing_primary_keys: error_0,
        //                     code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
        //                         file!().to_owned(),
        //                         line!(),
        //                         column!(),
        //                         Some(error_occurence_lib::code_occurence::MacroOccurence {
        //                             file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
        //                             line: 1961,
        //                             column: 13,
        //                         }),
        //                     ),
        //                 };
        //                 eprintln!("{error}");
        //                 let mut response = axum::response::IntoResponse::into_response(axum::Json(TryUpdateManyRouteLogicResponseVariants::from(error)));
        //                 *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
        //                 return response;
        //             }
        //             Err(error_1) => {
        //                 let error = TryUpdateManyRouteLogicErrorNamed::NonExistingPrimaryKeysAndRollback {
        //                     non_existing_primary_keys: error_0,
        //                     rollback: error_1,
        //                     code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
        //                         file!().to_owned(),
        //                         line!(),
        //                         column!(),
        //                         Some(error_occurence_lib::code_occurence::MacroOccurence {
        //                             file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
        //                             line: 1977,
        //                             column: 13,
        //                         }),
        //                     ),
        //                 };
        //                 eprintln!("{error}");
        //                 let mut response = axum::response::IntoResponse::into_response(axum::Json(TryUpdateManyRouteLogicResponseVariants::from(error)));
        //                 *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
        //                 return response;
        //             }
        //         }
        //     }
        // }
        if let Err(error_0) = executor.commit().await {
            let error = TryUpdateManyRouteLogicErrorNamed::Postgresql {
                postgresql: error_0,
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_owned(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                        line: 2071,
                        column: 245,
                    }),
                ),
            };
            eprintln!("{error}");
            let mut response = axum::response::IntoResponse::into_response(axum::Json(TryUpdateManyRouteLogicResponseVariants::from(error)));
            *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
            return response;
        }
        value
    };
    let mut response = axum::response::IntoResponse::into_response(axum::Json(TryUpdateManyRouteLogicResponseVariants::Desirable(value)));
    *response.status_mut() = axum::http::StatusCode::OK;
    return response;
}
#[derive(Debug, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum TryUpdateManyErrorNamed {
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
    NotUniquePrimaryKey {
        #[eo_to_std_string_string]
        not_unique_primary_key: postgresql_crud::postgresql_type::StdPrimitiveI64AsNotNullBigSerialInitializedByPostgresqlRead,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    TryUpdateManyRouteLogicErrorNamedWithSerializeDeserialize {
        #[eo_to_std_string_string]
        try_update_many_route_logic_error_named_with_serialize_deserialize: TryUpdateManyRouteLogicErrorNamedWithSerializeDeserialize,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
pub async fn try_update_many(
    server_location: &std::primitive::str,
    parameters: UpdateManyParameters,
) -> Result<std::vec::Vec<<postgresql_crud::postgresql_type::StdPrimitiveI64AsNotNullBigSerialInitializedByPostgresql as postgresql_crud::PostgresqlTypePrimaryKey>::PrimaryKey>, TryUpdateManyErrorNamed> {
    let payload = {
        let mut acc = std::vec::Vec::new();
        for element in &parameters.payload.0 {
            if !acc.contains(&&element.column_6e88acb0_c566_4fef_8a09_66a41338cf36) {
                acc.push(&element.column_6e88acb0_c566_4fef_8a09_66a41338cf36);
            } else {
                // let error_0 = element.column_6e88acb0_c566_4fef_8a09_66a41338cf36.clone();
                // return Err(TryUpdateManyErrorNamed::NotUniquePrimaryKey {
                //     not_unique_primary_key: error_0,
                //     code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                //         file!().to_owned(),
                //         line!(),
                //         column!(),
                //         Some(error_occurence_lib::code_occurence::MacroOccurence {
                //             file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                //             line: 4153,
                //             column: 203,
                //         }),
                //     ),
                // });
                todo!()
            }
        }
        let value = UpdateManyPayload::from(parameters.payload);
        match serde_json::to_string(&value) {
            Ok(value) => value,
            Err(error_0) => {
                return Err(TryUpdateManyErrorNamed::SerdeJsonToString {
                    serde_json_to_string: error_0,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                            line: 2405,
                            column: 178,
                        }),
                    ),
                });
            }
        }
    };
    let url = format!("{}/example/update_many", server_location,);
    let future = reqwest::Client::new()
        .patch(&url)
        .header(&postgresql_crud::CommitSnakeCase.to_string(), git_info::PROJECT_GIT_INFO.commit)
        .header(reqwest::header::CONTENT_TYPE, "application/json")
        .body(payload)
        .send();
    let response = match future.await {
        Ok(value) => value,
        Err(error_0) => {
            return Err(TryUpdateManyErrorNamed::Reqwest {
                reqwest: error_0,
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_owned(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                        line: 2457,
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
            return Err(TryUpdateManyErrorNamed::FailedToGetResponseText {
                status_code: error_0,
                headers: error_1,
                reqwest: error_2,
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_owned(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                        line: 2474,
                        column: 192,
                    }),
                ),
            });
        }
    };
    let expected_response = match serde_json::from_str::<TryUpdateManyRouteLogicResponseVariants>(&error_2) {
        Ok(value) => value,
        Err(error_3) => {
            return Err(TryUpdateManyErrorNamed::DeserializeResponse {
                status_code: error_0,
                headers: error_1,
                response_text: error_2,
                serde: error_3,
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_owned(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                        line: 2487,
                        column: 178,
                    }),
                ),
            });
        }
    };
    let try_update_many_route_logic_error_named_with_serialize_deserialize = match expected_response {
        TryUpdateManyRouteLogicResponseVariants::Desirable(value) => {
            let value = value;
            return Ok(value);
        }
        TryUpdateManyRouteLogicResponseVariants::CheckBodySize { check_body_size, code_occurence } => TryUpdateManyRouteLogicErrorNamedWithSerializeDeserialize::CheckBodySize { check_body_size, code_occurence },
        TryUpdateManyRouteLogicResponseVariants::Postgresql { postgresql, code_occurence } => TryUpdateManyRouteLogicErrorNamedWithSerializeDeserialize::Postgresql { postgresql, code_occurence },
        TryUpdateManyRouteLogicResponseVariants::SerdeJson { serde_json, code_occurence } => TryUpdateManyRouteLogicErrorNamedWithSerializeDeserialize::SerdeJson { serde_json, code_occurence },
        TryUpdateManyRouteLogicResponseVariants::CheckCommit { check_commit, code_occurence } => TryUpdateManyRouteLogicErrorNamedWithSerializeDeserialize::CheckCommit { check_commit, code_occurence },
        TryUpdateManyRouteLogicResponseVariants::RowAndRollback { row, rollback, code_occurence } => TryUpdateManyRouteLogicErrorNamedWithSerializeDeserialize::RowAndRollback { row, rollback, code_occurence },
        TryUpdateManyRouteLogicResponseVariants::NonExistingPrimaryKeys { non_existing_primary_keys, code_occurence } => TryUpdateManyRouteLogicErrorNamedWithSerializeDeserialize::NonExistingPrimaryKeys { non_existing_primary_keys, code_occurence },
        TryUpdateManyRouteLogicResponseVariants::NonExistingPrimaryKeysAndRollback { non_existing_primary_keys, rollback, code_occurence } => TryUpdateManyRouteLogicErrorNamedWithSerializeDeserialize::NonExistingPrimaryKeysAndRollback { non_existing_primary_keys, rollback, code_occurence },
        TryUpdateManyRouteLogicResponseVariants::NotUniquePrimaryKey { not_unique_primary_key, code_occurence } => TryUpdateManyRouteLogicErrorNamedWithSerializeDeserialize::NotUniquePrimaryKey { not_unique_primary_key, code_occurence },
        TryUpdateManyRouteLogicResponseVariants::NoPayloadFieldsPrimaryKey { no_payload_fields_primary_key, code_occurence } => TryUpdateManyRouteLogicErrorNamedWithSerializeDeserialize::NoPayloadFieldsPrimaryKey { no_payload_fields_primary_key, code_occurence },
        TryUpdateManyRouteLogicResponseVariants::CheckedAdd { code_occurence } => TryUpdateManyRouteLogicErrorNamedWithSerializeDeserialize::CheckedAdd { code_occurence },
    };
    Err(TryUpdateManyErrorNamed::TryUpdateManyRouteLogicErrorNamedWithSerializeDeserialize {
        try_update_many_route_logic_error_named_with_serialize_deserialize,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
            file!().to_owned(),
            line!(),
            column!(),
            Some(error_occurence_lib::code_occurence::MacroOccurence {
                file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                line: 2525,
                column: 223,
            }),
        ),
    })
}
impl postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for UpdateManyPayloadElement {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self {
            column_6e88acb0_c566_4fef_8a09_66a41338cf36: postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            animal_as_not_null_jsonb_object: Some(postgresql_crud::Value {
                value: postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            }),
        }
    }
}
impl postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for UpdateManyPayload {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self(vec![postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()])
    }
}
pub async fn update_many_payload_example_route_logic() -> axum::response::Response {
    let mut response = axum::response::IntoResponse::into_response(axum::Json(
        <UpdateManyPayload as postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement>::default_but_option_is_always_some_and_vec_always_contains_one_element(),
    ));
    *response.status_mut() = axum::http::StatusCode::OK;
    return response;
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema)]
pub struct UpdateOnePayload {
    pub column_6e88acb0_c566_4fef_8a09_66a41338cf36: postgresql_crud::postgresql_type::StdPrimitiveI64AsNotNullBigSerialInitializedByPostgresqlUpdate,
    pub animal_as_not_null_jsonb_object: std::option::Option<postgresql_crud::Value<<AnimalAsNotNullJsonbObject as postgresql_crud::PostgresqlType>::Update>>,
}
#[derive(Debug)]
pub struct UpdateOneParameters {
    pub payload: UpdateOnePayload,
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize)]
pub enum TryUpdateOneRouteLogicResponseVariants {
    Desirable(<postgresql_crud::postgresql_type::StdPrimitiveI64AsNotNullBigSerialInitializedByPostgresql as postgresql_crud::PostgresqlTypePrimaryKey>::PrimaryKey),
    CheckBodySize {
        check_body_size: postgresql_crud::check_body_size::CheckBodySizeErrorNamedWithSerializeDeserialize,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Postgresql {
        postgresql: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    SerdeJson {
        serde_json: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    CheckCommit {
        check_commit: postgresql_crud::check_commit::CheckCommitErrorNamedWithSerializeDeserialize,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NoPayloadFieldsPrimaryKey {
        no_payload_fields_primary_key: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    RowAndRollback {
        row: std::string::String,
        rollback: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    CheckedAdd {
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
impl std::convert::From<TryUpdateOneRouteLogicErrorNamed> for TryUpdateOneRouteLogicResponseVariants {
    fn from(value: TryUpdateOneRouteLogicErrorNamed) -> Self {
        match value.into_serialize_deserialize_version() {
            TryUpdateOneRouteLogicErrorNamedWithSerializeDeserialize::CheckBodySize { check_body_size, code_occurence } => Self::CheckBodySize { check_body_size, code_occurence },
            TryUpdateOneRouteLogicErrorNamedWithSerializeDeserialize::Postgresql { postgresql, code_occurence } => Self::Postgresql { postgresql, code_occurence },
            TryUpdateOneRouteLogicErrorNamedWithSerializeDeserialize::SerdeJson { serde_json, code_occurence } => Self::SerdeJson { serde_json, code_occurence },
            TryUpdateOneRouteLogicErrorNamedWithSerializeDeserialize::CheckCommit { check_commit, code_occurence } => Self::CheckCommit { check_commit, code_occurence },
            TryUpdateOneRouteLogicErrorNamedWithSerializeDeserialize::NoPayloadFieldsPrimaryKey { no_payload_fields_primary_key, code_occurence } => Self::NoPayloadFieldsPrimaryKey { no_payload_fields_primary_key, code_occurence },
            TryUpdateOneRouteLogicErrorNamedWithSerializeDeserialize::RowAndRollback { row, rollback, code_occurence } => Self::RowAndRollback { row, rollback, code_occurence },
            TryUpdateOneRouteLogicErrorNamedWithSerializeDeserialize::CheckedAdd { code_occurence } => Self::CheckedAdd { code_occurence },
        }
    }
}
#[derive(Debug, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum TryUpdateOneRouteLogicErrorNamed {
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
    CheckCommit {
        #[eo_error_occurence]
        check_commit: postgresql_crud::check_commit::CheckCommitErrorNamed,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NoPayloadFieldsPrimaryKey {
        #[eo_to_std_string_string]
        no_payload_fields_primary_key: postgresql_crud::postgresql_type::StdPrimitiveI64AsNotNullBigSerialInitializedByPostgresqlUpdate,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    RowAndRollback {
        #[eo_to_std_string_string]
        row: sqlx::Error,
        #[eo_to_std_string_string]
        rollback: sqlx::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    CheckedAdd {
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
pub async fn try_update_one_route_logic(app_state: axum::extract::State<crate::repositories_types::server::routes::app_state::DynArcCombinationOfAppStateLogicTraits>, request: axum::extract::Request) -> axum::response::Response {
    let (parts, body) = request.into_parts();
    let headers = parts.headers;
    let body_bytes = match postgresql_crud::check_body_size::check_body_size(body, *app_state.get_maximum_size_of_http_body_in_bytes()).await {
        Ok(value) => value,
        Err(error_0) => {
            let error = TryUpdateOneRouteLogicErrorNamed::CheckBodySize {
                check_body_size: error_0,
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_owned(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                        line: 2290,
                        column: 264,
                    }),
                ),
            };
            eprintln!("{error}");
            let mut response = axum::response::IntoResponse::into_response(axum::Json(TryUpdateOneRouteLogicResponseVariants::from(error)));
            *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
            return response;
        }
    };
    let parameters = UpdateOneParameters {
        payload: match serde_json::from_slice::<UpdateOnePayload>(&body_bytes) {
            Ok(value) => {
                let value = UpdateOnePayload::from(value);
                if let None = &value.animal_as_not_null_jsonb_object {
                    let error_0 = value.column_6e88acb0_c566_4fef_8a09_66a41338cf36.clone();
                    let error = TryUpdateOneRouteLogicErrorNamed::NoPayloadFieldsPrimaryKey {
                        no_payload_fields_primary_key: error_0,
                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                            file!().to_owned(),
                            line!(),
                            column!(),
                            Some(error_occurence_lib::code_occurence::MacroOccurence {
                                file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                                line: 1906,
                                column: 287,
                            }),
                        ),
                    };
                    eprintln!("{error}");
                    let mut response = axum::response::IntoResponse::into_response(axum::Json(TryUpdateOneRouteLogicResponseVariants::from(error)));
                    *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                    return response;
                }
                value
            }
            Err(error_0) => {
                let error = TryUpdateOneRouteLogicErrorNamed::SerdeJson {
                    serde_json: error_0,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                            line: 2362,
                            column: 249,
                        }),
                    ),
                };
                eprintln!("{error}");
                let mut response = axum::response::IntoResponse::into_response(axum::Json(TryUpdateOneRouteLogicResponseVariants::from(error)));
                *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
                return response;
            }
        },
    };
    let query_string = {
        let mut increment: std::primitive::u64 = 0;
        let mut query = std::string::String::from("update example set ");
        if let Some(value) = &parameters.payload.animal_as_not_null_jsonb_object {
            match <AnimalAsNotNullJsonbObject as postgresql_crud::PostgresqlType>::update_query_part(&value.value, &"", &"animal_as_not_null_jsonb_object", &"", &mut increment) {
                Ok(value) => {
                    query.push_str(&format!("animal_as_not_null_jsonb_object = {value},"));
                }
                Err(error_0) => {
                    let error = TryUpdateOneRouteLogicErrorNamed::CheckedAdd {
                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                            file!().to_owned(),
                            line!(),
                            column!(),
                            Some(error_occurence_lib::code_occurence::MacroOccurence {
                                file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                                line: 4304,
                                column: 264,
                            }),
                        ),
                    };
                    eprintln!("{error}");
                    let mut response = axum::response::IntoResponse::into_response(axum::Json(TryUpdateOneRouteLogicResponseVariants::from(error)));
                    *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
                    return response;
                }
            }
        }
        let _: Option<char> = query.pop();
        match <postgresql_crud::postgresql_type::StdPrimitiveI64AsNotNullBigSerialInitializedByPostgresql as postgresql_crud::PostgresqlType>::update_query_part(&parameters.payload.column_6e88acb0_c566_4fef_8a09_66a41338cf36, &"", &"column_6e88acb0_c566_4fef_8a09_66a41338cf36", &"", &mut increment)
        {
            Ok(value) => {
                query.push_str(&format!(" where column_6e88acb0_c566_4fef_8a09_66a41338cf36 = {value}"));
            }
            Err(error_0) => {
                let error = TryUpdateOneRouteLogicErrorNamed::CheckedAdd {
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                            line: 4328,
                            column: 260,
                        }),
                    ),
                };
                eprintln!("{error}");
                let mut response = axum::response::IntoResponse::into_response(axum::Json(TryUpdateOneRouteLogicResponseVariants::from(error)));
                *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
                return response;
            }
        }
        query.push_str(&format!(" returning column_6e88acb0_c566_4fef_8a09_66a41338cf36"));
        query
    };
    println!("{}", query_string);
    let binded_query = {
        let mut query = sqlx::query::<sqlx::Postgres>(&query_string);
        if let Some(value) = parameters.payload.animal_as_not_null_jsonb_object {
            query = <AnimalAsNotNullJsonbObject as postgresql_crud::PostgresqlType>::update_query_bind(value.value, query);
        }
        query = <postgresql_crud::postgresql_type::StdPrimitiveI64AsNotNullBigSerialInitializedByPostgresql as postgresql_crud::PostgresqlType>::update_query_bind(parameters.payload.column_6e88acb0_c566_4fef_8a09_66a41338cf36, query);
        query
    };
    let mut pool_connection = match app_state.get_postgres_pool().acquire().await {
        Ok(value) => value,
        Err(error_0) => {
            let error = TryUpdateOneRouteLogicErrorNamed::Postgresql {
                postgresql: error_0,
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_owned(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                        line: 2312,
                        column: 253,
                    }),
                ),
            };
            eprintln!("{error}");
            let mut response = axum::response::IntoResponse::into_response(axum::Json(TryUpdateOneRouteLogicResponseVariants::from(error)));
            *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
            return response;
        }
    };
    let executor = match sqlx::Acquire::acquire(&mut pool_connection).await {
        Ok(value) => value,
        Err(error_0) => {
            let error = TryUpdateOneRouteLogicErrorNamed::Postgresql {
                postgresql: error_0,
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_owned(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                        line: 2312,
                        column: 253,
                    }),
                ),
            };
            eprintln!("{error}");
            let mut response = axum::response::IntoResponse::into_response(axum::Json(TryUpdateOneRouteLogicResponseVariants::from(error)));
            *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
            return response;
        }
    };
    let value = {
        let mut executor = match sqlx::Acquire::begin(executor).await {
            Ok(value) => value,
            Err(error_0) => {
                let error = TryUpdateOneRouteLogicErrorNamed::Postgresql {
                    postgresql: error_0,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                            line: 2059,
                            column: 245,
                        }),
                    ),
                };
                eprintln!("{error}");
                let mut response = axum::response::IntoResponse::into_response(axum::Json(TryUpdateOneRouteLogicResponseVariants::from(error)));
                *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                return response;
            }
        };
        let value = {
            match binded_query.fetch_one(executor.as_mut()).await {
                Ok(value) => match sqlx::Row::try_get::<<postgresql_crud::postgresql_type::StdPrimitiveI64AsNotNullBigSerialInitializedByPostgresql as postgresql_crud::PostgresqlTypePrimaryKey>::PrimaryKey, &std::primitive::str>(&value, "column_6e88acb0_c566_4fef_8a09_66a41338cf36") {
                    Ok(value) => value,
                    Err(error_0) => match executor.rollback().await {
                        Ok(_) => {
                            let error = TryUpdateOneRouteLogicErrorNamed::Postgresql {
                                postgresql: error_0,
                                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                    file!().to_owned(),
                                    line!(),
                                    column!(),
                                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                                        file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                                        line: 2711,
                                        column: 111,
                                    }),
                                ),
                            };
                            eprintln!("{error}");
                            let mut response = axum::response::IntoResponse::into_response(axum::Json(TryUpdateOneRouteLogicResponseVariants::from(error)));
                            *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                            return response;
                        }
                        Err(error_1) => {
                            let error = TryUpdateOneRouteLogicErrorNamed::RowAndRollback {
                                row: error_0,
                                rollback: error_1,
                                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                    file!().to_owned(),
                                    line!(),
                                    column!(),
                                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                                        file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                                        line: 2711,
                                        column: 140,
                                    }),
                                ),
                            };
                            eprintln!("{error}");
                            let mut response = axum::response::IntoResponse::into_response(axum::Json(TryUpdateOneRouteLogicResponseVariants::from(error)));
                            *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                            return response;
                        }
                    },
                },
                Err(error_0) => match executor.rollback().await {
                    Ok(_) => {
                        let error = TryUpdateOneRouteLogicErrorNamed::Postgresql {
                            postgresql: error_0,
                            code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                file!().to_owned(),
                                line!(),
                                column!(),
                                Some(error_occurence_lib::code_occurence::MacroOccurence {
                                    file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                                    line: 2713,
                                    column: 107,
                                }),
                            ),
                        };
                        eprintln!("{error}");
                        let mut response = axum::response::IntoResponse::into_response(axum::Json(TryUpdateOneRouteLogicResponseVariants::from(error)));
                        *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                        return response;
                    }
                    Err(error_1) => {
                        let error = TryUpdateOneRouteLogicErrorNamed::RowAndRollback {
                            row: error_0,
                            rollback: error_1,
                            code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                file!().to_owned(),
                                line!(),
                                column!(),
                                Some(error_occurence_lib::code_occurence::MacroOccurence {
                                    file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                                    line: 2713,
                                    column: 136,
                                }),
                            ),
                        };
                        eprintln!("{error}");
                        let mut response = axum::response::IntoResponse::into_response(axum::Json(TryUpdateOneRouteLogicResponseVariants::from(error)));
                        *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                        return response;
                    }
                },
            }
        };
        if let Err(error_0) = executor.commit().await {
            let error = TryUpdateOneRouteLogicErrorNamed::Postgresql {
                postgresql: error_0,
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_owned(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                        line: 2071,
                        column: 245,
                    }),
                ),
            };
            eprintln!("{error}");
            let mut response = axum::response::IntoResponse::into_response(axum::Json(TryUpdateOneRouteLogicResponseVariants::from(error)));
            *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
            return response;
        }
        value
    };
    let mut response = axum::response::IntoResponse::into_response(axum::Json(TryUpdateOneRouteLogicResponseVariants::Desirable(value)));
    *response.status_mut() = axum::http::StatusCode::OK;
    return response;
}
#[derive(Debug, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum TryUpdateOneErrorNamed {
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
    TryUpdateOneRouteLogicErrorNamedWithSerializeDeserialize {
        #[eo_to_std_string_string]
        try_update_one_route_logic_error_named_with_serialize_deserialize: TryUpdateOneRouteLogicErrorNamedWithSerializeDeserialize,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
pub async fn try_update_one(server_location: &std::primitive::str, parameters: UpdateOneParameters) -> Result<<postgresql_crud::postgresql_type::StdPrimitiveI64AsNotNullBigSerialInitializedByPostgresql as postgresql_crud::PostgresqlTypePrimaryKey>::PrimaryKey, TryUpdateOneErrorNamed> {
    let payload = {
        let value = UpdateOnePayload::from(parameters.payload);
        match serde_json::to_string(&value) {
            Ok(value) => value,
            Err(error_0) => {
                return Err(TryUpdateOneErrorNamed::SerdeJsonToString {
                    serde_json_to_string: error_0,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                            line: 2405,
                            column: 178,
                        }),
                    ),
                });
            }
        }
    };
    let url = format!("{}/example/update_one", server_location,);
    let future = reqwest::Client::new()
        .patch(&url)
        .header(&postgresql_crud::CommitSnakeCase.to_string(), git_info::PROJECT_GIT_INFO.commit)
        .header(reqwest::header::CONTENT_TYPE, "application/json")
        .body(payload)
        .send();
    let response = match future.await {
        Ok(value) => value,
        Err(error_0) => {
            return Err(TryUpdateOneErrorNamed::Reqwest {
                reqwest: error_0,
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_owned(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                        line: 2457,
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
            return Err(TryUpdateOneErrorNamed::FailedToGetResponseText {
                status_code: error_0,
                headers: error_1,
                reqwest: error_2,
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_owned(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                        line: 2474,
                        column: 192,
                    }),
                ),
            });
        }
    };
    let expected_response = match serde_json::from_str::<TryUpdateOneRouteLogicResponseVariants>(&error_2) {
        Ok(value) => value,
        Err(error_3) => {
            return Err(TryUpdateOneErrorNamed::DeserializeResponse {
                status_code: error_0,
                headers: error_1,
                response_text: error_2,
                serde: error_3,
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_owned(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                        line: 2487,
                        column: 178,
                    }),
                ),
            });
        }
    };
    let try_update_one_route_logic_error_named_with_serialize_deserialize = match expected_response {
        TryUpdateOneRouteLogicResponseVariants::Desirable(value) => {
            let value = value;
            return Ok(value);
        }
        TryUpdateOneRouteLogicResponseVariants::CheckBodySize { check_body_size, code_occurence } => TryUpdateOneRouteLogicErrorNamedWithSerializeDeserialize::CheckBodySize { check_body_size, code_occurence },
        TryUpdateOneRouteLogicResponseVariants::Postgresql { postgresql, code_occurence } => TryUpdateOneRouteLogicErrorNamedWithSerializeDeserialize::Postgresql { postgresql, code_occurence },
        TryUpdateOneRouteLogicResponseVariants::SerdeJson { serde_json, code_occurence } => TryUpdateOneRouteLogicErrorNamedWithSerializeDeserialize::SerdeJson { serde_json, code_occurence },
        TryUpdateOneRouteLogicResponseVariants::CheckCommit { check_commit, code_occurence } => TryUpdateOneRouteLogicErrorNamedWithSerializeDeserialize::CheckCommit { check_commit, code_occurence },
        TryUpdateOneRouteLogicResponseVariants::NoPayloadFieldsPrimaryKey { no_payload_fields_primary_key, code_occurence } => TryUpdateOneRouteLogicErrorNamedWithSerializeDeserialize::NoPayloadFieldsPrimaryKey { no_payload_fields_primary_key, code_occurence },
        TryUpdateOneRouteLogicResponseVariants::RowAndRollback { row, rollback, code_occurence } => TryUpdateOneRouteLogicErrorNamedWithSerializeDeserialize::RowAndRollback { row, rollback, code_occurence },
        TryUpdateOneRouteLogicResponseVariants::CheckedAdd { code_occurence } => TryUpdateOneRouteLogicErrorNamedWithSerializeDeserialize::CheckedAdd { code_occurence },
    };
    Err(TryUpdateOneErrorNamed::TryUpdateOneRouteLogicErrorNamedWithSerializeDeserialize {
        try_update_one_route_logic_error_named_with_serialize_deserialize,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
            file!().to_owned(),
            line!(),
            column!(),
            Some(error_occurence_lib::code_occurence::MacroOccurence {
                file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                line: 2525,
                column: 223,
            }),
        ),
    })
}
impl postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for UpdateOnePayload {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self {
            column_6e88acb0_c566_4fef_8a09_66a41338cf36: postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            animal_as_not_null_jsonb_object: Some(postgresql_crud::Value {
                value: postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            }),
        }
    }
}
pub async fn update_one_payload_example_route_logic() -> axum::response::Response {
    let mut response = axum::response::IntoResponse::into_response(axum::Json(
        <UpdateOnePayload as postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement>::default_but_option_is_always_some_and_vec_always_contains_one_element(),
    ));
    *response.status_mut() = axum::http::StatusCode::OK;
    return response;
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema)]
pub struct DeleteManyPayload {
    pub column_6e88acb0_c566_4fef_8a09_66a41338cf36: std::option::Option<postgresql_crud::PostgresqlTypeWhere<<postgresql_crud::postgresql_type::StdPrimitiveI64AsNotNullBigSerialInitializedByPostgresql as postgresql_crud::PostgresqlType>::WhereElement>>,
    pub animal_as_not_null_jsonb_object: std::option::Option<postgresql_crud::PostgresqlTypeWhere<<AnimalAsNotNullJsonbObject as postgresql_crud::PostgresqlType>::WhereElement>>,
}
#[derive(Debug)]
pub struct DeleteManyParameters {
    pub payload: DeleteManyPayload,
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize)]
pub enum TryDeleteManyRouteLogicResponseVariants {
    Desirable(std::vec::Vec<<postgresql_crud::postgresql_type::StdPrimitiveI64AsNotNullBigSerialInitializedByPostgresql as postgresql_crud::PostgresqlTypePrimaryKey>::PrimaryKey>),
    CheckBodySize {
        check_body_size: postgresql_crud::check_body_size::CheckBodySizeErrorNamedWithSerializeDeserialize,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Postgresql {
        postgresql: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    SerdeJson {
        serde_json: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    CheckCommit {
        check_commit: postgresql_crud::check_commit::CheckCommitErrorNamedWithSerializeDeserialize,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NoPayloadFields {
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NoPrimaryKeys {
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    RowAndRollback {
        row: std::string::String,
        rollback: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NonExistingPrimaryKeys {
        non_existing_primary_keys: std::vec::Vec<std::string::String>,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NonExistingPrimaryKeysAndRollback {
        non_existing_primary_keys: std::vec::Vec<std::string::String>,
        rollback: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NotUniquePrimaryKey {
        not_unique_primary_key: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    CheckedAdd {
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
impl std::convert::From<TryDeleteManyRouteLogicErrorNamed> for TryDeleteManyRouteLogicResponseVariants {
    fn from(value: TryDeleteManyRouteLogicErrorNamed) -> Self {
        match value.into_serialize_deserialize_version() {
            TryDeleteManyRouteLogicErrorNamedWithSerializeDeserialize::CheckBodySize { check_body_size, code_occurence } => Self::CheckBodySize { check_body_size, code_occurence },
            TryDeleteManyRouteLogicErrorNamedWithSerializeDeserialize::Postgresql { postgresql, code_occurence } => Self::Postgresql { postgresql, code_occurence },
            TryDeleteManyRouteLogicErrorNamedWithSerializeDeserialize::SerdeJson { serde_json, code_occurence } => Self::SerdeJson { serde_json, code_occurence },
            TryDeleteManyRouteLogicErrorNamedWithSerializeDeserialize::CheckCommit { check_commit, code_occurence } => Self::CheckCommit { check_commit, code_occurence },
            TryDeleteManyRouteLogicErrorNamedWithSerializeDeserialize::NoPayloadFields { code_occurence } => Self::NoPayloadFields { code_occurence },
            TryDeleteManyRouteLogicErrorNamedWithSerializeDeserialize::NoPrimaryKeys { code_occurence } => Self::NoPrimaryKeys { code_occurence },
            TryDeleteManyRouteLogicErrorNamedWithSerializeDeserialize::RowAndRollback { row, rollback, code_occurence } => Self::RowAndRollback { row, rollback, code_occurence },
            TryDeleteManyRouteLogicErrorNamedWithSerializeDeserialize::NonExistingPrimaryKeys { non_existing_primary_keys, code_occurence } => Self::NonExistingPrimaryKeys { non_existing_primary_keys, code_occurence },
            TryDeleteManyRouteLogicErrorNamedWithSerializeDeserialize::NonExistingPrimaryKeysAndRollback { non_existing_primary_keys, rollback, code_occurence } => Self::NonExistingPrimaryKeysAndRollback { non_existing_primary_keys, rollback, code_occurence },
            TryDeleteManyRouteLogicErrorNamedWithSerializeDeserialize::NotUniquePrimaryKey { not_unique_primary_key, code_occurence } => Self::NotUniquePrimaryKey { not_unique_primary_key, code_occurence },
            TryDeleteManyRouteLogicErrorNamedWithSerializeDeserialize::CheckedAdd { code_occurence } => Self::CheckedAdd { code_occurence },
        }
    }
}
#[derive(Debug, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum TryDeleteManyRouteLogicErrorNamed {
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
    CheckCommit {
        #[eo_error_occurence]
        check_commit: postgresql_crud::check_commit::CheckCommitErrorNamed,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NoPayloadFields {
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NoPrimaryKeys {
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    RowAndRollback {
        #[eo_to_std_string_string]
        row: sqlx::Error,
        #[eo_to_std_string_string]
        rollback: sqlx::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NonExistingPrimaryKeys {
        #[eo_vec_to_std_string_string]
        non_existing_primary_keys: std::vec::Vec<postgresql_crud::postgresql_type::StdPrimitiveI64AsNotNullBigSerialInitializedByPostgresqlRead>,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NonExistingPrimaryKeysAndRollback {
        #[eo_vec_to_std_string_string]
        non_existing_primary_keys: std::vec::Vec<postgresql_crud::postgresql_type::StdPrimitiveI64AsNotNullBigSerialInitializedByPostgresqlRead>,
        #[eo_to_std_string_string]
        rollback: sqlx::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NotUniquePrimaryKey {
        #[eo_to_std_string_string]
        not_unique_primary_key: postgresql_crud::postgresql_type::StdPrimitiveI64AsNotNullBigSerialInitializedByPostgresqlRead,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    CheckedAdd {
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
pub async fn try_delete_many_route_logic(app_state: axum::extract::State<crate::repositories_types::server::routes::app_state::DynArcCombinationOfAppStateLogicTraits>, request: axum::extract::Request) -> axum::response::Response {
    let (parts, body) = request.into_parts();
    let headers = parts.headers;
    let body_bytes = match postgresql_crud::check_body_size::check_body_size(body, *app_state.get_maximum_size_of_http_body_in_bytes()).await {
        Ok(value) => value,
        Err(error_0) => {
            let error = TryDeleteManyRouteLogicErrorNamed::CheckBodySize {
                check_body_size: error_0,
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_owned(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                        line: 2290,
                        column: 264,
                    }),
                ),
            };
            eprintln!("{error}");
            let mut response = axum::response::IntoResponse::into_response(axum::Json(TryDeleteManyRouteLogicResponseVariants::from(error)));
            *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
            return response;
        }
    };
    let parameters = DeleteManyParameters {
        payload: match serde_json::from_slice::<DeleteManyPayload>(&body_bytes) {
            Ok(value) => {
                let value = DeleteManyPayload::from(value);
                value
            }
            Err(error_0) => {
                let error = TryDeleteManyRouteLogicErrorNamed::SerdeJson {
                    serde_json: error_0,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                            line: 2362,
                            column: 249,
                        }),
                    ),
                };
                eprintln!("{error}");
                let mut response = axum::response::IntoResponse::into_response(axum::Json(TryDeleteManyRouteLogicResponseVariants::from(error)));
                *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
                return response;
            }
        },
    };
    let query_string = {
        format!("delete from example {} returning column_6e88acb0_c566_4fef_8a09_66a41338cf36", {
            let mut increment: std::primitive::u64 = 0;
            let mut additional_parameters = std::string::String::from("where");
            let mut is_first_push_to_additional_parameters_already_happend = false;
            if let Some(value) = &parameters.payload.column_6e88acb0_c566_4fef_8a09_66a41338cf36 {
                match postgresql_crud::PostgresqlTypeWhereFilter::query_part(value, &mut increment, &"column_6e88acb0_c566_4fef_8a09_66a41338cf36", is_first_push_to_additional_parameters_already_happend) {
                    Ok(value) => {
                        additional_parameters.push_str(&value);
                        is_first_push_to_additional_parameters_already_happend = true;
                    }
                    Err(error_0) => {
                        let error = TryDeleteManyRouteLogicErrorNamed::CheckedAdd {
                            code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                file!().to_owned(),
                                line!(),
                                column!(),
                                Some(error_occurence_lib::code_occurence::MacroOccurence {
                                    file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                                    line: 4549,
                                    column: 260,
                                }),
                            ),
                        };
                        eprintln!("{error}");
                        let mut response = axum::response::IntoResponse::into_response(axum::Json(TryDeleteManyRouteLogicResponseVariants::from(error)));
                        *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
                        return response;
                    }
                }
            }
            if let Some(value) = &parameters.payload.animal_as_not_null_jsonb_object {
                match postgresql_crud::PostgresqlTypeWhereFilter::query_part(value, &mut increment, &"animal_as_not_null_jsonb_object", is_first_push_to_additional_parameters_already_happend) {
                    Ok(value) => {
                        additional_parameters.push_str(&value);
                        // is_first_push_to_additional_parameters_already_happend = true;
                    }
                    Err(error_0) => {
                        let error = TryDeleteManyRouteLogicErrorNamed::CheckedAdd {
                            code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                file!().to_owned(),
                                line!(),
                                column!(),
                                Some(error_occurence_lib::code_occurence::MacroOccurence {
                                    file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                                    line: 4549,
                                    column: 260,
                                }),
                            ),
                        };
                        eprintln!("{error}");
                        let mut response = axum::response::IntoResponse::into_response(axum::Json(TryDeleteManyRouteLogicResponseVariants::from(error)));
                        *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
                        return response;
                    }
                }
            }
            additional_parameters
        })
    };
    println!("{}", query_string);
    let binded_query = {
        let mut query = sqlx::query::<sqlx::Postgres>(&query_string);
        if let Some(value) = parameters.payload.column_6e88acb0_c566_4fef_8a09_66a41338cf36 {
            query = postgresql_crud::PostgresqlTypeWhereFilter::query_bind(value, query);
        }
        if let Some(value) = parameters.payload.animal_as_not_null_jsonb_object {
            query = postgresql_crud::PostgresqlTypeWhereFilter::query_bind(value, query);
        }
        query
    };
    let mut pool_connection = match app_state.get_postgres_pool().acquire().await {
        Ok(value) => value,
        Err(error_0) => {
            let error = TryDeleteManyRouteLogicErrorNamed::Postgresql {
                postgresql: error_0,
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_owned(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                        line: 2312,
                        column: 253,
                    }),
                ),
            };
            eprintln!("{error}");
            let mut response = axum::response::IntoResponse::into_response(axum::Json(TryDeleteManyRouteLogicResponseVariants::from(error)));
            *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
            return response;
        }
    };
    let executor = match sqlx::Acquire::acquire(&mut pool_connection).await {
        Ok(value) => value,
        Err(error_0) => {
            let error = TryDeleteManyRouteLogicErrorNamed::Postgresql {
                postgresql: error_0,
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_owned(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                        line: 2312,
                        column: 253,
                    }),
                ),
            };
            eprintln!("{error}");
            let mut response = axum::response::IntoResponse::into_response(axum::Json(TryDeleteManyRouteLogicResponseVariants::from(error)));
            *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
            return response;
        }
    };
    let value = {
        let mut executor = match sqlx::Acquire::begin(executor).await {
            Ok(value) => value,
            Err(error_0) => {
                let error = TryDeleteManyRouteLogicErrorNamed::Postgresql {
                    postgresql: error_0,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                            line: 2059,
                            column: 245,
                        }),
                    ),
                };
                eprintln!("{error}");
                let mut response = axum::response::IntoResponse::into_response(axum::Json(TryDeleteManyRouteLogicResponseVariants::from(error)));
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
                    Some(value) => match sqlx::Row::try_get::<<postgresql_crud::postgresql_type::StdPrimitiveI64AsNotNullBigSerialInitializedByPostgresql as postgresql_crud::PostgresqlTypePrimaryKey>::PrimaryKey, &std::primitive::str>(&value, "column_6e88acb0_c566_4fef_8a09_66a41338cf36") {
                        Ok(value) => Some(value),
                        Err(error_0) => {
                            drop(rows);
                            match executor.rollback().await {
                                Ok(_) => {
                                    let error = TryDeleteManyRouteLogicErrorNamed::Postgresql {
                                        postgresql: error_0,
                                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                            file!().to_owned(),
                                            line!(),
                                            column!(),
                                            Some(error_occurence_lib::code_occurence::MacroOccurence {
                                                file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                                                line: 2693,
                                                column: 128,
                                            }),
                                        ),
                                    };
                                    eprintln!("{error}");
                                    let mut response = axum::response::IntoResponse::into_response(axum::Json(TryDeleteManyRouteLogicResponseVariants::from(error)));
                                    *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                                    return response;
                                }
                                Err(error_1) => {
                                    let error = TryDeleteManyRouteLogicErrorNamed::RowAndRollback {
                                        row: error_0,
                                        rollback: error_1,
                                        code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                            file!().to_owned(),
                                            line!(),
                                            column!(),
                                            Some(error_occurence_lib::code_occurence::MacroOccurence {
                                                file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                                                line: 2693,
                                                column: 157,
                                            }),
                                        ),
                                    };
                                    eprintln!("{error}");
                                    let mut response = axum::response::IntoResponse::into_response(axum::Json(TryDeleteManyRouteLogicResponseVariants::from(error)));
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
                            let error = TryDeleteManyRouteLogicErrorNamed::Postgresql {
                                postgresql: error_0,
                                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                    file!().to_owned(),
                                    line!(),
                                    column!(),
                                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                                        file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                                        line: 2695,
                                        column: 124,
                                    }),
                                ),
                            };
                            eprintln!("{error}");
                            let mut response = axum::response::IntoResponse::into_response(axum::Json(TryDeleteManyRouteLogicResponseVariants::from(error)));
                            *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                            return response;
                        }
                        Err(error_1) => {
                            let error = TryDeleteManyRouteLogicErrorNamed::RowAndRollback {
                                row: error_0,
                                rollback: error_1,
                                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                    file!().to_owned(),
                                    line!(),
                                    column!(),
                                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                                        file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                                        line: 2695,
                                        column: 153,
                                    }),
                                ),
                            };
                            eprintln!("{error}");
                            let mut response = axum::response::IntoResponse::into_response(axum::Json(TryDeleteManyRouteLogicResponseVariants::from(error)));
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
            let error = TryDeleteManyRouteLogicErrorNamed::Postgresql {
                postgresql: error_0,
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_owned(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                        line: 2071,
                        column: 245,
                    }),
                ),
            };
            eprintln!("{error}");
            let mut response = axum::response::IntoResponse::into_response(axum::Json(TryDeleteManyRouteLogicResponseVariants::from(error)));
            *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
            return response;
        }
        value
    };
    let mut response = axum::response::IntoResponse::into_response(axum::Json(TryDeleteManyRouteLogicResponseVariants::Desirable(value)));
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
    TryDeleteManyRouteLogicErrorNamedWithSerializeDeserialize {
        #[eo_to_std_string_string]
        try_delete_many_route_logic_error_named_with_serialize_deserialize: TryDeleteManyRouteLogicErrorNamedWithSerializeDeserialize,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
pub async fn try_delete_many(
    server_location: &std::primitive::str,
    parameters: DeleteManyParameters,
) -> Result<std::vec::Vec<<postgresql_crud::postgresql_type::StdPrimitiveI64AsNotNullBigSerialInitializedByPostgresql as postgresql_crud::PostgresqlTypePrimaryKey>::PrimaryKey>, TryDeleteManyErrorNamed> {
    let payload = {
        let value = DeleteManyPayload::from(parameters.payload);
        match serde_json::to_string(&value) {
            Ok(value) => value,
            Err(error_0) => {
                return Err(TryDeleteManyErrorNamed::SerdeJsonToString {
                    serde_json_to_string: error_0,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                            line: 2405,
                            column: 178,
                        }),
                    ),
                });
            }
        }
    };
    let url = format!("{}/example/delete_many", server_location,);
    let future = reqwest::Client::new()
        .delete(&url)
        .header(&postgresql_crud::CommitSnakeCase.to_string(), git_info::PROJECT_GIT_INFO.commit)
        .header(reqwest::header::CONTENT_TYPE, "application/json")
        .body(payload)
        .send();
    let response = match future.await {
        Ok(value) => value,
        Err(error_0) => {
            return Err(TryDeleteManyErrorNamed::Reqwest {
                reqwest: error_0,
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_owned(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                        line: 2457,
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
            return Err(TryDeleteManyErrorNamed::FailedToGetResponseText {
                status_code: error_0,
                headers: error_1,
                reqwest: error_2,
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_owned(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                        line: 2474,
                        column: 192,
                    }),
                ),
            });
        }
    };
    let expected_response = match serde_json::from_str::<TryDeleteManyRouteLogicResponseVariants>(&error_2) {
        Ok(value) => value,
        Err(error_3) => {
            return Err(TryDeleteManyErrorNamed::DeserializeResponse {
                status_code: error_0,
                headers: error_1,
                response_text: error_2,
                serde: error_3,
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_owned(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                        line: 2487,
                        column: 178,
                    }),
                ),
            });
        }
    };
    let try_delete_many_route_logic_error_named_with_serialize_deserialize = match expected_response {
        TryDeleteManyRouteLogicResponseVariants::Desirable(value) => {
            let value = value;
            return Ok(value);
        }
        TryDeleteManyRouteLogicResponseVariants::CheckBodySize { check_body_size, code_occurence } => TryDeleteManyRouteLogicErrorNamedWithSerializeDeserialize::CheckBodySize { check_body_size, code_occurence },
        TryDeleteManyRouteLogicResponseVariants::Postgresql { postgresql, code_occurence } => TryDeleteManyRouteLogicErrorNamedWithSerializeDeserialize::Postgresql { postgresql, code_occurence },
        TryDeleteManyRouteLogicResponseVariants::SerdeJson { serde_json, code_occurence } => TryDeleteManyRouteLogicErrorNamedWithSerializeDeserialize::SerdeJson { serde_json, code_occurence },
        TryDeleteManyRouteLogicResponseVariants::CheckCommit { check_commit, code_occurence } => TryDeleteManyRouteLogicErrorNamedWithSerializeDeserialize::CheckCommit { check_commit, code_occurence },
        TryDeleteManyRouteLogicResponseVariants::NoPayloadFields { code_occurence } => TryDeleteManyRouteLogicErrorNamedWithSerializeDeserialize::NoPayloadFields { code_occurence },
        TryDeleteManyRouteLogicResponseVariants::NoPrimaryKeys { code_occurence } => TryDeleteManyRouteLogicErrorNamedWithSerializeDeserialize::NoPrimaryKeys { code_occurence },
        TryDeleteManyRouteLogicResponseVariants::RowAndRollback { row, rollback, code_occurence } => TryDeleteManyRouteLogicErrorNamedWithSerializeDeserialize::RowAndRollback { row, rollback, code_occurence },
        TryDeleteManyRouteLogicResponseVariants::NonExistingPrimaryKeys { non_existing_primary_keys, code_occurence } => TryDeleteManyRouteLogicErrorNamedWithSerializeDeserialize::NonExistingPrimaryKeys { non_existing_primary_keys, code_occurence },
        TryDeleteManyRouteLogicResponseVariants::NonExistingPrimaryKeysAndRollback { non_existing_primary_keys, rollback, code_occurence } => TryDeleteManyRouteLogicErrorNamedWithSerializeDeserialize::NonExistingPrimaryKeysAndRollback { non_existing_primary_keys, rollback, code_occurence },
        TryDeleteManyRouteLogicResponseVariants::NotUniquePrimaryKey { not_unique_primary_key, code_occurence } => TryDeleteManyRouteLogicErrorNamedWithSerializeDeserialize::NotUniquePrimaryKey { not_unique_primary_key, code_occurence },
        TryDeleteManyRouteLogicResponseVariants::CheckedAdd { code_occurence } => TryDeleteManyRouteLogicErrorNamedWithSerializeDeserialize::CheckedAdd { code_occurence },
    };
    Err(TryDeleteManyErrorNamed::TryDeleteManyRouteLogicErrorNamedWithSerializeDeserialize {
        try_delete_many_route_logic_error_named_with_serialize_deserialize,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
            file!().to_owned(),
            line!(),
            column!(),
            Some(error_occurence_lib::code_occurence::MacroOccurence {
                file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                line: 2525,
                column: 223,
            }),
        ),
    })
}
impl postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for DeleteManyPayload {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self {
            column_6e88acb0_c566_4fef_8a09_66a41338cf36: Some(postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
            animal_as_not_null_jsonb_object: Some(postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()),
        }
    }
}
pub async fn delete_many_payload_example_route_logic() -> axum::response::Response {
    let mut response = axum::response::IntoResponse::into_response(axum::Json(
        <DeleteManyPayload as postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement>::default_but_option_is_always_some_and_vec_always_contains_one_element(),
    ));
    *response.status_mut() = axum::http::StatusCode::OK;
    return response;
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema)]
pub struct DeleteOnePayload {
    pub column_6e88acb0_c566_4fef_8a09_66a41338cf36: postgresql_crud::postgresql_type::StdPrimitiveI64AsNotNullBigSerialInitializedByPostgresqlRead,
}
#[derive(Debug)]
pub struct DeleteOneParameters {
    pub payload: DeleteOnePayload,
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize)]
pub enum TryDeleteOneRouteLogicResponseVariants {
    Desirable(<postgresql_crud::postgresql_type::StdPrimitiveI64AsNotNullBigSerialInitializedByPostgresql as postgresql_crud::PostgresqlTypePrimaryKey>::PrimaryKey),
    CheckBodySize {
        check_body_size: postgresql_crud::check_body_size::CheckBodySizeErrorNamedWithSerializeDeserialize,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Postgresql {
        postgresql: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    SerdeJson {
        serde_json: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    CheckCommit {
        check_commit: postgresql_crud::check_commit::CheckCommitErrorNamedWithSerializeDeserialize,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    RowAndRollback {
        row: std::string::String,
        rollback: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
impl std::convert::From<TryDeleteOneRouteLogicErrorNamed> for TryDeleteOneRouteLogicResponseVariants {
    fn from(value: TryDeleteOneRouteLogicErrorNamed) -> Self {
        match value.into_serialize_deserialize_version() {
            TryDeleteOneRouteLogicErrorNamedWithSerializeDeserialize::CheckBodySize { check_body_size, code_occurence } => Self::CheckBodySize { check_body_size, code_occurence },
            TryDeleteOneRouteLogicErrorNamedWithSerializeDeserialize::Postgresql { postgresql, code_occurence } => Self::Postgresql { postgresql, code_occurence },
            TryDeleteOneRouteLogicErrorNamedWithSerializeDeserialize::SerdeJson { serde_json, code_occurence } => Self::SerdeJson { serde_json, code_occurence },
            TryDeleteOneRouteLogicErrorNamedWithSerializeDeserialize::CheckCommit { check_commit, code_occurence } => Self::CheckCommit { check_commit, code_occurence },
            TryDeleteOneRouteLogicErrorNamedWithSerializeDeserialize::RowAndRollback { row, rollback, code_occurence } => Self::RowAndRollback { row, rollback, code_occurence },
        }
    }
}
#[derive(Debug, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum TryDeleteOneRouteLogicErrorNamed {
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
}
pub async fn try_delete_one_route_logic(app_state: axum::extract::State<crate::repositories_types::server::routes::app_state::DynArcCombinationOfAppStateLogicTraits>, request: axum::extract::Request) -> axum::response::Response {
    let (parts, body) = request.into_parts();
    let headers = parts.headers;
    let body_bytes = match postgresql_crud::check_body_size::check_body_size(body, *app_state.get_maximum_size_of_http_body_in_bytes()).await {
        Ok(value) => value,
        Err(error_0) => {
            let error = TryDeleteOneRouteLogicErrorNamed::CheckBodySize {
                check_body_size: error_0,
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_owned(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                        line: 2290,
                        column: 264,
                    }),
                ),
            };
            eprintln!("{error}");
            let mut response = axum::response::IntoResponse::into_response(axum::Json(TryDeleteOneRouteLogicResponseVariants::from(error)));
            *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
            return response;
        }
    };
    let parameters = DeleteOneParameters {
        payload: match serde_json::from_slice::<DeleteOnePayload>(&body_bytes) {
            Ok(value) => {
                let value = DeleteOnePayload::from(value);
                value
            }
            Err(error_0) => {
                let error = TryDeleteOneRouteLogicErrorNamed::SerdeJson {
                    serde_json: error_0,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                            line: 2362,
                            column: 249,
                        }),
                    ),
                };
                eprintln!("{error}");
                let mut response = axum::response::IntoResponse::into_response(axum::Json(TryDeleteOneRouteLogicResponseVariants::from(error)));
                *response.status_mut() = axum::http::StatusCode::BAD_REQUEST;
                return response;
            }
        },
    };
    let query_string = format!("delete from example where column_6e88acb0_c566_4fef_8a09_66a41338cf36 = $1  returning column_6e88acb0_c566_4fef_8a09_66a41338cf36");
    println!("{}", query_string);
    let binded_query = {
        let mut query = sqlx::query::<sqlx::Postgres>(&query_string);
        query = postgresql_crud::PostgresqlTypeWhereFilter::query_bind(parameters.payload.column_6e88acb0_c566_4fef_8a09_66a41338cf36, query);
        query
    };
    let mut pool_connection = match app_state.get_postgres_pool().acquire().await {
        Ok(value) => value,
        Err(error_0) => {
            let error = TryDeleteOneRouteLogicErrorNamed::Postgresql {
                postgresql: error_0,
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_owned(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                        line: 2312,
                        column: 253,
                    }),
                ),
            };
            eprintln!("{error}");
            let mut response = axum::response::IntoResponse::into_response(axum::Json(TryDeleteOneRouteLogicResponseVariants::from(error)));
            *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
            return response;
        }
    };
    let executor = match sqlx::Acquire::acquire(&mut pool_connection).await {
        Ok(value) => value,
        Err(error_0) => {
            let error = TryDeleteOneRouteLogicErrorNamed::Postgresql {
                postgresql: error_0,
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_owned(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                        line: 2312,
                        column: 253,
                    }),
                ),
            };
            eprintln!("{error}");
            let mut response = axum::response::IntoResponse::into_response(axum::Json(TryDeleteOneRouteLogicResponseVariants::from(error)));
            *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
            return response;
        }
    };
    let value = {
        let mut executor = match sqlx::Acquire::begin(executor).await {
            Ok(value) => value,
            Err(error_0) => {
                let error = TryDeleteOneRouteLogicErrorNamed::Postgresql {
                    postgresql: error_0,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                            line: 2059,
                            column: 245,
                        }),
                    ),
                };
                eprintln!("{error}");
                let mut response = axum::response::IntoResponse::into_response(axum::Json(TryDeleteOneRouteLogicResponseVariants::from(error)));
                *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                return response;
            }
        };
        let value = {
            match binded_query.fetch_one(executor.as_mut()).await {
                Ok(value) => match sqlx::Row::try_get::<<postgresql_crud::postgresql_type::StdPrimitiveI64AsNotNullBigSerialInitializedByPostgresql as postgresql_crud::PostgresqlTypePrimaryKey>::PrimaryKey, &std::primitive::str>(&value, "column_6e88acb0_c566_4fef_8a09_66a41338cf36") {
                    Ok(value) => value,
                    Err(error_0) => match executor.rollback().await {
                        Ok(_) => {
                            let error = TryDeleteOneRouteLogicErrorNamed::Postgresql {
                                postgresql: error_0,
                                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                    file!().to_owned(),
                                    line!(),
                                    column!(),
                                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                                        file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                                        line: 2711,
                                        column: 111,
                                    }),
                                ),
                            };
                            eprintln!("{error}");
                            let mut response = axum::response::IntoResponse::into_response(axum::Json(TryDeleteOneRouteLogicResponseVariants::from(error)));
                            *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                            return response;
                        }
                        Err(error_1) => {
                            let error = TryDeleteOneRouteLogicErrorNamed::RowAndRollback {
                                row: error_0,
                                rollback: error_1,
                                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                    file!().to_owned(),
                                    line!(),
                                    column!(),
                                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                                        file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                                        line: 2711,
                                        column: 140,
                                    }),
                                ),
                            };
                            eprintln!("{error}");
                            let mut response = axum::response::IntoResponse::into_response(axum::Json(TryDeleteOneRouteLogicResponseVariants::from(error)));
                            *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                            return response;
                        }
                    },
                },
                Err(error_0) => match executor.rollback().await {
                    Ok(_) => {
                        let error = TryDeleteOneRouteLogicErrorNamed::Postgresql {
                            postgresql: error_0,
                            code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                file!().to_owned(),
                                line!(),
                                column!(),
                                Some(error_occurence_lib::code_occurence::MacroOccurence {
                                    file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                                    line: 2713,
                                    column: 107,
                                }),
                            ),
                        };
                        eprintln!("{error}");
                        let mut response = axum::response::IntoResponse::into_response(axum::Json(TryDeleteOneRouteLogicResponseVariants::from(error)));
                        *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                        return response;
                    }
                    Err(error_1) => {
                        let error = TryDeleteOneRouteLogicErrorNamed::RowAndRollback {
                            row: error_0,
                            rollback: error_1,
                            code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                                file!().to_owned(),
                                line!(),
                                column!(),
                                Some(error_occurence_lib::code_occurence::MacroOccurence {
                                    file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                                    line: 2713,
                                    column: 136,
                                }),
                            ),
                        };
                        eprintln!("{error}");
                        let mut response = axum::response::IntoResponse::into_response(axum::Json(TryDeleteOneRouteLogicResponseVariants::from(error)));
                        *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
                        return response;
                    }
                },
            }
        };
        if let Err(error_0) = executor.commit().await {
            let error = TryDeleteOneRouteLogicErrorNamed::Postgresql {
                postgresql: error_0,
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_owned(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                        line: 2071,
                        column: 245,
                    }),
                ),
            };
            eprintln!("{error}");
            let mut response = axum::response::IntoResponse::into_response(axum::Json(TryDeleteOneRouteLogicResponseVariants::from(error)));
            *response.status_mut() = axum::http::StatusCode::INTERNAL_SERVER_ERROR;
            return response;
        }
        value
    };
    let mut response = axum::response::IntoResponse::into_response(axum::Json(TryDeleteOneRouteLogicResponseVariants::Desirable(value)));
    *response.status_mut() = axum::http::StatusCode::OK;
    return response;
}
#[derive(Debug, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum TryDeleteOneErrorNamed {
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
    TryDeleteOneRouteLogicErrorNamedWithSerializeDeserialize {
        #[eo_to_std_string_string]
        try_delete_one_route_logic_error_named_with_serialize_deserialize: TryDeleteOneRouteLogicErrorNamedWithSerializeDeserialize,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
pub async fn try_delete_one(server_location: &std::primitive::str, parameters: DeleteOneParameters) -> Result<<postgresql_crud::postgresql_type::StdPrimitiveI64AsNotNullBigSerialInitializedByPostgresql as postgresql_crud::PostgresqlTypePrimaryKey>::PrimaryKey, TryDeleteOneErrorNamed> {
    let payload = {
        let value = DeleteOnePayload::from(parameters.payload);
        match serde_json::to_string(&value) {
            Ok(value) => value,
            Err(error_0) => {
                return Err(TryDeleteOneErrorNamed::SerdeJsonToString {
                    serde_json_to_string: error_0,
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        file!().to_owned(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                            line: 2405,
                            column: 178,
                        }),
                    ),
                });
            }
        }
    };
    let url = format!("{}/example/delete_one", server_location,);
    let future = reqwest::Client::new()
        .delete(&url)
        .header(&postgresql_crud::CommitSnakeCase.to_string(), git_info::PROJECT_GIT_INFO.commit)
        .header(reqwest::header::CONTENT_TYPE, "application/json")
        .body(payload)
        .send();
    let response = match future.await {
        Ok(value) => value,
        Err(error_0) => {
            return Err(TryDeleteOneErrorNamed::Reqwest {
                reqwest: error_0,
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_owned(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                        line: 2457,
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
            return Err(TryDeleteOneErrorNamed::FailedToGetResponseText {
                status_code: error_0,
                headers: error_1,
                reqwest: error_2,
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_owned(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                        line: 2474,
                        column: 192,
                    }),
                ),
            });
        }
    };
    let expected_response = match serde_json::from_str::<TryDeleteOneRouteLogicResponseVariants>(&error_2) {
        Ok(value) => value,
        Err(error_3) => {
            return Err(TryDeleteOneErrorNamed::DeserializeResponse {
                status_code: error_0,
                headers: error_1,
                response_text: error_2,
                serde: error_3,
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    file!().to_owned(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                        line: 2487,
                        column: 178,
                    }),
                ),
            });
        }
    };
    let try_delete_one_route_logic_error_named_with_serialize_deserialize = match expected_response {
        TryDeleteOneRouteLogicResponseVariants::Desirable(value) => {
            let value = value;
            return Ok(value);
        }
        TryDeleteOneRouteLogicResponseVariants::CheckBodySize { check_body_size, code_occurence } => TryDeleteOneRouteLogicErrorNamedWithSerializeDeserialize::CheckBodySize { check_body_size, code_occurence },
        TryDeleteOneRouteLogicResponseVariants::Postgresql { postgresql, code_occurence } => TryDeleteOneRouteLogicErrorNamedWithSerializeDeserialize::Postgresql { postgresql, code_occurence },
        TryDeleteOneRouteLogicResponseVariants::SerdeJson { serde_json, code_occurence } => TryDeleteOneRouteLogicErrorNamedWithSerializeDeserialize::SerdeJson { serde_json, code_occurence },
        TryDeleteOneRouteLogicResponseVariants::CheckCommit { check_commit, code_occurence } => TryDeleteOneRouteLogicErrorNamedWithSerializeDeserialize::CheckCommit { check_commit, code_occurence },
        TryDeleteOneRouteLogicResponseVariants::RowAndRollback { row, rollback, code_occurence } => TryDeleteOneRouteLogicErrorNamedWithSerializeDeserialize::RowAndRollback { row, rollback, code_occurence },
    };
    Err(TryDeleteOneErrorNamed::TryDeleteOneRouteLogicErrorNamedWithSerializeDeserialize {
        try_delete_one_route_logic_error_named_with_serialize_deserialize,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
            file!().to_owned(),
            line!(),
            column!(),
            Some(error_occurence_lib::code_occurence::MacroOccurence {
                file: std::string::String::from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                line: 2525,
                column: 223,
            }),
        ),
    })
}
impl postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for DeleteOnePayload {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self {
            column_6e88acb0_c566_4fef_8a09_66a41338cf36: postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
        }
    }
}
pub async fn delete_one_payload_example_route_logic() -> axum::response::Response {
    let mut response = axum::response::IntoResponse::into_response(axum::Json(
        <DeleteOnePayload as postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement>::default_but_option_is_always_some_and_vec_always_contains_one_element(),
    ));
    *response.status_mut() = axum::http::StatusCode::OK;
    return response;
}
