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
#[derive(Debug
    , postgresql_crud::GeneratePostgresqlJsonObjectType
)]
#[postgresql_crud::postgresql_json_object_type_pattern{[
    {
        "not_null_or_nullable": "NotNull",
        "postgresql_json_type_pattern": "Standart",
        "trait_gen": "PostgresqlTypeAndPostgresqlJsonType"//todo make it PostgresqlType. PostgresqlTypeAndPostgresqlJsonType now just for testing
    }
]}]
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

    pub column_7bd2f76f_276c_4855_8ee0_4b6ce0ac5015: postgresql_crud::postgresql_json_type::StdPrimitiveI8AsNotNullJsonbNumber,
    pub column_1495baea_b926_4e20_a223_5b7a1f06c211: postgresql_crud::postgresql_json_type::OptionStdPrimitiveI8AsNullableJsonbNumber,



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
    pub option_doggie_as_nullable_jsonb_object: OptionDoggieAsNullableJsonbObject,
    pub vec_of_doggie_with_id_as_not_null_array_of_not_null_jsonb_object_with_id: VecOfDoggieWithIdAsNotNullArrayOfNotNullJsonbObjectWithId,
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
#[postgresql_crud::postgresql_json_object_type_pattern{[
    {
        "not_null_or_nullable": "NotNull",
        "postgresql_json_type_pattern": "Standart",
        "trait_gen": "PostgresqlTypeAndPostgresqlJsonType"//todo make it PostgresqlType. PostgresqlTypeAndPostgresqlJsonType now just for testing
    }
]}]
pub struct Doggie {
    pub column_7bd2f76f_276c_4855_8ee0_4b6ce0ac5015: postgresql_crud::postgresql_json_type::StdPrimitiveI8AsNotNullJsonbNumber,
    pub column_f85f2f57_f85b_4126_be0f_cb5830f0475d: postgresql_crud::postgresql_json_type::StdPrimitiveI16AsNotNullJsonbNumber,
}
////////////////////////////
#[derive(Debug)]
pub struct VecOfDoggieWithIdAsNotNullArrayOfNotNullJsonbObjectWithId;
#[derive(Debug, Clone, PartialEq, Default, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)]
pub struct VecOfDoggieWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdTableTypeDeclaration(pub std::vec::Vec<DoggieWithIdAsNotNullJsonbObjectWithIdTableTypeDeclaration>);
impl postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for VecOfDoggieWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdTableTypeDeclaration {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self(vec![postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()])
    }
}
impl VecOfDoggieWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdTableTypeDeclaration {
    pub fn create_table_column_query_part(column: &dyn std::fmt::Display, is_primary_key: std::primitive::bool) -> impl std::fmt::Display {
        format!(
            "{column} jsonb not null check (jsonb_matches_schema('{}', {column}))",
            serde_json::to_string(&schemars::schema_for!(VecOfDoggieWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdTableTypeDeclaration)).unwrap()
        )
    }
}
#[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfDoggieWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdCreate(pub std::vec::Vec<DoggieWithIdAsNotNullJsonbObjectWithIdCreate>);
impl std::fmt::Display for VecOfDoggieWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdCreate {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{:?}", self)
    }
}
impl error_occurence_lib::ToStdStringString for VecOfDoggieWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdCreate {
    fn to_std_string_string(&self) -> std::string::String {
        format!("{self:?}")
    }
}
impl postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for VecOfDoggieWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdCreate {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self(vec![postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()])
    }
}
impl VecOfDoggieWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdCreate {
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
#[derive(Debug, Clone, Default, PartialEq, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfDoggieWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdSelect {
    pub doggie_with_id_as_not_null_jsonb_object_with_id_select: DoggieWithIdAsNotNullJsonbObjectWithIdSelect,//here
    pub dimension1_pagination: postgresql_crud::Pagination,//here pub
}
impl sqlx::Type<sqlx::Postgres> for VecOfDoggieWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdSelect {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <sqlx::types::Json<VecOfDoggieWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdSelect> as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> bool {
        <sqlx::types::Json<VecOfDoggieWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdSelect> as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
impl sqlx::Decode<'_, sqlx::Postgres> for VecOfDoggieWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdSelect {
    fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        match <sqlx::types::Json<VecOfDoggieWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdSelect> as sqlx::Decode<sqlx::Postgres>>::decode(value) {
            Ok(value) => Ok(value.0),
            Err(error) => Err(error),
        }
    }
}
impl postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for VecOfDoggieWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdSelect {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        //here
        Self {
            doggie_with_id_as_not_null_jsonb_object_with_id_select: postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
            dimension1_pagination: postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
        }
    }
}
impl VecOfDoggieWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdSelect {
    fn select_query_part(&self, field_ident: &std::primitive::str, column_name_and_maybe_field_getter: &std::primitive::str, column_name_and_maybe_field_getter_for_error_message: &std::primitive::str, is_postgresql_type: std::primitive::bool) -> std::string::String {
        let doggie_with_id_as_not_null_jsonb_object_with_id_select_select_query_part = self.doggie_with_id_as_not_null_jsonb_object_with_id_select.select_query_part(
            field_ident,
            &"value",
            &"value",
            true
        );//here todo rename is_postgresql_type to is need to wrap into jsonb_build_object field
        //here
        let dimension1_start = self.dimension1_pagination.start();
        let dimension1_end = self.dimension1_pagination.end();
        format!("jsonb_build_object('{field_ident}',jsonb_build_object('value',(select jsonb_agg(({doggie_with_id_as_not_null_jsonb_object_with_id_select_select_query_part})) from jsonb_array_elements((select {column_name_and_maybe_field_getter}->'{field_ident}')) with ordinality where ordinality between {dimension1_start} and {dimension1_end})))")
    }
}
//////////////////////////////////////////
//add
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub enum VecOfDoggieWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdWhereElement {
    Equal(postgresql_crud::where_element_filters::PostgresqlJsonTypeWhereElementEqual<VecOfDoggieWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdTableTypeDeclaration>),
}
impl<'a> postgresql_crud::PostgresqlTypeWhereFilter<'a> for VecOfDoggieWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdWhereElement {
    fn query_part(&self, increment: &mut std::primitive::u64, column: &dyn std::fmt::Display, is_need_to_add_logical_operator: std::primitive::bool) -> Result<std::string::String, postgresql_crud::QueryPartErrorNamed> {
        match &self {
            Self::Equal(value) => postgresql_crud::PostgresqlTypeWhereFilter::query_part(value, increment, column, is_need_to_add_logical_operator),
        }
    }
    fn query_bind(self, query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
        match self {
            Self::Equal(value) => postgresql_crud::PostgresqlTypeWhereFilter::query_bind(value, query),
        }
    }
}
impl error_occurence_lib::ToStdStringString for VecOfDoggieWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdWhereElement {
    fn to_std_string_string(&self) -> std::string::String {
        format!("{self:#?}")
    }
}
impl postgresql_crud::AllEnumVariantsArrayDefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for VecOfDoggieWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdWhereElement {
    fn all_enum_variants_array_default_but_std_option_option_is_always_some_and_std_vec_vec_always_contains_one_element() -> std::vec::Vec<Self> {
        vec![Self::Equal(postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element())]
    }
}
///////////////////////////////////////////
#[derive(Debug, Clone, PartialEq, Default, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)]//here add Deserialize
pub struct VecOfDoggieWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdRead(pub std::vec::Vec<DoggieWithIdAsNotNullJsonbObjectWithIdRead>);//here 
impl postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for VecOfDoggieWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdRead {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self(vec![postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()])
    }
}
impl sqlx::Type<sqlx::Postgres> for VecOfDoggieWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdRead {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <sqlx::types::Json<VecOfDoggieWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdRead> as sqlx::Type<sqlx::Postgres>>::type_info()
    }
    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> bool {
        <sqlx::types::Json<VecOfDoggieWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdRead> as sqlx::Type<sqlx::Postgres>>::compatible(ty)
    }
}
impl sqlx::Decode<'_, sqlx::Postgres> for VecOfDoggieWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdRead {
    fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        match <sqlx::types::Json<VecOfDoggieWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdRead> as sqlx::Decode<sqlx::Postgres>>::decode(value) {
            Ok(value) => Ok(value.0),
            Err(error) => Err(error),
        }
    }
}

