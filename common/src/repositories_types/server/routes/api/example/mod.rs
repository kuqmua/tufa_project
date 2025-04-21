#[derive(Debug
    , postgresql_crud::GeneratePostgresqlCrud
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
#[postgresql_crud::create_many_additional_route_logic{
    println!("kekw");
}]
#[postgresql_crud::create_one_additional_route_logic{}]
#[postgresql_crud::read_many_additional_route_logic{}]
#[postgresql_crud::read_one_additional_route_logic{}]
#[postgresql_crud::update_many_additional_route_logic{}]
#[postgresql_crud::update_one_additional_route_logic{}]
#[postgresql_crud::delete_many_additional_route_logic{}]
#[postgresql_crud::delete_one_additional_route_logic{}]
#[postgresql_crud::common_additional_route_logic{
    // if let Err(error) = postgresql_crud::check_commit::check_commit(
    //     *app_state.get_enable_api_git_commit_check(),
    //     &headers,
    // ) {
    //     let status_code = postgresql_crud::GetAxumHttpStatusCode::get_axum_http_status_code(&error);
    //     //todo use reserved work instead of TryCreateManyRouteLogicErrorNamed
    //     let error = TryCreateManyRouteLogicErrorNamed::CheckCommit {
    //         check_commit: error,
    //         code_occurence: error_occurence_lib::code_occurence!(),
    //     };
    //     eprintln!("{error}");
    //     let mut response = axum::response::IntoResponse::into_response(axum::Json(
    //         TryCreateManyRouteLogicResponseVariants::from(error),
    //     ));
    //     *response.status_mut() = status_code;
    //     return response;
    // }
}]
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
    #[generate_postgresql_crud_primary_key]
    pub column_6e88acb0_c566_4fef_8a09_66a41338cf36: postgresql_crud::postgresql_type::StdPrimitiveI64AsNotNullBigSerialInitializedByPostgresql,
    // pub column_8384082c_21de_4a21_95e9_a398644d8ea1: postgresql_crud::postgresql_type::SqlxTypesUuidUuidAsNotNullUuidV4InitializedByPostgresql,

    // //todo rename like postgresql types: NotNull prefix, not postfix
    pub object_animal_as_jsonb_not_null: ObjectAnimalAsJsonbNotNull,
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
#[derive(Debug, postgresql_crud::GeneratePostgresqlJsonType)]
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
    pub vec_of_std_primitive_i8_as_not_null_array_of_not_null_number: postgresql_crud::postgresql_json_type::VecOfStdPrimitiveI8AsNotNullArrayOfNotNullJsonbNumber,
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
    // pub std_option_option_std_vec_vec_std_primitive_i8: postgresql_crud::postgresql_json_type::StdOptionOptionStdVecVecStdPrimitiveI8,
    pub option_vec_of_std_primitive_i8_as_nullable_array_of_not_null_number: postgresql_crud::postgresql_json_type::OptionVecOfStdPrimitiveI8AsNullableArrayOfNotNullJsonbNumber,
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
    pub vec_of_option_std_primitive_i8_as_not_null_array_of_nullable_jsonb_number: postgresql_crud::postgresql_json_type::VecOfOptionStdPrimitiveI8AsNotNullArrayOfNullableJsonbNumber,
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
    pub option_vec_of_option_std_primitive_i8_as_nullable_array_of_nullable_number: postgresql_crud::postgresql_json_type::OptionVecOfOptionStdPrimitiveI8AsNullableArrayOfNullableJsonbNumber,
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

    // pub object_doggie: ObjectDoggie,
    // pub std_option_option_object: StdOptionOptionObjectDoggie,
    // pub std_vec_vec_object: StdVecVecObjectWithIdDoggie,
    // pub std_option_option_std_vec_vec_object: StdOptionOptionStdVecVecObjectWithIdDoggie,

    // pub std_vec_vec_object_with_id: StdVecVecObjectWithIdDoggie,
    // pub std_option_option_std_vec_vec_object_with_id: StdOptionOptionStdVecVecObjectWithIdDoggie
}
// /////////////////////////////////////////
// #[derive(
//     Debug,
//     Clone,
//     PartialEq,
//     Default,
//     serde::Serialize,
//     serde::Deserialize,
//     utoipa::ToSchema,
//     schemars::JsonSchema,
//     postgresql_crud::GeneratePostgresqlJsonType
// )] //user type must implement utoipa::ToSchema trait
// pub struct Doggie {
//     pub std_primitive_i8: postgresql_crud::postgresql_json_type::StdPrimitiveI8,
//     pub std_option_option_std_primitive_i8: postgresql_crud::postgresql_json_type::StdOptionOptionStdPrimitiveI8,
//     pub std_vec_vec_std_primitive_i8: postgresql_crud::postgresql_json_type::StdVecVecStdPrimitiveI8,
//     pub std_option_option_std_vec_vec_std_primitive_i8: postgresql_crud::postgresql_json_type::StdOptionOptionStdVecVecStdPrimitiveI8,
//     pub std_vec_vec_std_option_option_std_primitive_i8: postgresql_crud::postgresql_json_type::StdVecVecStdOptionOptionStdPrimitiveI8,
//     pub std_option_option_std_vec_vec_std_option_option_std_primitive_i8: postgresql_crud::postgresql_json_type::StdOptionOptionStdVecVecStdOptionOptionStdPrimitiveI8,
// }
///////////////////////////////////////////////////
#[cfg(test)]
mod tests {
    #[test]
    fn test_size_of() {
        assert_eq!(std::mem::size_of::<crate::repositories_types::server::routes::api::example::Example>(), 0);
    }
}
//////////////////