//////////////////////////
//here //todo maybe write analog of  postgresql_crud::UniqueVec but with id
#[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct DoggieWithIdAsNotNullJsonbObjectWithIdUpdateHandle {
    pub id: <postgresql_crud::postgresql_json_type::UuidUuidAsNotNullJsonbString as postgresql_crud::PostgresqlJsonType>::Update,//is it update? maybe read?
    pub fields: DoggieAsNotNullJsonbObjectUpdate,
}
//////////////////////////v
#[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct StdVecVecObjectWithIdVecOfDoggieWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdJsonArrayChange {
    // #[serde(skip_serializing_if = "Vec::is_empty")]//here
    create: std::vec::Vec<DoggieWithIdAsNotNullJsonbObjectWithIdCreate>,//here maybe VecOfDoggieWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdCreate. was std::vec::Vec<VecOfDoggieWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdToCreateWithGeneratedId>
    // #[serde(skip_serializing_if = "Vec::is_empty")]//here
    update: std::vec::Vec<DoggieWithIdAsNotNullJsonbObjectWithIdUpdateHandle>,//here 
    // #[serde(skip_serializing_if = "Vec::is_empty")]//here
    delete: std::vec::Vec<<postgresql_crud::postgresql_json_type::UuidUuidAsNotNullJsonbString as postgresql_crud::PostgresqlJsonType>::Update>,
}
#[derive(Debug, serde::Serialize, serde :: Deserialize, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum StdVecVecObjectWithIdVecOfDoggieWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdJsonArrayChangeTryNewErrorNamed {
    CreateUpdateDeleteCheckFieldsAreEmpty {
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
impl StdVecVecObjectWithIdVecOfDoggieWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdJsonArrayChange {
    pub fn try_new(
        create: std::vec::Vec<DoggieWithIdAsNotNullJsonbObjectWithIdCreate>,//here
        update: std::vec::Vec<DoggieWithIdAsNotNullJsonbObjectWithIdUpdateHandle>,//here
        delete: std::vec::Vec<<postgresql_crud::postgresql_json_type::UuidUuidAsNotNullJsonbString as postgresql_crud::PostgresqlJsonType>::Update>,
    ) -> Result<Self, StdVecVecObjectWithIdVecOfDoggieWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdJsonArrayChangeTryNewErrorNamed> {
        if create.is_empty() && update.is_empty() && delete.is_empty() {
            return Err(StdVecVecObjectWithIdVecOfDoggieWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdJsonArrayChangeTryNewErrorNamed::CreateUpdateDeleteCheckFieldsAreEmpty { code_occurence: error_occurence_lib::code_occurence!() });
        }
        {
            let update_acc = {
                let mut update_acc = vec![];
                for element in &update {
                    let id = &element.id;
                    if update_acc.contains(&id) {
                        return Err(StdVecVecObjectWithIdVecOfDoggieWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdJsonArrayChangeTryNewErrorNamed::NotUniqueIdInJsonUpdateArray {
                            error: format!("custom serde error deserializing StdVecVecObjectWithIdVecOfDoggieWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdJsonArrayChange: not unique id in json update array: {}", id),//here was id.0
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
                        return Err(StdVecVecObjectWithIdVecOfDoggieWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdJsonArrayChangeTryNewErrorNamed::NotUniqueIdInJsonDeleteArray {
                            error: format!("custom serde error deserializing StdVecVecObjectWithIdVecOfDoggieWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdJsonArrayChange: not unique id in json delete array: {}", element),//here was element.0
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
                    return Err(StdVecVecObjectWithIdVecOfDoggieWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdJsonArrayChangeTryNewErrorNamed::NotUniqueIdInJsonUpdateAndDeleteArrays {
                        error: format!(
                            "custom serde error deserializing StdVecVecObjectWithIdVecOfDoggieWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdJsonArrayChange: not unique id in json update and delete arrays: {}",
                            element//here was element.0
                        ),
                        code_occurence: error_occurence_lib::code_occurence!(),
                    });
                }
            }
        }
        Ok(Self { create, update, delete })
    }
}
impl<'de> serde::Deserialize<'de> for StdVecVecObjectWithIdVecOfDoggieWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdJsonArrayChange {
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
            fn expecting(&self, __formatter: &mut serde::__private::Formatter<'_>) -> serde::__private::fmt::Result {
                serde::__private::Formatter::write_str(__formatter, "field identifier")
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
            marker: serde::__private::PhantomData<StdVecVecObjectWithIdVecOfDoggieWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdJsonArrayChange>,
            lifetime: serde::__private::PhantomData<&'de ()>,
        }
        impl<'de> serde::de::Visitor<'de> for __Visitor<'de> {
            type Value = StdVecVecObjectWithIdVecOfDoggieWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdJsonArrayChange;
            fn expecting(&self, __formatter: &mut serde::__private::Formatter<'_>) -> serde::__private::fmt::Result {
                serde::__private::Formatter::write_str(__formatter, "tuple struct StdVecVecObjectWithIdVecOfDoggieWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdJsonArrayChange")
            }
            #[inline]
            fn visit_seq<__A>(self, mut __seq: __A) -> serde::__private::Result<Self::Value, __A::Error>
            where
                __A: serde::de::SeqAccess<'de>,
            {
                let __field0 = match serde::de::SeqAccess::next_element::<std::vec::Vec<DoggieWithIdAsNotNullJsonbObjectWithIdCreate>>(&mut __seq)? {//here was VecOfDoggieWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdToCreateWithGeneratedId
                    serde::__private::Some(__value) => __value,
                    serde::__private::None => {
                        vec![]
                    }
                };
                let __field1 = match serde::de::SeqAccess::next_element::<std::vec::Vec<DoggieWithIdAsNotNullJsonbObjectWithIdUpdateHandle>>(&mut __seq)? {//here was DoggieWithIdAsNotNullJsonbObjectWithIdUpdateElement
                    serde::__private::Some(__value) => __value,
                    serde::__private::None => {
                        vec![]
                    }
                };
                let __field2 = match serde::de::SeqAccess::next_element::<std::vec::Vec<<postgresql_crud::postgresql_json_type::UuidUuidAsNotNullJsonbString as postgresql_crud::PostgresqlJsonType>::Update>>(&mut __seq)? {
                    serde::__private::Some(__value) => __value,
                    serde::__private::None => {
                        vec![]
                    }
                };
                match StdVecVecObjectWithIdVecOfDoggieWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdJsonArrayChange::try_new(__field0, __field1, __field2) {
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
                let mut __field0: serde::__private::Option<std::vec::Vec<DoggieWithIdAsNotNullJsonbObjectWithIdCreate>> = serde::__private::None;//here was VecOfDoggieWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdToCreateWithGeneratedId
                let mut __field1: serde::__private::Option<std::vec::Vec<DoggieWithIdAsNotNullJsonbObjectWithIdUpdateHandle>> = serde::__private::None;//here was DoggieWithIdAsNotNullJsonbObjectWithIdUpdateElement
                let mut __field2: serde::__private::Option<std::vec::Vec<<postgresql_crud::postgresql_json_type::UuidUuidAsNotNullJsonbString as postgresql_crud::PostgresqlJsonType>::Update>> = serde::__private::None;
                while let serde::__private::Some(__key) = serde::de::MapAccess::next_key::<__Field>(&mut __map)? {
                    match __key {
                        __Field::__field0 => {
                            if serde::__private::Option::is_some(&__field0) {
                                return serde::__private::Err(<__A::Error as serde::de::Error>::duplicate_field("create"));
                            }
                            __field0 = serde::__private::Some(serde::de::MapAccess::next_value::<std::vec::Vec<DoggieWithIdAsNotNullJsonbObjectWithIdCreate>>(&mut __map)?);//here was VecOfDoggieWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdToCreateWithGeneratedId
                        }
                        __Field::__field1 => {
                            if serde::__private::Option::is_some(&__field1) {
                                return serde::__private::Err(<__A::Error as serde::de::Error>::duplicate_field("update"));
                            }
                            __field1 = serde::__private::Some(serde::de::MapAccess::next_value::<std::vec::Vec<DoggieWithIdAsNotNullJsonbObjectWithIdUpdateHandle>>(&mut __map)?);//here was DoggieWithIdAsNotNullJsonbObjectWithIdUpdateElement 
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
                    serde::__private::None => {
                        vec![]
                    }
                };
                let __field2 = match __field2 {
                    serde::__private::Some(__field2) => __field2,
                    serde::__private::None => {
                        vec![]
                    }
                };
                match StdVecVecObjectWithIdVecOfDoggieWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdJsonArrayChange::try_new(__field0, __field1, __field2) {
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
            "StdVecVecObjectWithIdVecOfDoggieWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdJsonArrayChange",
            FIELDS,
            __Visitor {
                marker: serde::__private::PhantomData::<StdVecVecObjectWithIdVecOfDoggieWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdJsonArrayChange>,
                lifetime: serde::__private::PhantomData,
            },
        )
    }
}
impl StdVecVecObjectWithIdVecOfDoggieWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdJsonArrayChange {
    fn update_query_part(&self, jsonb_set_accumulator: &std::primitive::str, jsonb_set_target: &std::primitive::str, jsonb_set_path: &std::primitive::str, increment: &mut std::primitive::u64) -> Result<std::string::String, postgresql_crud::QueryPartErrorNamed> {
        let update_query_part_acc = {
            let mut element_acc = std::string::String::from("elem");
            if self.update.is_empty() {
                element_acc
            } else {
                let mut update_query_part_acc = std::string::String::default();
                let generate_jsonb_set_target = |value: &std::primitive::str| format!("{jsonb_set_target}->'{value}'");
                for element_handle in &self.update {
                    let id_increment = match increment.checked_add(1) {
                        Some(value) => {
                            *increment = value;
                            increment.to_string()
                        }
                        None => {
                            return Err(postgresql_crud::QueryPartErrorNamed::CheckedAdd { code_occurence: error_occurence_lib::code_occurence!() });
                        }
                    };
                    // for element in element_handle.fields.to_vec() {//here
                    //     match element {
                    //         DoggieAsNotNullJsonbObjectUpdateElement::Column7Bd2F76F276C48558Ee04B6Ce0Ac5015(value) => {//here
                    //             match <postgresql_crud::postgresql_json_type::StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud::PostgresqlJsonType>::update_query_part(
                    //                 &value.value,
                    //                 &element_acc,
                    //                 &generate_jsonb_set_target("column_7bd2f76f_276c_4855_8ee0_4b6ce0ac5015"),
                    //                 "column_7bd2f76f_276c_4855_8ee0_4b6ce0ac5015",
                    //                 increment,
                    //             ) {
                    //                 Ok(value) => {
                    //                     element_acc = value;
                    //                 }
                    //                 Err(error) => {
                    //                     return Err(error);
                    //                 }
                    //             }
                    //         }
                    //         DoggieAsNotNullJsonbObjectUpdateElement::ColumnF85F2F57F85B4126Be0FCb5830F0475D(value) => {//here
                    //             match <postgresql_crud::postgresql_json_type::StdPrimitiveI16AsNotNullJsonbNumber as postgresql_crud::PostgresqlJsonType>::update_query_part(
                    //                 &value.value,
                    //                 &element_acc,
                    //                 &generate_jsonb_set_target("column_f85f2f57_f85b_4126_be0f_cb5830f0475d"),
                    //                 "column_f85f2f57_f85b_4126_be0f_cb5830f0475d",
                    //                 increment,
                    //             ) {
                    //                 Ok(value) => {
                    //                     element_acc = value;
                    //                 }
                    //                 Err(error) => {
                    //                     return Err(error);
                    //                 }
                    //             }
                    //         }
                    //     }
                    // }
                    ///////////////
                    //here added
                    match element_handle.fields.update_query_part(
                        &element_acc,
                        jsonb_set_target,
                        jsonb_set_path,
                        increment
                    ) {
                        Ok(value) => {
                            element_acc = value;
                        }
                        Err(error) => {
                            return Err(error);
                        }
                    }
                    ///////////////
                    update_query_part_acc.push_str(&format!("when elem ->> 'id' = ${id_increment} then {element_acc} "));
                }
                let _ = update_query_part_acc.pop();
                format!("case {update_query_part_acc} else elem end")
            }
        };
        let delete_query_part_acc = {
            let mut delete_query_part_acc = std::string::String::default();
            for _ in &self.delete {
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
            for element in &self.create {
                match element.create_query_part(increment) {
                    Ok(value) => {
                        create_query_part_acc.push_str(&format!("{value},"));
                    }
                    Err(error) => {
                        return Err(postgresql_crud::QueryPartErrorNamed::CheckedAdd { code_occurence: error_occurence_lib::code_occurence!() });
                    }
                }
            }
            let _ = create_query_part_acc.pop();
            create_query_part_acc
        };
        let maybe_where = if delete_query_part_acc.is_empty() { std::string::String::default() } else { format!(" where {delete_query_part_acc}") };
        let maybe_jsonb_build_array = if create_query_part_acc.is_empty() { std::string::String::default() } else { format!(" || jsonb_build_array({create_query_part_acc})") };
        Ok (format ! ("jsonb_set({jsonb_set_accumulator},'{{{jsonb_set_path}}}', case when jsonb_array_length({jsonb_set_target}) = 0 then '[]'::jsonb else (select coalesce((select jsonb_agg({update_query_part_acc}) from jsonb_array_elements({jsonb_set_target}) as elem {maybe_where}), '[]'::jsonb)) end {maybe_jsonb_build_array})"))
    }
    fn update_query_bind(self, mut query: sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments> {
        for element_handle in self.update {
            query = query.bind(element_handle.id.to_string());//here
            // for element in element_handle.fields.into_vec() {//here
            //     match element {
            //         DoggieAsNotNullJsonbObjectUpdateElement::Column7Bd2F76F276C48558Ee04B6Ce0Ac5015(value) => {//here
            //             query = <postgresql_crud::postgresql_json_type::StdPrimitiveI8AsNotNullJsonbNumber as postgresql_crud::PostgresqlJsonType>::update_query_bind(value.value, query);
            //         }
            //         DoggieAsNotNullJsonbObjectUpdateElement::ColumnF85F2F57F85B4126Be0FCb5830F0475D(value) => {//here
            //             query = <postgresql_crud::postgresql_json_type::StdPrimitiveI16AsNotNullJsonbNumber as postgresql_crud::PostgresqlJsonType>::update_query_bind(value.value, query);
            //         }
            //     }
            // }
            ///////////////////
            query = element_handle.fields.update_query_bind(query);
            //////////////////
        }
        for element in self.delete {
            query = query.bind(element.to_string());//here
        }
        for element in self.create {
            query = element.create_query_bind(query);
        }
        query
    }
}
impl postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for StdVecVecObjectWithIdVecOfDoggieWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdJsonArrayChange {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self {
            create: vec![postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()],
            update: vec![
                // postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()
                //todo remove 
                DoggieWithIdAsNotNullJsonbObjectWithIdUpdateHandle {
                    id: postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
                    fields: postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element(),
                }
            ],
            delete: vec![postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element()],
        }
    }
}
#[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema, schemars :: JsonSchema)]
pub struct VecOfDoggieWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdUpdate(pub StdVecVecObjectWithIdVecOfDoggieWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdJsonArrayChange);
impl postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement for VecOfDoggieWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdUpdate {
    fn default_but_option_is_always_some_and_vec_always_contains_one_element() -> Self {
        Self(postgresql_crud::DefaultButOptionIsAlwaysSomeAndVecAlwaysContainsOneElement::default_but_option_is_always_some_and_vec_always_contains_one_element())
    }
}
impl VecOfDoggieWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdUpdate {
    fn update_query_part(&self, jsonb_set_accumulator: &std::primitive::str, jsonb_set_target: &std::primitive::str, jsonb_set_path: &std::primitive::str, increment: &mut std::primitive::u64) -> Result<std::string::String, postgresql_crud::QueryPartErrorNamed> {
        // todo!()
        self.0.update_query_part(jsonb_set_accumulator, jsonb_set_target, jsonb_set_path, increment)//here
    }
    fn update_query_bind(self, query: sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments> {
        // todo!()
        self.0.update_query_bind(query)//here
    }
}
impl postgresql_crud::PostgresqlJsonType for VecOfDoggieWithIdAsNotNullArrayOfNotNullJsonbObjectWithId {
    type TableTypeDeclaration = VecOfDoggieWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdTableTypeDeclaration;
    type Create = VecOfDoggieWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdCreate;
    fn create_query_part(value: &Self::Create, increment: &mut std::primitive::u64) -> Result<std::string::String, postgresql_crud::QueryPartErrorNamed> {
        value.create_query_part(increment)
    }
    fn create_query_bind(value: Self::Create, query: sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments> {
        value.create_query_bind(query)
    }
    type Select = VecOfDoggieWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdSelect;
    fn select_query_part(value: &Self::Select, field_ident: &std::primitive::str, column_name_and_maybe_field_getter: &std::primitive::str, column_name_and_maybe_field_getter_for_error_message: &std::primitive::str, is_postgresql_type: std::primitive::bool) -> std::string::String {
        value.select_query_part(field_ident, column_name_and_maybe_field_getter, column_name_and_maybe_field_getter_for_error_message, is_postgresql_type)
    }
    type WhereElement = VecOfDoggieWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdWhereElement;
    type Read = VecOfDoggieWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdRead;
    type Update = VecOfDoggieWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdUpdate;
    fn update_query_part(value: &Self::Update, jsonb_set_accumulator: &std::primitive::str, jsonb_set_target: &std::primitive::str, jsonb_set_path: &std::primitive::str, increment: &mut std::primitive::u64) -> Result<std::string::String, postgresql_crud::QueryPartErrorNamed> {
        value.update_query_part(jsonb_set_accumulator, jsonb_set_target, jsonb_set_path, increment)
    }
    fn update_query_bind(value: Self::Update, query: sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments> {
        value.update_query_bind(query)
    }
}
impl postgresql_crud::PostgresqlType for VecOfDoggieWithIdAsNotNullArrayOfNotNullJsonbObjectWithId {
    type TableTypeDeclaration = VecOfDoggieWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdTableTypeDeclaration;
    type Create = VecOfDoggieWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdCreate;
    fn create_query_part(value: &Self::Create, increment: &mut std::primitive::u64) -> Result<std::string::String, postgresql_crud::QueryPartErrorNamed> {
        value.create_query_part(increment)
    }
    fn create_query_bind(value: Self::Create, query: sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments> {
        value.create_query_bind(query)
    }
    type Select = VecOfDoggieWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdSelect;
    fn select_query_part(value: &Self::Select, column: &std::primitive::str) -> std::string::String {
        format!("{} as {column}", {
            let field_ident = column;
            let column_name_and_maybe_field_getter = column;
            let column_name_and_maybe_field_getter_for_error_message = column;
            let is_postgresql_type = true;
            value.select_query_part(field_ident, column_name_and_maybe_field_getter, column_name_and_maybe_field_getter_for_error_message, is_postgresql_type)
        })
    }
    type WhereElement = VecOfDoggieWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdWhereElement;
    type Read = VecOfDoggieWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdRead;
    type Update = VecOfDoggieWithIdAsNotNullArrayOfNotNullJsonbObjectWithIdUpdate;
    fn update_query_part(value: &Self::Update, jsonb_set_accumulator: &std::primitive::str, jsonb_set_target: &std::primitive::str, jsonb_set_path: &std::primitive::str, increment: &mut std::primitive::u64) -> Result<std::string::String, postgresql_crud::QueryPartErrorNamed> {
        value.update_query_part(jsonb_set_accumulator, jsonb_set_target, jsonb_set_path, increment)
    }
    fn update_query_bind<'a>(value: Self::Update, query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
        value.update_query_bind(query)
    }
}