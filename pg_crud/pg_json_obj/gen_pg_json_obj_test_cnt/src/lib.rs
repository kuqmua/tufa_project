#[allow(unused_qualifications)]
#[allow(clippy::absolute_paths)]
#[allow(clippy::arbitrary_source_item_ordering)]
mod obj_example_gen_pg_json_obj_mod {
    #[derive(Debug, Clone, Copy)]
    pub struct ObjExampleAsNnJsonbObj;
    #[derive(Debug, Clone, Copy)]
    pub struct ObjExampleWithIdAsNnJsonbObjWithId;
    #[allow(clippy::arbitrary_source_item_ordering)]
    #[cfg(feature = "test-utils")]
    impl ObjExampleWithIdAsNnJsonbObjWithId {
        fn rd_ids_and_cr_into_wh_eq(
            rd_ids: ObjExampleWithIdAsNnJsonbObjWithIdRdIds,
            cr: ObjExampleWithIdAsNnJsonbObjWithIdCr,
        ) -> ObjExampleWithIdAsNnJsonbObjWithIdWh {
            ObjExampleWithIdAsNnJsonbObjWithIdWh :: Eq (pg_crud :: PgJsonWhEq { oprtr : pg_crud :: Oprtr :: Or , v : ObjExampleWithIdAsNnJsonbObjWithIdTt :: new (< pg_crud :: UuidUuidAsNnJsonbString as pg_crud :: PgJsonTestCases > :: rd_ids_and_cr_into_tt (rd_ids . 0. v . id , pg_crud :: DfltSomeOneEl :: dflt_some_one_el ()) , < pg_crud :: I8AsNnJsonbNbr as pg_crud :: PgJsonTestCases > :: rd_ids_and_cr_into_tt (rd_ids . 0. v . field_0 , cr . field_0) , < pg_crud :: OptI8AsNlJsonbNbr as pg_crud :: PgJsonTestCases > :: rd_ids_and_cr_into_tt (rd_ids . 0. v . field_1 , cr . field_1) , < pg_crud :: VecOfI8AsNnArrOfNnJsonbNbr as pg_crud :: PgJsonTestCases > :: rd_ids_and_cr_into_tt (rd_ids . 0. v . field_2 , cr . field_2)) , })
        }
        fn rd_ids_and_cr_into_vec_wh_eq_using_fields(
            rd_ids: ObjExampleWithIdAsNnJsonbObjWithIdRdIds,
            cr: ObjExampleWithIdAsNnJsonbObjWithIdCr,
        ) -> pg_crud::NotEmptyUnqVec<ObjExampleWithIdAsNnJsonbObjWithIdWh> {
            pg_crud :: NotEmptyUnqVec :: try_new (vec ! [ObjExampleWithIdAsNnJsonbObjWithIdWh :: Id (pg_crud :: PgTypeWh :: new (pg_crud :: Oprtr :: And , < pg_crud :: UuidUuidAsNnJsonbString as pg_crud :: PgJsonTestCases > :: rd_ids_and_cr_into_vec_wh_eq_using_fields (rd_ids . 0. v . id , pg_crud :: DfltSomeOneEl :: dflt_some_one_el ()) ,) ,) , ObjExampleWithIdAsNnJsonbObjWithIdWh :: Field0 (pg_crud :: PgTypeWh :: new (pg_crud :: Oprtr :: And , < pg_crud :: I8AsNnJsonbNbr as pg_crud :: PgJsonTestCases > :: rd_ids_and_cr_into_vec_wh_eq_using_fields (rd_ids . 0. v . field_0 , cr . field_0) ,) ,) , ObjExampleWithIdAsNnJsonbObjWithIdWh :: Field1 (pg_crud :: PgTypeWh :: new (pg_crud :: Oprtr :: And , < pg_crud :: OptI8AsNlJsonbNbr as pg_crud :: PgJsonTestCases > :: rd_ids_and_cr_into_vec_wh_eq_using_fields (rd_ids . 0. v . field_1 , cr . field_1) ,) ,) , ObjExampleWithIdAsNnJsonbObjWithIdWh :: Field2 (pg_crud :: PgTypeWh :: new (pg_crud :: Oprtr :: And , < pg_crud :: VecOfI8AsNnArrOfNnJsonbNbr as pg_crud :: PgJsonTestCases > :: rd_ids_and_cr_into_vec_wh_eq_using_fields (rd_ids . 0. v . field_2 , cr . field_2) ,) ,)]) . expect ("5473d8c4")
        }
        fn rd_ids_and_cr_into_vec_wh_eq_to_json_field(
            rd_ids: ObjExampleWithIdAsNnJsonbObjWithIdRdIds,
            cr: ObjExampleWithIdAsNnJsonbObjWithIdCr,
        ) -> pg_crud::NotEmptyUnqVec<ObjExampleWithIdAsNnJsonbObjWithIdWh> {
            pg_crud :: NotEmptyUnqVec :: try_new (vec ! [ObjExampleWithIdAsNnJsonbObjWithIdWh :: Id (pg_crud :: PgTypeWh :: new (pg_crud :: Oprtr :: Or , < pg_crud :: UuidUuidAsNnJsonbString as pg_crud :: PgJsonTestCases > :: rd_ids_and_cr_into_vec_wh_eq_to_json_field (rd_ids . 0. v . id , pg_crud :: DfltSomeOneEl :: dflt_some_one_el ()) ,) ,) , ObjExampleWithIdAsNnJsonbObjWithIdWh :: Field0 (pg_crud :: PgTypeWh :: new (pg_crud :: Oprtr :: Or , < pg_crud :: I8AsNnJsonbNbr as pg_crud :: PgJsonTestCases > :: rd_ids_and_cr_into_vec_wh_eq_to_json_field (rd_ids . 0. v . field_0 , cr . field_0) ,) ,) , ObjExampleWithIdAsNnJsonbObjWithIdWh :: Field1 (pg_crud :: PgTypeWh :: new (pg_crud :: Oprtr :: Or , < pg_crud :: OptI8AsNlJsonbNbr as pg_crud :: PgJsonTestCases > :: rd_ids_and_cr_into_vec_wh_eq_to_json_field (rd_ids . 0. v . field_1 , cr . field_1) ,) ,) , ObjExampleWithIdAsNnJsonbObjWithIdWh :: Field2 (pg_crud :: PgTypeWh :: new (pg_crud :: Oprtr :: Or , < pg_crud :: VecOfI8AsNnArrOfNnJsonbNbr as pg_crud :: PgJsonTestCases > :: rd_ids_and_cr_into_vec_wh_eq_to_json_field (rd_ids . 0. v . field_2 , cr . field_2) ,) ,)]) . expect ("221a4c55")
        }
    }
    #[cfg(feature = "test-utils")]
    const _: () = {
        let _ = ObjExampleWithIdAsNnJsonbObjWithId::rd_ids_and_cr_into_wh_eq;
        let _ = ObjExampleWithIdAsNnJsonbObjWithId::rd_ids_and_cr_into_wh_eq;
        let _ = ObjExampleWithIdAsNnJsonbObjWithId::rd_ids_and_cr_into_vec_wh_eq_using_fields;
        let _ = ObjExampleWithIdAsNnJsonbObjWithId::rd_ids_and_cr_into_vec_wh_eq_using_fields;
        let _ = ObjExampleWithIdAsNnJsonbObjWithId::rd_ids_and_cr_into_vec_wh_eq_to_json_field;
        let _ = ObjExampleWithIdAsNnJsonbObjWithId::rd_ids_and_cr_into_vec_wh_eq_to_json_field;
    };
    #[allow(clippy::arbitrary_source_item_ordering)]
    #[derive(
        Debug,
        Clone,
        PartialEq,
        serde :: Serialize,
        serde :: Deserialize,
        utoipa :: ToSchema,
        schemars :: JsonSchema,
    )]
    pub struct ObjExampleAsNnJsonbObjTt {
        field_0: <pg_crud::I8AsNnJsonbNbr as pg_crud::PgJson>::Tt,
        field_1: <pg_crud::OptI8AsNlJsonbNbr as pg_crud::PgJson>::Tt,
        field_2: <pg_crud::VecOfI8AsNnArrOfNnJsonbNbr as pg_crud::PgJson>::Tt,
    }
    impl ObjExampleAsNnJsonbObjTt {
        #[must_use]
        pub const fn new(
            field_0: <pg_crud::I8AsNnJsonbNbr as pg_crud::PgJson>::Tt,
            field_1: <pg_crud::OptI8AsNlJsonbNbr as pg_crud::PgJson>::Tt,
            field_2: <pg_crud::VecOfI8AsNnArrOfNnJsonbNbr as pg_crud::PgJson>::Tt,
        ) -> Self {
            Self {
                field_0,
                field_1,
                field_2,
            }
        }
    }
    impl pg_crud::DfltSomeOneEl for ObjExampleAsNnJsonbObjTt {
        fn dflt_some_one_el() -> Self {
            Self {
                field_0: pg_crud::DfltSomeOneEl::dflt_some_one_el(),
                field_1: pg_crud::DfltSomeOneEl::dflt_some_one_el(),
                field_2: pg_crud::DfltSomeOneEl::dflt_some_one_el(),
            }
        }
    }
    impl sqlx::Type<sqlx::Postgres> for ObjExampleAsNnJsonbObjTt {
        fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> bool {
            <sqlx::types::Json<Self> as sqlx::Type<sqlx::Postgres>>::compatible(ty)
        }
        fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
            <sqlx::types::Json<Self> as sqlx::Type<sqlx::Postgres>>::type_info()
        }
    }
    impl sqlx::Encode<'_, sqlx::Postgres> for ObjExampleAsNnJsonbObjTt {
        fn encode_by_ref(
            &self,
            buf: &mut sqlx::postgres::PgArgumentBuffer,
        ) -> Result<sqlx::encode::IsNull, Box<dyn std::error::Error + Send + Sync>> {
            sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&sqlx::types::Json(self), buf)
        }
    }
    #[allow(clippy::arbitrary_source_item_ordering)]
    #[derive(
        Debug,
        Clone,
        PartialEq,
        serde :: Serialize,
        serde :: Deserialize,
        utoipa :: ToSchema,
        schemars :: JsonSchema,
    )]
    pub struct ObjExampleWithIdAsNnJsonbObjWithIdTt {
        id: <pg_crud::UuidUuidAsNnJsonbString as pg_crud::PgJson>::Tt,
        field_0: <pg_crud::I8AsNnJsonbNbr as pg_crud::PgJson>::Tt,
        field_1: <pg_crud::OptI8AsNlJsonbNbr as pg_crud::PgJson>::Tt,
        field_2: <pg_crud::VecOfI8AsNnArrOfNnJsonbNbr as pg_crud::PgJson>::Tt,
    }
    impl ObjExampleWithIdAsNnJsonbObjWithIdTt {
        #[must_use]
        pub const fn new(
            id: <pg_crud::UuidUuidAsNnJsonbString as pg_crud::PgJson>::Tt,
            field_0: <pg_crud::I8AsNnJsonbNbr as pg_crud::PgJson>::Tt,
            field_1: <pg_crud::OptI8AsNlJsonbNbr as pg_crud::PgJson>::Tt,
            field_2: <pg_crud::VecOfI8AsNnArrOfNnJsonbNbr as pg_crud::PgJson>::Tt,
        ) -> Self {
            Self {
                id,
                field_0,
                field_1,
                field_2,
            }
        }
    }
    impl pg_crud::DfltSomeOneEl for ObjExampleWithIdAsNnJsonbObjWithIdTt {
        fn dflt_some_one_el() -> Self {
            Self {
                id: pg_crud::DfltSomeOneEl::dflt_some_one_el(),
                field_0: pg_crud::DfltSomeOneEl::dflt_some_one_el(),
                field_1: pg_crud::DfltSomeOneEl::dflt_some_one_el(),
                field_2: pg_crud::DfltSomeOneEl::dflt_some_one_el(),
            }
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        serde :: Serialize,
        serde :: Deserialize,
        utoipa :: ToSchema,
        schemars :: JsonSchema,
    )]
    pub struct ObjExampleAsNnJsonbObjCr {
        field_0: <pg_crud::I8AsNnJsonbNbr as pg_crud::PgJson>::Cr,
        field_1: <pg_crud::OptI8AsNlJsonbNbr as pg_crud::PgJson>::Cr,
        field_2: <pg_crud::VecOfI8AsNnArrOfNnJsonbNbr as pg_crud::PgJson>::Cr,
    }
    impl ObjExampleAsNnJsonbObjCr {
        #[must_use]
        pub const fn new(
            field_0: <pg_crud::I8AsNnJsonbNbr as pg_crud::PgJson>::Cr,
            field_1: <pg_crud::OptI8AsNlJsonbNbr as pg_crud::PgJson>::Cr,
            field_2: <pg_crud::VecOfI8AsNnArrOfNnJsonbNbr as pg_crud::PgJson>::Cr,
        ) -> Self {
            Self {
                field_0,
                field_1,
                field_2,
            }
        }
    }
    impl pg_crud::DfltSomeOneEl for ObjExampleAsNnJsonbObjCr {
        fn dflt_some_one_el() -> Self {
            Self {
                field_0: pg_crud::DfltSomeOneEl::dflt_some_one_el(),
                field_1: pg_crud::DfltSomeOneEl::dflt_some_one_el(),
                field_2: pg_crud::DfltSomeOneEl::dflt_some_one_el(),
            }
        }
    }
    impl sqlx::Type<sqlx::Postgres> for ObjExampleAsNnJsonbObjCr {
        fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> bool {
            <sqlx::types::Json<Self> as sqlx::Type<sqlx::Postgres>>::compatible(ty)
        }
        fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
            <sqlx::types::Json<Self> as sqlx::Type<sqlx::Postgres>>::type_info()
        }
    }
    impl sqlx::Encode<'_, sqlx::Postgres> for ObjExampleAsNnJsonbObjCr {
        fn encode_by_ref(
            &self,
            buf: &mut sqlx::postgres::PgArgumentBuffer,
        ) -> Result<sqlx::encode::IsNull, Box<dyn std::error::Error + Send + Sync>> {
            sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&sqlx::types::Json(self), buf)
        }
    }
    #[allow(clippy::arbitrary_source_item_ordering)]
    #[derive(
        Debug,
        Clone,
        PartialEq,
        serde :: Serialize,
        serde :: Deserialize,
        utoipa :: ToSchema,
        schemars :: JsonSchema,
    )]
    pub struct ObjExampleWithIdAsNnJsonbObjWithIdCr {
        field_0: <pg_crud::I8AsNnJsonbNbr as pg_crud::PgJson>::Cr,
        field_1: <pg_crud::OptI8AsNlJsonbNbr as pg_crud::PgJson>::Cr,
        field_2: <pg_crud::VecOfI8AsNnArrOfNnJsonbNbr as pg_crud::PgJson>::Cr,
    }
    impl ObjExampleWithIdAsNnJsonbObjWithIdCr {
        #[must_use]
        pub const fn new(
            field_0: <pg_crud::I8AsNnJsonbNbr as pg_crud::PgJson>::Cr,
            field_1: <pg_crud::OptI8AsNlJsonbNbr as pg_crud::PgJson>::Cr,
            field_2: <pg_crud::VecOfI8AsNnArrOfNnJsonbNbr as pg_crud::PgJson>::Cr,
        ) -> Self {
            Self {
                field_0,
                field_1,
                field_2,
            }
        }
    }
    impl pg_crud::DfltSomeOneEl for ObjExampleWithIdAsNnJsonbObjWithIdCr {
        fn dflt_some_one_el() -> Self {
            Self {
                field_0: pg_crud::DfltSomeOneEl::dflt_some_one_el(),
                field_1: pg_crud::DfltSomeOneEl::dflt_some_one_el(),
                field_2: pg_crud::DfltSomeOneEl::dflt_some_one_el(),
            }
        }
    }
    impl std::fmt::Display for ObjExampleAsNnJsonbObjCr {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{self:?}")
        }
    }
    impl loc_lib::ToErrString for ObjExampleAsNnJsonbObjCr {
        fn to_err_string(&self) -> String {
            format!("{self:#?}")
        }
    }
    impl std::fmt::Display for ObjExampleWithIdAsNnJsonbObjWithIdCr {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{self:?}")
        }
    }
    impl loc_lib::ToErrString for ObjExampleWithIdAsNnJsonbObjWithIdCr {
        fn to_err_string(&self) -> String {
            format!("{self:#?}")
        }
    }
    #[allow(clippy::arbitrary_source_item_ordering)]
    #[derive(Debug, Clone, PartialEq, serde :: Serialize)]
    pub struct ObjExampleAsNnJsonbObjCrForQuery {
        field_0: <pg_crud::I8AsNnJsonbNbr as pg_crud::PgJson>::CrForQuery,
        field_1: <pg_crud::OptI8AsNlJsonbNbr as pg_crud::PgJson>::CrForQuery,
        field_2: <pg_crud::VecOfI8AsNnArrOfNnJsonbNbr as pg_crud::PgJson>::CrForQuery,
    }
    impl From<ObjExampleAsNnJsonbObjCr> for ObjExampleAsNnJsonbObjCrForQuery {
        fn from(v: ObjExampleAsNnJsonbObjCr) -> Self {
            Self {
                field_0: <pg_crud::I8AsNnJsonbNbr as pg_crud::PgJson>::CrForQuery::from(v.field_0),
                field_1: <pg_crud::OptI8AsNlJsonbNbr as pg_crud::PgJson>::CrForQuery::from(
                    v.field_1,
                ),
                field_2: <pg_crud::VecOfI8AsNnArrOfNnJsonbNbr as pg_crud::PgJson>::CrForQuery::from(
                    v.field_2,
                ),
            }
        }
    }
    impl sqlx::Type<sqlx::Postgres> for ObjExampleAsNnJsonbObjCrForQuery {
        fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> bool {
            <sqlx::types::Json<Self> as sqlx::Type<sqlx::Postgres>>::compatible(ty)
        }
        fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
            <sqlx::types::Json<Self> as sqlx::Type<sqlx::Postgres>>::type_info()
        }
    }
    impl sqlx::Encode<'_, sqlx::Postgres> for ObjExampleAsNnJsonbObjCrForQuery {
        fn encode_by_ref(
            &self,
            buf: &mut sqlx::postgres::PgArgumentBuffer,
        ) -> Result<sqlx::encode::IsNull, Box<dyn std::error::Error + Send + Sync>> {
            sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&sqlx::types::Json(self), buf)
        }
    }
    #[allow(clippy::arbitrary_source_item_ordering)]
    #[derive(Debug, Clone, PartialEq, serde :: Serialize)]
    pub struct ObjExampleWithIdAsNnJsonbObjWithIdCrForQuery {
        id: <pg_crud::UuidUuidAsNnJsonbString as pg_crud::PgJson>::CrForQuery,
        field_0: <pg_crud::I8AsNnJsonbNbr as pg_crud::PgJson>::CrForQuery,
        field_1: <pg_crud::OptI8AsNlJsonbNbr as pg_crud::PgJson>::CrForQuery,
        field_2: <pg_crud::VecOfI8AsNnArrOfNnJsonbNbr as pg_crud::PgJson>::CrForQuery,
    }
    impl From<ObjExampleWithIdAsNnJsonbObjWithIdCr> for ObjExampleWithIdAsNnJsonbObjWithIdCrForQuery {
        fn from(v: ObjExampleWithIdAsNnJsonbObjWithIdCr) -> Self {
            Self {
                id: <pg_crud::UuidUuidAsNnJsonbString as pg_crud::PgJson>::CrForQuery::new(
                    uuid::Uuid::new_v4(),
                ),
                field_0: <pg_crud::I8AsNnJsonbNbr as pg_crud::PgJson>::CrForQuery::from(v.field_0),
                field_1: <pg_crud::OptI8AsNlJsonbNbr as pg_crud::PgJson>::CrForQuery::from(
                    v.field_1,
                ),
                field_2: <pg_crud::VecOfI8AsNnArrOfNnJsonbNbr as pg_crud::PgJson>::CrForQuery::from(
                    v.field_2,
                ),
            }
        }
    }
    #[allow(clippy::arbitrary_source_item_ordering)]
    #[derive(
        Debug,
        Clone,
        PartialEq,
        serde :: Serialize,
        serde :: Deserialize,
        utoipa :: ToSchema,
        schemars :: JsonSchema,
    )]
    pub struct ObjExampleAsNnJsonbObjSel(pg_crud::NotEmptyUnqVec<ObjExampleAsNnJsonbObjSelEl>);
    impl ObjExampleAsNnJsonbObjSel {
        #[must_use]
        pub const fn new(v: pg_crud::NotEmptyUnqVec<ObjExampleAsNnJsonbObjSelEl>) -> Self {
            Self(v)
        }
        fn sel_qp(
            &self,
            col_field: &str,
            col_field_for_er_msg: &str,
        ) -> Result<String, pg_crud::QpEr> {
            let mut acc_sel_qp = String::default();
            for el in self.0.to_vec() {
                if { use std :: fmt :: Write as _ ; write ! (acc_sel_qp , "{}||" , match el { ObjExampleAsNnJsonbObjSelEl :: Field0 (v_3c8acf6a) => match < pg_crud :: I8AsNnJsonbNbr as pg_crud :: PgJson > :: sel_qp (v_3c8acf6a , "field_0" , col_field , col_field_for_er_msg , false ,) { Ok (v_d54cf786) => v_d54cf786 , Err (er) => { return Err (er) ; } } , ObjExampleAsNnJsonbObjSelEl :: Field1 (v_3c8acf6a) => match < pg_crud :: OptI8AsNlJsonbNbr as pg_crud :: PgJson > :: sel_qp (v_3c8acf6a , "field_1" , col_field , col_field_for_er_msg , false ,) { Ok (v_d54cf786) => v_d54cf786 , Err (er) => { return Err (er) ; } } , ObjExampleAsNnJsonbObjSelEl :: Field2 (v_3c8acf6a) => match < pg_crud :: VecOfI8AsNnArrOfNnJsonbNbr as pg_crud :: PgJson > :: sel_qp (v_3c8acf6a , "field_2" , col_field , col_field_for_er_msg , false ,) { Ok (v_d54cf786) => v_d54cf786 , Err (er) => { return Err (er) ; } } }) } . is_err () { return Err (pg_crud :: QpEr :: WriteIntoBuffer { loc : loc_lib :: loc ! () }) ; }
            }
            let _: Option<char> = acc_sel_qp.pop();
            let _: Option<char> = acc_sel_qp.pop();
            Ok(acc_sel_qp)
        }
        fn sel_qp_pg_type(&self, col: &str) -> Result<String, pg_crud::QpEr> {
            self.sel_qp(col, col)
        }
    }
    impl sqlx::Type<sqlx::Postgres> for ObjExampleAsNnJsonbObjSel {
        fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> bool {
            <sqlx::types::Json<Self> as sqlx::Type<sqlx::Postgres>>::compatible(ty)
        }
        fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
            <sqlx::types::Json<Self> as sqlx::Type<sqlx::Postgres>>::type_info()
        }
    }
    impl sqlx::Decode<'_, sqlx::Postgres> for ObjExampleAsNnJsonbObjSel {
        fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
            match <sqlx::types::Json<Self> as sqlx::Decode<sqlx::Postgres>>::decode(value) {
                Ok(v) => Ok(v.0),
                Err(er) => Err(er),
            }
        }
    }
    impl pg_crud::DfltSomeOneEl for ObjExampleAsNnJsonbObjSel {
        fn dflt_some_one_el() -> Self {
            Self(pg_crud::DfltSomeOneEl::dflt_some_one_el())
        }
    }
    impl pg_crud::DfltSomeOneElMaxPageSize for ObjExampleAsNnJsonbObjSel {
        fn dflt_some_one_el_max_page_size() -> Self {
            Self(pg_crud::DfltSomeOneElMaxPageSize::dflt_some_one_el_max_page_size())
        }
    }
    #[allow(clippy::arbitrary_source_item_ordering)]
    #[derive(
        Debug,
        Clone,
        PartialEq,
        serde :: Serialize,
        serde :: Deserialize,
        utoipa :: ToSchema,
        schemars :: JsonSchema,
    )]
    pub enum ObjExampleAsNnJsonbObjSelEl {
        #[serde(rename(serialize = "field_0", deserialize = "field_0"))]
        Field0(<pg_crud::I8AsNnJsonbNbr as pg_crud::PgJson>::Sel),
        #[serde(rename(serialize = "field_1", deserialize = "field_1"))]
        Field1(<pg_crud::OptI8AsNlJsonbNbr as pg_crud::PgJson>::Sel),
        #[serde(rename(serialize = "field_2", deserialize = "field_2"))]
        Field2(<pg_crud::VecOfI8AsNnArrOfNnJsonbNbr as pg_crud::PgJson>::Sel),
    }
    impl loc_lib::ToErrString for ObjExampleAsNnJsonbObjSelEl {
        fn to_err_string(&self) -> String {
            format!("{self:?}")
        }
    }
    impl pg_crud::AllEnumVrtsArrDfltSomeOneEl for ObjExampleAsNnJsonbObjSelEl {
        fn all_vrts_dflt_some_one_el() -> Vec<Self> {
            vec![
                Self::Field0(pg_crud::DfltSomeOneEl::dflt_some_one_el()),
                Self::Field1(pg_crud::DfltSomeOneEl::dflt_some_one_el()),
                Self::Field2(pg_crud::DfltSomeOneEl::dflt_some_one_el()),
            ]
        }
    }
    impl pg_crud::AllEnumVrtsArrDfltSomeOneElMaxPageSize for ObjExampleAsNnJsonbObjSelEl {
        fn all_vrts_dflt_some_one_el_max_page_size() -> Vec<Self> {
            vec![
                Self::Field0(pg_crud::DfltSomeOneElMaxPageSize::dflt_some_one_el_max_page_size()),
                Self::Field1(pg_crud::DfltSomeOneElMaxPageSize::dflt_some_one_el_max_page_size()),
                Self::Field2(pg_crud::DfltSomeOneElMaxPageSize::dflt_some_one_el_max_page_size()),
            ]
        }
    }
    #[allow(clippy::arbitrary_source_item_ordering)]
    #[derive(
        Debug,
        Clone,
        PartialEq,
        serde :: Serialize,
        serde :: Deserialize,
        utoipa :: ToSchema,
        schemars :: JsonSchema,
    )]
    pub struct ObjExampleWithIdAsNnJsonbObjWithIdSel(
        pg_crud::NotEmptyUnqVec<ObjExampleWithIdAsNnJsonbObjWithIdSelEl>,
    );
    impl ObjExampleWithIdAsNnJsonbObjWithIdSel {
        #[must_use]
        pub const fn new(
            v: pg_crud::NotEmptyUnqVec<ObjExampleWithIdAsNnJsonbObjWithIdSelEl>,
        ) -> Self {
            Self(v)
        }
    }
    impl sqlx::Type<sqlx::Postgres> for ObjExampleWithIdAsNnJsonbObjWithIdSel {
        fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> bool {
            <sqlx::types::Json<Self> as sqlx::Type<sqlx::Postgres>>::compatible(ty)
        }
        fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
            <sqlx::types::Json<Self> as sqlx::Type<sqlx::Postgres>>::type_info()
        }
    }
    impl sqlx::Decode<'_, sqlx::Postgres> for ObjExampleWithIdAsNnJsonbObjWithIdSel {
        fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
            match <sqlx::types::Json<Self> as sqlx::Decode<sqlx::Postgres>>::decode(value) {
                Ok(v) => Ok(v.0),
                Err(er) => Err(er),
            }
        }
    }
    impl pg_crud::DfltSomeOneEl for ObjExampleWithIdAsNnJsonbObjWithIdSel {
        fn dflt_some_one_el() -> Self {
            Self(pg_crud::DfltSomeOneEl::dflt_some_one_el())
        }
    }
    impl pg_crud::DfltSomeOneElMaxPageSize for ObjExampleWithIdAsNnJsonbObjWithIdSel {
        fn dflt_some_one_el_max_page_size() -> Self {
            Self(pg_crud::DfltSomeOneElMaxPageSize::dflt_some_one_el_max_page_size())
        }
    }
    #[allow(clippy::arbitrary_source_item_ordering)]
    #[derive(
        Debug,
        Clone,
        PartialEq,
        serde :: Serialize,
        serde :: Deserialize,
        utoipa :: ToSchema,
        schemars :: JsonSchema,
    )]
    pub enum ObjExampleWithIdAsNnJsonbObjWithIdSelEl {
        #[serde(rename(serialize = "id", deserialize = "id"))]
        Id(<pg_crud::UuidUuidAsNnJsonbString as pg_crud::PgJson>::Sel),
        #[serde(rename(serialize = "field_0", deserialize = "field_0"))]
        Field0(<pg_crud::I8AsNnJsonbNbr as pg_crud::PgJson>::Sel),
        #[serde(rename(serialize = "field_1", deserialize = "field_1"))]
        Field1(<pg_crud::OptI8AsNlJsonbNbr as pg_crud::PgJson>::Sel),
        #[serde(rename(serialize = "field_2", deserialize = "field_2"))]
        Field2(<pg_crud::VecOfI8AsNnArrOfNnJsonbNbr as pg_crud::PgJson>::Sel),
    }
    impl loc_lib::ToErrString for ObjExampleWithIdAsNnJsonbObjWithIdSelEl {
        fn to_err_string(&self) -> String {
            format!("{self:?}")
        }
    }
    impl pg_crud::AllEnumVrtsArrDfltSomeOneEl for ObjExampleWithIdAsNnJsonbObjWithIdSelEl {
        fn all_vrts_dflt_some_one_el() -> Vec<Self> {
            vec![
                Self::Id(pg_crud::DfltSomeOneEl::dflt_some_one_el()),
                Self::Field0(pg_crud::DfltSomeOneEl::dflt_some_one_el()),
                Self::Field1(pg_crud::DfltSomeOneEl::dflt_some_one_el()),
                Self::Field2(pg_crud::DfltSomeOneEl::dflt_some_one_el()),
            ]
        }
    }
    impl pg_crud::AllEnumVrtsArrDfltSomeOneElMaxPageSize for ObjExampleWithIdAsNnJsonbObjWithIdSelEl {
        fn all_vrts_dflt_some_one_el_max_page_size() -> Vec<Self> {
            vec![
                Self::Id(pg_crud::DfltSomeOneElMaxPageSize::dflt_some_one_el_max_page_size()),
                Self::Field0(pg_crud::DfltSomeOneElMaxPageSize::dflt_some_one_el_max_page_size()),
                Self::Field1(pg_crud::DfltSomeOneElMaxPageSize::dflt_some_one_el_max_page_size()),
                Self::Field2(pg_crud::DfltSomeOneElMaxPageSize::dflt_some_one_el_max_page_size()),
            ]
        }
    }
    #[allow(clippy::arbitrary_source_item_ordering)]
    #[derive(
        Debug,
        Clone,
        PartialEq,
        serde :: Serialize,
        serde :: Deserialize,
        utoipa :: ToSchema,
        schemars :: JsonSchema,
    )]
    pub enum ObjExampleAsNnJsonbObjWh {
        Field0(pg_crud::PgTypeWh<<pg_crud::I8AsNnJsonbNbr as pg_crud::PgJson>::Wh>),
        Field1(pg_crud::PgTypeWh<<pg_crud::OptI8AsNlJsonbNbr as pg_crud::PgJson>::Wh>),
        Field2(pg_crud::PgTypeWh<<pg_crud::VecOfI8AsNnArrOfNnJsonbNbr as pg_crud::PgJson>::Wh>),
        Eq(pg_crud::PgJsonWhEq<<ObjExampleAsNnJsonbObj as pg_crud::PgJson>::Tt>),
    }
    #[allow(clippy::arbitrary_source_item_ordering)]
    impl<'lt> pg_crud::PgTypeWhFlt<'lt> for ObjExampleAsNnJsonbObjWh {
        fn qp(
            &self,
            incr: &mut u64,
            col: &dyn std::fmt::Display,
            add_oprtr: bool,
        ) -> Result<String, pg_crud::QpEr> {
            match &self {
                Self::Field0(v_b93ffc1d) => pg_crud::PgTypeWhFlt::qp(
                    v_b93ffc1d,
                    incr,
                    &format!("{col}->'field_0'"),
                    add_oprtr,
                ),
                Self::Field1(v_b93ffc1d) => pg_crud::PgTypeWhFlt::qp(
                    v_b93ffc1d,
                    incr,
                    &format!("{col}->'field_1'"),
                    add_oprtr,
                ),
                Self::Field2(v_b93ffc1d) => pg_crud::PgTypeWhFlt::qp(
                    v_b93ffc1d,
                    incr,
                    &format!("{col}->'field_2'"),
                    add_oprtr,
                ),
                Self::Eq(v_6781c7e3) => pg_crud::PgTypeWhFlt::qp(v_6781c7e3, incr, &col, add_oprtr),
            }
        }
        fn qb(
            self,
            query: sqlx::query::Query<'lt, sqlx::Postgres, sqlx::postgres::PgArguments>,
        ) -> Result<sqlx::query::Query<'lt, sqlx::Postgres, sqlx::postgres::PgArguments>, String>
        {
            match self {
                Self::Field0(v) => pg_crud::PgTypeWhFlt::qb(v, query),
                Self::Field1(v) => pg_crud::PgTypeWhFlt::qb(v, query),
                Self::Field2(v) => pg_crud::PgTypeWhFlt::qb(v, query),
                Self::Eq(v) => pg_crud::PgTypeWhFlt::qb(v, query),
            }
        }
    }
    impl loc_lib::ToErrString for ObjExampleAsNnJsonbObjWh {
        fn to_err_string(&self) -> String {
            format!("{self:?}")
        }
    }
    impl pg_crud::AllEnumVrtsArrDfltSomeOneEl for ObjExampleAsNnJsonbObjWh {
        fn all_vrts_dflt_some_one_el() -> Vec<Self> {
            vec![
                Self::Field0(pg_crud::DfltSomeOneEl::dflt_some_one_el()),
                Self::Field1(pg_crud::DfltSomeOneEl::dflt_some_one_el()),
                Self::Field2(pg_crud::DfltSomeOneEl::dflt_some_one_el()),
                Self::Eq(pg_crud::DfltSomeOneEl::dflt_some_one_el()),
            ]
        }
    }
    #[allow(clippy::arbitrary_source_item_ordering)]
    #[derive(
        Debug,
        Clone,
        PartialEq,
        serde :: Serialize,
        serde :: Deserialize,
        utoipa :: ToSchema,
        schemars :: JsonSchema,
    )]
    pub enum ObjExampleWithIdAsNnJsonbObjWithIdWh {
        Id(pg_crud::PgTypeWh<<pg_crud::UuidUuidAsNnJsonbString as pg_crud::PgJson>::Wh>),
        Field0(pg_crud::PgTypeWh<<pg_crud::I8AsNnJsonbNbr as pg_crud::PgJson>::Wh>),
        Field1(pg_crud::PgTypeWh<<pg_crud::OptI8AsNlJsonbNbr as pg_crud::PgJson>::Wh>),
        Field2(pg_crud::PgTypeWh<<pg_crud::VecOfI8AsNnArrOfNnJsonbNbr as pg_crud::PgJson>::Wh>),
        Eq(pg_crud::PgJsonWhEq<ObjExampleWithIdAsNnJsonbObjWithIdTt>),
    }
    #[allow(clippy::arbitrary_source_item_ordering)]
    impl<'lt> pg_crud::PgTypeWhFlt<'lt> for ObjExampleWithIdAsNnJsonbObjWithIdWh {
        fn qp(
            &self,
            incr: &mut u64,
            col: &dyn std::fmt::Display,
            add_oprtr: bool,
        ) -> Result<String, pg_crud::QpEr> {
            match &self {
                Self::Id(v_b93ffc1d) => {
                    pg_crud::PgTypeWhFlt::qp(v_b93ffc1d, incr, &format!("{col}->'id'"), add_oprtr)
                }
                Self::Field0(v_b93ffc1d) => pg_crud::PgTypeWhFlt::qp(
                    v_b93ffc1d,
                    incr,
                    &format!("{col}->'field_0'"),
                    add_oprtr,
                ),
                Self::Field1(v_b93ffc1d) => pg_crud::PgTypeWhFlt::qp(
                    v_b93ffc1d,
                    incr,
                    &format!("{col}->'field_1'"),
                    add_oprtr,
                ),
                Self::Field2(v_b93ffc1d) => pg_crud::PgTypeWhFlt::qp(
                    v_b93ffc1d,
                    incr,
                    &format!("{col}->'field_2'"),
                    add_oprtr,
                ),
                Self::Eq(v_31e7fe47) => pg_crud::PgTypeWhFlt::qp(v_31e7fe47, incr, &col, add_oprtr),
            }
        }
        fn qb(
            self,
            query: sqlx::query::Query<'lt, sqlx::Postgres, sqlx::postgres::PgArguments>,
        ) -> Result<sqlx::query::Query<'lt, sqlx::Postgres, sqlx::postgres::PgArguments>, String>
        {
            match self {
                Self::Id(v) => pg_crud::PgTypeWhFlt::qb(v, query),
                Self::Field0(v) => pg_crud::PgTypeWhFlt::qb(v, query),
                Self::Field1(v) => pg_crud::PgTypeWhFlt::qb(v, query),
                Self::Field2(v) => pg_crud::PgTypeWhFlt::qb(v, query),
                Self::Eq(v_45b5a7f0) => pg_crud::PgTypeWhFlt::qb(v_45b5a7f0, query),
            }
        }
    }
    impl loc_lib::ToErrString for ObjExampleWithIdAsNnJsonbObjWithIdWh {
        fn to_err_string(&self) -> String {
            format!("{self:?}")
        }
    }
    impl pg_crud::AllEnumVrtsArrDfltSomeOneEl for ObjExampleWithIdAsNnJsonbObjWithIdWh {
        fn all_vrts_dflt_some_one_el() -> Vec<Self> {
            vec![
                Self::Id(pg_crud::DfltSomeOneEl::dflt_some_one_el()),
                Self::Field0(pg_crud::DfltSomeOneEl::dflt_some_one_el()),
                Self::Field1(pg_crud::DfltSomeOneEl::dflt_some_one_el()),
                Self::Field2(pg_crud::DfltSomeOneEl::dflt_some_one_el()),
                Self::Eq(pg_crud::DfltSomeOneEl::dflt_some_one_el()),
            ]
        }
    }
    #[allow(clippy::arbitrary_source_item_ordering)]
    #[derive(
        Debug, Clone, PartialEq, serde :: Serialize, utoipa :: ToSchema, schemars :: JsonSchema,
    )]
    pub struct ObjExampleAsNnJsonbObjRd {
        #[serde(skip_serializing_if = "Option::is_none")]
        field_0: Option<pg_crud::V<<pg_crud::I8AsNnJsonbNbr as pg_crud::PgJson>::Rd>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        field_1: Option<pg_crud::V<<pg_crud::OptI8AsNlJsonbNbr as pg_crud::PgJson>::Rd>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        field_2: Option<pg_crud::V<<pg_crud::VecOfI8AsNnArrOfNnJsonbNbr as pg_crud::PgJson>::Rd>>,
    }
    #[derive(
        Debug, serde :: Serialize, serde :: Deserialize, thiserror :: Error, loc_lib :: Location,
    )]
    pub enum ObjExampleAsNnJsonbObjRdTryFromEr {
        AllFieldsAreNone { loc: loc_lib::loc::Loc },
    }
    impl ObjExampleAsNnJsonbObjRd {
        pub fn try_new(
            field_0: Option<pg_crud::V<<pg_crud::I8AsNnJsonbNbr as pg_crud::PgJson>::Rd>>,
            field_1: Option<pg_crud::V<<pg_crud::OptI8AsNlJsonbNbr as pg_crud::PgJson>::Rd>>,
            field_2: Option<
                pg_crud::V<<pg_crud::VecOfI8AsNnArrOfNnJsonbNbr as pg_crud::PgJson>::Rd>,
            >,
        ) -> Result<Self, ObjExampleAsNnJsonbObjRdTryFromEr> {
            if matches!((&field_0, &field_1, &field_2), (None, None, None)) {
                return Err(ObjExampleAsNnJsonbObjRdTryFromEr::AllFieldsAreNone {
                    loc: loc_lib::loc!(),
                });
            }
            Ok(Self {
                field_0,
                field_1,
                field_2,
            })
        }
    }
    #[derive(serde :: Deserialize)]
    #[allow(clippy::arbitrary_source_item_ordering)]
    struct ObjExampleAsNnJsonbObjRdRaw {
        field_0: Option<pg_crud::V<<pg_crud::I8AsNnJsonbNbr as pg_crud::PgJson>::Rd>>,
        field_1: Option<pg_crud::V<<pg_crud::OptI8AsNlJsonbNbr as pg_crud::PgJson>::Rd>>,
        field_2: Option<pg_crud::V<<pg_crud::VecOfI8AsNnArrOfNnJsonbNbr as pg_crud::PgJson>::Rd>>,
    }
    #[allow(unused_qualifications)]
    #[allow(clippy::absolute_paths)]
    #[allow(clippy::arbitrary_source_item_ordering)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for ObjExampleAsNnJsonbObjRd {
            fn deserialize<__D>(__deserializer: __D) -> Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                let raw = <ObjExampleAsNnJsonbObjRdRaw as _serde::Deserialize>::deserialize(
                    __deserializer,
                )?;
                Self::try_new(raw.field_0, raw.field_1, raw.field_2)
                    .map_err(|er| _serde::de::Error::custom(format!("{er:?}")))
            }
        }
    };
    impl pg_crud::DfltSomeOneEl for ObjExampleAsNnJsonbObjRd {
        fn dflt_some_one_el() -> Self {
            Self {
                field_0: Some(pg_crud::V {
                    v: pg_crud::DfltSomeOneEl::dflt_some_one_el(),
                }),
                field_1: Some(pg_crud::V {
                    v: pg_crud::DfltSomeOneEl::dflt_some_one_el(),
                }),
                field_2: Some(pg_crud::V {
                    v: pg_crud::DfltSomeOneEl::dflt_some_one_el(),
                }),
            }
        }
    }
    impl sqlx::Type<sqlx::Postgres> for ObjExampleAsNnJsonbObjRd {
        fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> bool {
            <sqlx::types::Json<Self> as sqlx::Type<sqlx::Postgres>>::compatible(ty)
        }
        fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
            <sqlx::types::Json<Self> as sqlx::Type<sqlx::Postgres>>::type_info()
        }
    }
    impl sqlx::Encode<'_, sqlx::Postgres> for ObjExampleAsNnJsonbObjRd {
        fn encode_by_ref(
            &self,
            buf: &mut sqlx::postgres::PgArgumentBuffer,
        ) -> Result<sqlx::encode::IsNull, Box<dyn std::error::Error + Send + Sync>> {
            sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&sqlx::types::Json(self), buf)
        }
    }
    impl sqlx::Decode<'_, sqlx::Postgres> for ObjExampleAsNnJsonbObjRd {
        fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
            match <sqlx::types::Json<Self> as sqlx::Decode<sqlx::Postgres>>::decode(value) {
                Ok(v) => Ok(v.0),
                Err(er) => Err(er),
            }
        }
    }
    #[allow(clippy::arbitrary_source_item_ordering)]
    #[derive(
        Debug, Clone, PartialEq, serde :: Serialize, utoipa :: ToSchema, schemars :: JsonSchema,
    )]
    pub struct ObjExampleWithIdAsNnJsonbObjWithIdRd {
        #[serde(skip_serializing_if = "Option::is_none")]
        id: Option<pg_crud::V<<pg_crud::UuidUuidAsNnJsonbString as pg_crud::PgJson>::Rd>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        field_0: Option<pg_crud::V<<pg_crud::I8AsNnJsonbNbr as pg_crud::PgJson>::Rd>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        field_1: Option<pg_crud::V<<pg_crud::OptI8AsNlJsonbNbr as pg_crud::PgJson>::Rd>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        field_2: Option<pg_crud::V<<pg_crud::VecOfI8AsNnArrOfNnJsonbNbr as pg_crud::PgJson>::Rd>>,
    }
    #[derive(
        Debug, serde :: Serialize, serde :: Deserialize, thiserror :: Error, loc_lib :: Location,
    )]
    pub enum ObjExampleWithIdAsNnJsonbObjWithIdRdTryFromEr {
        AllFieldsAreNone { loc: loc_lib::loc::Loc },
    }
    impl ObjExampleWithIdAsNnJsonbObjWithIdRd {
        pub fn try_new(
            id: Option<pg_crud::V<<pg_crud::UuidUuidAsNnJsonbString as pg_crud::PgJson>::Rd>>,
            field_0: Option<pg_crud::V<<pg_crud::I8AsNnJsonbNbr as pg_crud::PgJson>::Rd>>,
            field_1: Option<pg_crud::V<<pg_crud::OptI8AsNlJsonbNbr as pg_crud::PgJson>::Rd>>,
            field_2: Option<
                pg_crud::V<<pg_crud::VecOfI8AsNnArrOfNnJsonbNbr as pg_crud::PgJson>::Rd>,
            >,
        ) -> Result<Self, ObjExampleWithIdAsNnJsonbObjWithIdRdTryFromEr> {
            if matches!(
                (&id, &field_0, &field_1, &field_2),
                (None, None, None, None)
            ) {
                return Err(
                    ObjExampleWithIdAsNnJsonbObjWithIdRdTryFromEr::AllFieldsAreNone {
                        loc: loc_lib::loc!(),
                    },
                );
            }
            Ok(Self {
                id,
                field_0,
                field_1,
                field_2,
            })
        }
    }
    #[derive(serde :: Deserialize)]
    #[allow(clippy::arbitrary_source_item_ordering)]
    struct ObjExampleWithIdAsNnJsonbObjWithIdRdRaw {
        id: Option<pg_crud::V<<pg_crud::UuidUuidAsNnJsonbString as pg_crud::PgJson>::Rd>>,
        field_0: Option<pg_crud::V<<pg_crud::I8AsNnJsonbNbr as pg_crud::PgJson>::Rd>>,
        field_1: Option<pg_crud::V<<pg_crud::OptI8AsNlJsonbNbr as pg_crud::PgJson>::Rd>>,
        field_2: Option<pg_crud::V<<pg_crud::VecOfI8AsNnArrOfNnJsonbNbr as pg_crud::PgJson>::Rd>>,
    }
    #[allow(unused_qualifications)]
    #[allow(clippy::absolute_paths)]
    #[allow(clippy::arbitrary_source_item_ordering)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for ObjExampleWithIdAsNnJsonbObjWithIdRd {
            fn deserialize<__D>(__deserializer: __D) -> Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                let raw =
                    <ObjExampleWithIdAsNnJsonbObjWithIdRdRaw as _serde::Deserialize>::deserialize(
                        __deserializer,
                    )?;
                Self::try_new(raw.id, raw.field_0, raw.field_1, raw.field_2)
                    .map_err(|er| _serde::de::Error::custom(format!("{er:?}")))
            }
        }
    };
    impl pg_crud::DfltSomeOneEl for ObjExampleWithIdAsNnJsonbObjWithIdRd {
        fn dflt_some_one_el() -> Self {
            Self {
                id: Some(pg_crud::V {
                    v: pg_crud::DfltSomeOneEl::dflt_some_one_el(),
                }),
                field_0: Some(pg_crud::V {
                    v: pg_crud::DfltSomeOneEl::dflt_some_one_el(),
                }),
                field_1: Some(pg_crud::V {
                    v: pg_crud::DfltSomeOneEl::dflt_some_one_el(),
                }),
                field_2: Some(pg_crud::V {
                    v: pg_crud::DfltSomeOneEl::dflt_some_one_el(),
                }),
            }
        }
    }
    impl sqlx::Type<sqlx::Postgres> for ObjExampleWithIdAsNnJsonbObjWithIdRd {
        fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> bool {
            <sqlx::types::Json<Self> as sqlx::Type<sqlx::Postgres>>::compatible(ty)
        }
        fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
            <sqlx::types::Json<Self> as sqlx::Type<sqlx::Postgres>>::type_info()
        }
    }
    impl sqlx::Decode<'_, sqlx::Postgres> for ObjExampleWithIdAsNnJsonbObjWithIdRd {
        fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
            match <sqlx::types::Json<Self> as sqlx::Decode<sqlx::Postgres>>::decode(value) {
                Ok(v) => Ok(v.0),
                Err(er) => Err(er),
            }
        }
    }
    #[allow(clippy::arbitrary_source_item_ordering)]
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    pub struct ObjExampleAsNnJsonbObjRdIdsH {
        field_0: <pg_crud::I8AsNnJsonbNbr as pg_crud::PgJson>::RdIds,
        field_1: <pg_crud::OptI8AsNlJsonbNbr as pg_crud::PgJson>::RdIds,
        field_2: <pg_crud::VecOfI8AsNnArrOfNnJsonbNbr as pg_crud::PgJson>::RdIds,
    }
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    pub struct ObjExampleAsNnJsonbObjRdIds(pg_crud::V<ObjExampleAsNnJsonbObjRdIdsH>);
    impl sqlx::Decode<'_, sqlx::Postgres> for ObjExampleAsNnJsonbObjRdIds {
        fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
            match <sqlx::types::Json<Self> as sqlx::Decode<sqlx::Postgres>>::decode(value) {
                Ok(v) => Ok(v.0),
                Err(er) => Err(er),
            }
        }
    }
    impl sqlx::Type<sqlx::Postgres> for ObjExampleAsNnJsonbObjRdIds {
        fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> bool {
            <sqlx::types::Json<Self> as sqlx::Type<sqlx::Postgres>>::compatible(ty)
        }
        fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
            <sqlx::types::Json<Self> as sqlx::Type<sqlx::Postgres>>::type_info()
        }
    }
    #[allow(clippy::arbitrary_source_item_ordering)]
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    pub struct ObjExampleWithIdAsNnJsonbObjWithIdRdIdsH {
        id: <pg_crud::UuidUuidAsNnJsonbString as pg_crud::PgJson>::RdIds,
        field_0: <pg_crud::I8AsNnJsonbNbr as pg_crud::PgJson>::RdIds,
        field_1: <pg_crud::OptI8AsNlJsonbNbr as pg_crud::PgJson>::RdIds,
        field_2: <pg_crud::VecOfI8AsNnArrOfNnJsonbNbr as pg_crud::PgJson>::RdIds,
    }
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    pub struct ObjExampleWithIdAsNnJsonbObjWithIdRdIds(
        pub pg_crud::V<ObjExampleWithIdAsNnJsonbObjWithIdRdIdsH>,
    );
    impl sqlx::Decode<'_, sqlx::Postgres> for ObjExampleWithIdAsNnJsonbObjWithIdRdIds {
        fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
            match <sqlx::types::Json<Self> as sqlx::Decode<sqlx::Postgres>>::decode(value) {
                Ok(v) => Ok(v.0),
                Err(er) => Err(er),
            }
        }
    }
    impl sqlx::Type<sqlx::Postgres> for ObjExampleWithIdAsNnJsonbObjWithIdRdIds {
        fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> bool {
            <sqlx::types::Json<Self> as sqlx::Type<sqlx::Postgres>>::compatible(ty)
        }
        fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
            <sqlx::types::Json<Self> as sqlx::Type<sqlx::Postgres>>::type_info()
        }
    }
    #[allow(clippy::arbitrary_source_item_ordering)]
    #[derive(Debug, Clone, PartialEq)]
    pub struct ObjExampleAsNnJsonbObjRdInn {
        field_0: Option<pg_crud::V<<pg_crud::I8AsNnJsonbNbr as pg_crud::PgJson>::RdInn>>,
        field_1: Option<pg_crud::V<<pg_crud::OptI8AsNlJsonbNbr as pg_crud::PgJson>::RdInn>>,
        field_2:
            Option<pg_crud::V<<pg_crud::VecOfI8AsNnArrOfNnJsonbNbr as pg_crud::PgJson>::RdInn>>,
    }
    #[allow(clippy::arbitrary_source_item_ordering)]
    #[derive(Debug, Clone, PartialEq)]
    pub struct ObjExampleWithIdAsNnJsonbObjWithIdRdInn {
        id: Option<pg_crud::V<<pg_crud::UuidUuidAsNnJsonbString as pg_crud::PgJson>::RdInn>>,
        field_0: Option<pg_crud::V<<pg_crud::I8AsNnJsonbNbr as pg_crud::PgJson>::RdInn>>,
        field_1: Option<pg_crud::V<<pg_crud::OptI8AsNlJsonbNbr as pg_crud::PgJson>::RdInn>>,
        field_2:
            Option<pg_crud::V<<pg_crud::VecOfI8AsNnArrOfNnJsonbNbr as pg_crud::PgJson>::RdInn>>,
    }
    #[allow(clippy::arbitrary_source_item_ordering)]
    #[derive(
        Debug,
        Clone,
        PartialEq,
        serde :: Serialize,
        serde :: Deserialize,
        utoipa :: ToSchema,
        schemars :: JsonSchema,
    )]
    pub struct ObjExampleAsNnJsonbObjUpd(pg_crud::NotEmptyUnqVec<ObjExampleAsNnJsonbObjUpdEl>);
    impl ObjExampleAsNnJsonbObjUpd {
        #[must_use]
        pub const fn new(v: pg_crud::NotEmptyUnqVec<ObjExampleAsNnJsonbObjUpdEl>) -> Self {
            Self(v)
        }
    }
    impl pg_crud::DfltSomeOneEl for ObjExampleAsNnJsonbObjUpd {
        fn dflt_some_one_el() -> Self {
            Self(pg_crud::DfltSomeOneEl::dflt_some_one_el())
        }
    }
    #[allow(clippy::arbitrary_source_item_ordering)]
    #[derive(
        Debug,
        Clone,
        PartialEq,
        serde :: Serialize,
        serde :: Deserialize,
        utoipa :: ToSchema,
        schemars :: JsonSchema,
    )]
    pub enum ObjExampleAsNnJsonbObjUpdEl {
        #[serde(rename(serialize = "field_0", deserialize = "field_0"))]
        Field0(pg_crud::V<<pg_crud::I8AsNnJsonbNbr as pg_crud::PgJson>::Upd>),
        #[serde(rename(serialize = "field_1", deserialize = "field_1"))]
        Field1(pg_crud::V<<pg_crud::OptI8AsNlJsonbNbr as pg_crud::PgJson>::Upd>),
        #[serde(rename(serialize = "field_2", deserialize = "field_2"))]
        Field2(pg_crud::V<<pg_crud::VecOfI8AsNnArrOfNnJsonbNbr as pg_crud::PgJson>::Upd>),
    }
    impl pg_crud::AllEnumVrtsArrDfltSomeOneEl for ObjExampleAsNnJsonbObjUpdEl {
        fn all_vrts_dflt_some_one_el() -> Vec<Self> {
            vec![
                Self::Field0(pg_crud::V {
                    v: pg_crud::DfltSomeOneEl::dflt_some_one_el(),
                }),
                Self::Field1(pg_crud::V {
                    v: pg_crud::DfltSomeOneEl::dflt_some_one_el(),
                }),
                Self::Field2(pg_crud::V {
                    v: pg_crud::DfltSomeOneEl::dflt_some_one_el(),
                }),
            ]
        }
    }
    #[allow(clippy::arbitrary_source_item_ordering)]
    #[derive(
        Debug,
        Clone,
        PartialEq,
        serde :: Serialize,
        serde :: Deserialize,
        utoipa :: ToSchema,
        schemars :: JsonSchema,
    )]
    pub struct ObjExampleWithIdAsNnJsonbObjWithIdUpdEl {
        id: <pg_crud::UuidUuidAsNnJsonbString as pg_crud::PgJson>::Upd,
        fields: <ObjExampleAsNnJsonbObj as pg_crud::PgJson>::Upd,
    }
    impl ObjExampleWithIdAsNnJsonbObjWithIdUpdEl {
        #[must_use]
        pub const fn new(
            id: <pg_crud::UuidUuidAsNnJsonbString as pg_crud::PgJson>::Upd,
            fields: <ObjExampleAsNnJsonbObj as pg_crud::PgJson>::Upd,
        ) -> Self {
            Self { id, fields }
        }
    }
    impl pg_crud::DfltSomeOneEl for ObjExampleWithIdAsNnJsonbObjWithIdUpdEl {
        fn dflt_some_one_el() -> Self {
            Self {
                id: pg_crud::DfltSomeOneEl::dflt_some_one_el(),
                fields: pg_crud::DfltSomeOneEl::dflt_some_one_el(),
            }
        }
    }
    #[allow(clippy::arbitrary_source_item_ordering)]
    #[derive(Debug, Clone, PartialEq, serde :: Serialize)]
    pub struct ObjExampleAsNnJsonbObjUpdForQuery(
        pg_crud::NotEmptyUnqVec<ObjExampleAsNnJsonbObjUpdForQueryEl>,
    );
    impl ObjExampleAsNnJsonbObjUpdForQuery {
        #[allow(clippy::single_call_fn)]
        fn sel_only_updd_ids_qp(
            &self,
            col_field: &str,
            incr: &mut u64,
        ) -> Result<String, pg_crud::QpEr> {
            let mut acc_8e628eaf = String::default();
            for el in self.0.to_vec() {
                match & el { ObjExampleAsNnJsonbObjUpdForQueryEl :: Field0 (v_939e13d6) => { match < pg_crud :: I8AsNnJsonbNbr as pg_crud :: PgJson > :: sel_only_updd_ids_qp (& v_939e13d6 . v , "field_0" , col_field , incr) { Ok (mut v_c3ae3be4) => { let _ : Option < char > = v_c3ae3be4 . pop () ; if { use std :: fmt :: Write as _ ; write ! (acc_8e628eaf , "jsonb_build_object({v_c3ae3be4})||") } . is_err () { return Err (pg_crud :: QpEr :: WriteIntoBuffer { loc : loc_lib :: loc ! () }) ; } } , Err (er) => { return Err (er) ; } } } , ObjExampleAsNnJsonbObjUpdForQueryEl :: Field1 (v_939e13d6) => { match < pg_crud :: OptI8AsNlJsonbNbr as pg_crud :: PgJson > :: sel_only_updd_ids_qp (& v_939e13d6 . v , "field_1" , col_field , incr) { Ok (mut v_c3ae3be4) => { let _ : Option < char > = v_c3ae3be4 . pop () ; if { use std :: fmt :: Write as _ ; write ! (acc_8e628eaf , "jsonb_build_object({v_c3ae3be4})||") } . is_err () { return Err (pg_crud :: QpEr :: WriteIntoBuffer { loc : loc_lib :: loc ! () }) ; } } , Err (er) => { return Err (er) ; } } } , ObjExampleAsNnJsonbObjUpdForQueryEl :: Field2 (v_939e13d6) => { match < pg_crud :: VecOfI8AsNnArrOfNnJsonbNbr as pg_crud :: PgJson > :: sel_only_updd_ids_qp (& v_939e13d6 . v , "field_2" , col_field , incr) { Ok (mut v_c3ae3be4) => { let _ : Option < char > = v_c3ae3be4 . pop () ; if { use std :: fmt :: Write as _ ; write ! (acc_8e628eaf , "jsonb_build_object({v_c3ae3be4})||") } . is_err () { return Err (pg_crud :: QpEr :: WriteIntoBuffer { loc : loc_lib :: loc ! () }) ; } } , Err (er) => { return Err (er) ; } } } }
            }
            let _: Option<char> = acc_8e628eaf.pop();
            let _: Option<char> = acc_8e628eaf.pop();
            Ok(acc_8e628eaf)
        }
    }
    impl From<<ObjExampleAsNnJsonbObj as pg_crud::PgJson>::Upd>
        for <ObjExampleAsNnJsonbObj as pg_crud::PgJson>::UpdForQuery
    {
        fn from(v: <ObjExampleAsNnJsonbObj as pg_crud::PgJson>::Upd) -> Self {
            Self(pg_crud::NotEmptyUnqVec::from_t1_impl_from_t2(v.0))
        }
    }
    #[derive(Debug, Clone, PartialEq, serde :: Serialize)]
    pub enum ObjExampleAsNnJsonbObjUpdForQueryEl {
        #[serde(rename(serialize = "field_0", deserialize = "field_0"))]
        Field0(pg_crud::V<<pg_crud::I8AsNnJsonbNbr as pg_crud::PgJson>::UpdForQuery>),
        #[serde(rename(serialize = "field_1", deserialize = "field_1"))]
        Field1(pg_crud::V<<pg_crud::OptI8AsNlJsonbNbr as pg_crud::PgJson>::UpdForQuery>),
        #[serde(rename(serialize = "field_2", deserialize = "field_2"))]
        Field2(pg_crud::V<<pg_crud::VecOfI8AsNnArrOfNnJsonbNbr as pg_crud::PgJson>::UpdForQuery>),
    }
    impl From<ObjExampleAsNnJsonbObjUpdEl> for ObjExampleAsNnJsonbObjUpdForQueryEl {
        fn from(v: ObjExampleAsNnJsonbObjUpdEl) -> Self {
            match v {
                ObjExampleAsNnJsonbObjUpdEl::Field0(v_121f1c54) => Self::Field0(pg_crud::V {
                    v: <pg_crud::I8AsNnJsonbNbr as pg_crud::PgJson>::UpdForQuery::from(
                        v_121f1c54.v,
                    ),
                }),
                ObjExampleAsNnJsonbObjUpdEl::Field1(v_121f1c54) => Self::Field1(pg_crud::V {
                    v: <pg_crud::OptI8AsNlJsonbNbr as pg_crud::PgJson>::UpdForQuery::from(
                        v_121f1c54.v,
                    ),
                }),
                ObjExampleAsNnJsonbObjUpdEl::Field2(v_121f1c54) => Self::Field2(pg_crud::V {
                    v: <pg_crud::VecOfI8AsNnArrOfNnJsonbNbr as pg_crud::PgJson>::UpdForQuery::from(
                        v_121f1c54.v,
                    ),
                }),
            }
        }
    }
    #[allow(clippy::arbitrary_source_item_ordering)]
    #[derive(Debug, Clone, PartialEq, serde :: Serialize)]
    pub struct ObjExampleWithIdAsNnJsonbObjWithIdUpdForQueryEl {
        id: <pg_crud::UuidUuidAsNnJsonbString as pg_crud::PgJson>::UpdForQuery,
        fields: <ObjExampleAsNnJsonbObj as pg_crud::PgJson>::UpdForQuery,
    }
    impl ObjExampleWithIdAsNnJsonbObjWithIdUpdForQueryEl {
        #[must_use]
        pub const fn new(
            id: <pg_crud::UuidUuidAsNnJsonbString as pg_crud::PgJson>::UpdForQuery,
            fields: <ObjExampleAsNnJsonbObj as pg_crud::PgJson>::UpdForQuery,
        ) -> Self {
            Self { id, fields }
        }
    }
    impl From<ObjExampleWithIdAsNnJsonbObjWithIdUpdEl>
        for ObjExampleWithIdAsNnJsonbObjWithIdUpdForQueryEl
    {
        fn from(v: ObjExampleWithIdAsNnJsonbObjWithIdUpdEl) -> Self {
            Self {
                id: <pg_crud::UuidUuidAsNnJsonbString as pg_crud::PgJson>::UpdForQuery::from(v.id),
                fields: <ObjExampleAsNnJsonbObj as pg_crud::PgJson>::UpdForQuery::from(v.fields),
            }
        }
    }
    #[allow(clippy::arbitrary_source_item_ordering)]
    impl pg_crud::PgJson for ObjExampleAsNnJsonbObj {
        type Tt = ObjExampleAsNnJsonbObjTt;
        type Cr = ObjExampleAsNnJsonbObjCr;
        type CrForQuery = ObjExampleAsNnJsonbObjCrForQuery;
        type Sel = ObjExampleAsNnJsonbObjSel;
        fn sel_qp(
            v: &Self::Sel,
            fi: &str,
            col_field: &str,
            col_field_for_er_msg: &str,
            is_pg_type: bool,
        ) -> Result<String, pg_crud::QpEr> {
            match v.sel_qp(
                &if is_pg_type {
                    col_field.to_owned()
                } else {
                    format!("{col_field}->'{fi}'")
                },
                &format!("{col_field_for_er_msg}.{fi}"),
            ) {
                Ok(v_156121ad) => Ok(if is_pg_type {
                    v_156121ad
                } else {
                    format!("jsonb_build_object('{fi}',jsonb_build_object('v',{v_156121ad}))")
                }),
                Err(er) => Err(er),
            }
        }
        type Wh = ObjExampleAsNnJsonbObjWh;
        type Rd = ObjExampleAsNnJsonbObjRd;
        type RdIds = ObjExampleAsNnJsonbObjRdIds;
        fn sel_only_ids_qp(col_field: &str) -> Result<String, pg_crud::QpEr> {
            Ok({
                let mut acc = String::default();
                if {
                    use std::fmt::Write as _;
                    write!(
                        acc,
                        "jsonb_build_object('field_0',{})||",
                        match <pg_crud::I8AsNnJsonbNbr as pg_crud::PgJson>::sel_only_ids_qp(
                            &format!("{col_field}->'field_0'")
                        ) {
                            Ok(v_2317e0af) => v_2317e0af,
                            Err(er) => {
                                return Err(er);
                            }
                        }
                    )
                }
                .is_err()
                {
                    return Err(pg_crud::QpEr::WriteIntoBuffer {
                        loc: loc_lib::loc!(),
                    });
                }
                if {
                    use std::fmt::Write as _;
                    write!(
                        acc,
                        "jsonb_build_object('field_1',{})||",
                        match <pg_crud::OptI8AsNlJsonbNbr as pg_crud::PgJson>::sel_only_ids_qp(
                            &format!("{col_field}->'field_1'")
                        ) {
                            Ok(v_2317e0af) => v_2317e0af,
                            Err(er) => {
                                return Err(er);
                            }
                        }
                    )
                }
                .is_err()
                {
                    return Err(pg_crud::QpEr::WriteIntoBuffer {
                        loc: loc_lib::loc!(),
                    });
                }
                if { use std :: fmt :: Write as _ ; write ! (acc , "jsonb_build_object('field_2',{})||" , match < pg_crud :: VecOfI8AsNnArrOfNnJsonbNbr as pg_crud :: PgJson > :: sel_only_ids_qp (& format ! ("{col_field}->'field_2'")) { Ok (v_2317e0af) => v_2317e0af , Err (er) => { return Err (er) ; } }) } . is_err () { return Err (pg_crud :: QpEr :: WriteIntoBuffer { loc : loc_lib :: loc ! () }) ; }
                let _: Option<char> = acc.pop();
                let _: Option<char> = acc.pop();
                format!("jsonb_build_object('v',{acc})")
            })
        }
        type RdInn = ObjExampleAsNnJsonbObjRdInn;
        fn into_inn(v: Self::Rd) -> Self::RdInn {
            ObjExampleAsNnJsonbObjRdInn {
                field_0: v.field_0.map(|v_6e5af985| pg_crud::V {
                    v: <pg_crud::I8AsNnJsonbNbr as pg_crud::PgJson>::into_inn(v_6e5af985.v),
                }),
                field_1: v.field_1.map(|v_6e5af985| pg_crud::V {
                    v: <pg_crud::OptI8AsNlJsonbNbr as pg_crud::PgJson>::into_inn(v_6e5af985.v),
                }),
                field_2: v.field_2.map(|v_6e5af985| pg_crud::V {
                    v: <pg_crud::VecOfI8AsNnArrOfNnJsonbNbr as pg_crud::PgJson>::into_inn(
                        v_6e5af985.v,
                    ),
                }),
            }
        }
        type Upd = ObjExampleAsNnJsonbObjUpd;
        type UpdForQuery = ObjExampleAsNnJsonbObjUpdForQuery;
        fn upd_qp(
            v: &Self::UpdForQuery,
            jsonb_set_accumulator: &str,
            jsonb_set_target: &str,
            jsonb_set_path: &str,
            incr: &mut u64,
        ) -> Result<String, pg_crud::QpEr> {
            let _: &str = jsonb_set_accumulator;
            let _: &str = jsonb_set_path;
            let mut std_opt_opt_obj_acc = format!(
                "case when jsonb_typeof({jsonb_set_target}) = 'object' then ({jsonb_set_target})::jsonb else '{{}}'::jsonb end"
            );
            let gen_jsonb_set_target =
                |v_12d082b5: &str| format!("{jsonb_set_target}->'{v_12d082b5}'");
            for el in v.0.to_vec() {
                match el {
                    ObjExampleAsNnJsonbObjUpdForQueryEl::Field0(v_3b3fae4c) => {
                        match <pg_crud::I8AsNnJsonbNbr as pg_crud::PgJson>::upd_qp(
                            &v_3b3fae4c.v,
                            &std_opt_opt_obj_acc,
                            &gen_jsonb_set_target("field_0"),
                            "field_0",
                            incr,
                        ) {
                            Ok(v_5edc1648) => {
                                std_opt_opt_obj_acc = v_5edc1648;
                            }
                            Err(er) => {
                                return Err(er);
                            }
                        }
                    }
                    ObjExampleAsNnJsonbObjUpdForQueryEl::Field1(v_3b3fae4c) => {
                        match <pg_crud::OptI8AsNlJsonbNbr as pg_crud::PgJson>::upd_qp(
                            &v_3b3fae4c.v,
                            &std_opt_opt_obj_acc,
                            &gen_jsonb_set_target("field_1"),
                            "field_1",
                            incr,
                        ) {
                            Ok(v_5edc1648) => {
                                std_opt_opt_obj_acc = v_5edc1648;
                            }
                            Err(er) => {
                                return Err(er);
                            }
                        }
                    }
                    ObjExampleAsNnJsonbObjUpdForQueryEl::Field2(v_3b3fae4c) => {
                        match <pg_crud::VecOfI8AsNnArrOfNnJsonbNbr as pg_crud::PgJson>::upd_qp(
                            &v_3b3fae4c.v,
                            &std_opt_opt_obj_acc,
                            &gen_jsonb_set_target("field_2"),
                            "field_2",
                            incr,
                        ) {
                            Ok(v_5edc1648) => {
                                std_opt_opt_obj_acc = v_5edc1648;
                            }
                            Err(er) => {
                                return Err(er);
                            }
                        }
                    }
                }
            }
            if jsonb_set_path.is_empty() {
                Ok(std_opt_opt_obj_acc)
            } else {
                Ok(format!(
                    "jsonb_set({jsonb_set_accumulator},'{{{jsonb_set_path}}}',{std_opt_opt_obj_acc})"
                ))
            }
        }
        fn upd_qb(
            v: Self::UpdForQuery,
            mut query: sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>,
        ) -> Result<sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>, String>
        {
            for el in v.0.into_vec() {
                match el {
                    ObjExampleAsNnJsonbObjUpdForQueryEl::Field0(v_b27f5b09) => {
                        match <pg_crud::I8AsNnJsonbNbr as pg_crud::PgJson>::upd_qb(
                            v_b27f5b09.v,
                            query,
                        ) {
                            Ok(v_a4870bad) => {
                                query = v_a4870bad;
                            }
                            Err(er) => {
                                return Err(er);
                            }
                        }
                    }
                    ObjExampleAsNnJsonbObjUpdForQueryEl::Field1(v_b27f5b09) => {
                        match <pg_crud::OptI8AsNlJsonbNbr as pg_crud::PgJson>::upd_qb(
                            v_b27f5b09.v,
                            query,
                        ) {
                            Ok(v_a4870bad) => {
                                query = v_a4870bad;
                            }
                            Err(er) => {
                                return Err(er);
                            }
                        }
                    }
                    ObjExampleAsNnJsonbObjUpdForQueryEl::Field2(v_b27f5b09) => {
                        match <pg_crud::VecOfI8AsNnArrOfNnJsonbNbr as pg_crud::PgJson>::upd_qb(
                            v_b27f5b09.v,
                            query,
                        ) {
                            Ok(v_a4870bad) => {
                                query = v_a4870bad;
                            }
                            Err(er) => {
                                return Err(er);
                            }
                        }
                    }
                }
            }
            Ok(query)
        }
        fn sel_only_updd_ids_qp(
            v: &Self::UpdForQuery,
            fi: &str,
            col_field: &str,
            incr: &mut u64,
        ) -> Result<String, pg_crud::QpEr> {
            match v.sel_only_updd_ids_qp(&format!("{col_field}->'{fi}'"), incr) {
                Ok(v_e137951b) => Ok(format!("'{fi}',jsonb_build_object('v',{v_e137951b}),")),
                Err(er) => Err(er),
            }
        }
        fn sel_only_updd_ids_qb<'lt>(
            v: &'lt Self::UpdForQuery,
            mut query: sqlx::query::Query<'lt, sqlx::Postgres, sqlx::postgres::PgArguments>,
        ) -> Result<sqlx::query::Query<'lt, sqlx::Postgres, sqlx::postgres::PgArguments>, String>
        {
            for el in v.0.to_vec() {
                match el { ObjExampleAsNnJsonbObjUpdForQueryEl :: Field0 (v_b79c2851) => { match < pg_crud :: I8AsNnJsonbNbr as pg_crud :: PgJson > :: sel_only_updd_ids_qb (& v_b79c2851 . v , query) { Ok (v_e8914f75) => { query = v_e8914f75 ; } Err (er) => { return Err (er) ; } } } , ObjExampleAsNnJsonbObjUpdForQueryEl :: Field1 (v_b79c2851) => { match < pg_crud :: OptI8AsNlJsonbNbr as pg_crud :: PgJson > :: sel_only_updd_ids_qb (& v_b79c2851 . v , query) { Ok (v_e8914f75) => { query = v_e8914f75 ; } Err (er) => { return Err (er) ; } } } , ObjExampleAsNnJsonbObjUpdForQueryEl :: Field2 (v_b79c2851) => { match < pg_crud :: VecOfI8AsNnArrOfNnJsonbNbr as pg_crud :: PgJson > :: sel_only_updd_ids_qb (& v_b79c2851 . v , query) { Ok (v_e8914f75) => { query = v_e8914f75 ; } Err (er) => { return Err (er) ; } } } }
            }
            Ok(query)
        }
        fn sel_only_crd_ids_qp(
            v: &Self::CrForQuery,
            fi: &str,
            col_field: &str,
            incr: &mut u64,
        ) -> Result<String, pg_crud::QpEr> {
            Ok(format!("'{fi}',jsonb_build_object('v',{}),", {
                let mut acc_0fe559fa = String::new();
                match <pg_crud::I8AsNnJsonbNbr as pg_crud::PgJson>::sel_only_crd_ids_qp(
                    &v.field_0,
                    "field_0",
                    &format!("{col_field}->'field_0'"),
                    incr,
                ) {
                    Ok(mut v_cddc8a0a) => {
                        let _: Option<char> = v_cddc8a0a.pop();
                        if {
                            use std::fmt::Write as _;
                            write!(acc_0fe559fa, "jsonb_build_object({v_cddc8a0a})||")
                        }
                        .is_err()
                        {
                            return Err(pg_crud::QpEr::WriteIntoBuffer {
                                loc: loc_lib::loc!(),
                            });
                        }
                    }
                    Err(er) => {
                        return Err(er);
                    }
                }
                match <pg_crud::OptI8AsNlJsonbNbr as pg_crud::PgJson>::sel_only_crd_ids_qp(
                    &v.field_1,
                    "field_1",
                    &format!("{col_field}->'field_1'"),
                    incr,
                ) {
                    Ok(mut v_cddc8a0a) => {
                        let _: Option<char> = v_cddc8a0a.pop();
                        if {
                            use std::fmt::Write as _;
                            write!(acc_0fe559fa, "jsonb_build_object({v_cddc8a0a})||")
                        }
                        .is_err()
                        {
                            return Err(pg_crud::QpEr::WriteIntoBuffer {
                                loc: loc_lib::loc!(),
                            });
                        }
                    }
                    Err(er) => {
                        return Err(er);
                    }
                }
                match <pg_crud::VecOfI8AsNnArrOfNnJsonbNbr as pg_crud::PgJson>::sel_only_crd_ids_qp(
                    &v.field_2,
                    "field_2",
                    &format!("{col_field}->'field_2'"),
                    incr,
                ) {
                    Ok(mut v_cddc8a0a) => {
                        let _: Option<char> = v_cddc8a0a.pop();
                        if {
                            use std::fmt::Write as _;
                            write!(acc_0fe559fa, "jsonb_build_object({v_cddc8a0a})||")
                        }
                        .is_err()
                        {
                            return Err(pg_crud::QpEr::WriteIntoBuffer {
                                loc: loc_lib::loc!(),
                            });
                        }
                    }
                    Err(er) => {
                        return Err(er);
                    }
                }
                let _: Option<char> = acc_0fe559fa.pop();
                let _: Option<char> = acc_0fe559fa.pop();
                acc_0fe559fa
            }))
        }
        fn sel_only_crd_ids_qb<'lt>(
            v: &'lt Self::CrForQuery,
            mut query: sqlx::query::Query<'lt, sqlx::Postgres, sqlx::postgres::PgArguments>,
        ) -> Result<sqlx::query::Query<'lt, sqlx::Postgres, sqlx::postgres::PgArguments>, String>
        {
            match <pg_crud::I8AsNnJsonbNbr as pg_crud::PgJson>::sel_only_crd_ids_qb(
                &v.field_0, query,
            ) {
                Ok(v_231618d9) => {
                    query = v_231618d9;
                }
                Err(er) => {
                    return Err(er);
                }
            }
            match <pg_crud::OptI8AsNlJsonbNbr as pg_crud::PgJson>::sel_only_crd_ids_qb(
                &v.field_1, query,
            ) {
                Ok(v_231618d9) => {
                    query = v_231618d9;
                }
                Err(er) => {
                    return Err(er);
                }
            }
            match <pg_crud::VecOfI8AsNnArrOfNnJsonbNbr as pg_crud::PgJson>::sel_only_crd_ids_qb(
                &v.field_2, query,
            ) {
                Ok(v_231618d9) => {
                    query = v_231618d9;
                }
                Err(er) => {
                    return Err(er);
                }
            }
            Ok(query)
        }
    }
    #[allow(clippy::arbitrary_source_item_ordering)]
    impl pg_crud::PgType for ObjExampleAsNnJsonbObj {
        type Tt = ObjExampleAsNnJsonbObjTt;
        fn cr_tbl_col_qp(col: &dyn std::fmt::Display, _: bool) -> impl std::fmt::Display {
            format!(
                "{col} jsonb not null check (jsonb_matches_schema('{}', {col}))",
                serde_json::to_string(&schemars::schema_for!(ObjExampleAsNnJsonbObjTt))
                    .expect("59a1654b")
            )
        }
        type Cr = ObjExampleAsNnJsonbObjCr;
        fn cr_qp(_: &Self::Cr, incr: &mut u64) -> Result<String, pg_crud::QpEr> {
            match pg_crud::incr_checked_add_one_returning_incr(incr) {
                Ok(v_7df9eb00) => Ok(format!("${v_7df9eb00}")),
                Err(er) => Err(er),
            }
        }
        fn cr_qb(
            v: Self::Cr,
            mut query: sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>,
        ) -> Result<sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>, String>
        {
            if let Err(er) = query.try_bind(<Self as pg_crud::PgJson>::CrForQuery::from(v)) {
                return Err(er.to_string());
            }
            Ok(query)
        }
        type Sel = ObjExampleAsNnJsonbObjSel;
        fn sel_qp(v: &Self::Sel, col: &str) -> Result<String, pg_crud::QpEr> {
            match v.sel_qp_pg_type(col) {
                Ok(v_d91c19a6) => Ok(format!("{v_d91c19a6} as {col}")),
                Err(er) => Err(er),
            }
        }
        type Wh = ObjExampleAsNnJsonbObjWh;
        type Rd = ObjExampleAsNnJsonbObjRd;
        fn normalize(v: Self::Rd) -> Self::Rd {
            v
        }
        type RdIds = ObjExampleAsNnJsonbObjRdIds;
        fn sel_only_ids_qp(col: &str) -> Result<String, pg_crud::QpEr> {
            match <Self as pg_crud::PgJson>::sel_only_ids_qp(col) {
                Ok(v_e776e9fa) => Ok(format!("{v_e776e9fa} as {col},")),
                Err(er) => Err(er),
            }
        }
        type RdInn = ObjExampleAsNnJsonbObjRdInn;
        fn into_inn(v: Self::Rd) -> Self::RdInn {
            <Self as pg_crud::PgJson>::into_inn(v)
        }
        type Upd = ObjExampleAsNnJsonbObjUpd;
        type UpdForQuery = ObjExampleAsNnJsonbObjUpdForQuery;
        #[allow(unused_variables)]
        fn upd_qp(
            v: &Self::UpdForQuery,
            jsonb_set_accumulator: &str,
            jsonb_set_target: &str,
            jsonb_set_path: &str,
            incr: &mut u64,
        ) -> Result<String, pg_crud::QpEr> {
            <Self as pg_crud::PgJson>::upd_qp(
                v,
                jsonb_set_accumulator,
                jsonb_set_target,
                jsonb_set_path,
                incr,
            )
        }
        fn upd_qb(
            v: Self::UpdForQuery,
            query: sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>,
        ) -> Result<sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>, String>
        {
            <Self as pg_crud::PgJson>::upd_qb(v, query)
        }
        fn sel_only_updd_ids_qp(
            v: &Self::UpdForQuery,
            col: &str,
            incr: &mut u64,
        ) -> Result<String, pg_crud::QpEr> {
            match v.sel_only_updd_ids_qp(col, incr) {
                Ok(v_f0787243) => Ok(format!("jsonb_build_object('v',{v_f0787243}) as {col},")),
                Err(er) => Err(er),
            }
        }
        fn sel_only_updd_ids_qb<'lt>(
            v: &'lt Self::UpdForQuery,
            query: sqlx::query::Query<'lt, sqlx::Postgres, sqlx::postgres::PgArguments>,
        ) -> Result<sqlx::query::Query<'lt, sqlx::Postgres, sqlx::postgres::PgArguments>, String>
        {
            <Self as pg_crud::PgJson>::sel_only_updd_ids_qb(v, query)
        }
    }
    #[allow(unused_qualifications)]
    #[allow(clippy::absolute_paths)]
    #[allow(clippy::arbitrary_source_item_ordering)]
    #[cfg(feature = "test-utils")]
    #[allow(clippy::float_arithmetic)]
    impl pg_crud::PgJsonTestCases for ObjExampleAsNnJsonbObj {
        type PgJson = Self;
        type Sel = ObjExampleAsNnJsonbObjSel;
        fn opt_vec_cr() -> Option<Vec<<Self::PgJson as pg_crud::PgJson>::Cr>> {
            Some({
                let mut acc_ccd79a32 = Vec::new();
                if let Some(v_0296b347) =
                    <pg_crud::I8AsNnJsonbNbr as pg_crud::PgJsonTestCases>::opt_vec_cr()
                {
                    for el_37154498 in v_0296b347 {
                        let v = <Self as pg_crud::PgJson>::Cr::new(
                            el_37154498,
                            pg_crud::DfltSomeOneEl::dflt_some_one_el(),
                            pg_crud::DfltSomeOneEl::dflt_some_one_el(),
                        );
                        if !acc_ccd79a32.contains(&v) {
                            acc_ccd79a32.push(v);
                        }
                    }
                }
                if let Some(v_0296b347) =
                    <pg_crud::OptI8AsNlJsonbNbr as pg_crud::PgJsonTestCases>::opt_vec_cr()
                {
                    for el_37154498 in v_0296b347 {
                        let v = <Self as pg_crud::PgJson>::Cr::new(
                            pg_crud::DfltSomeOneEl::dflt_some_one_el(),
                            el_37154498,
                            pg_crud::DfltSomeOneEl::dflt_some_one_el(),
                        );
                        if !acc_ccd79a32.contains(&v) {
                            acc_ccd79a32.push(v);
                        }
                    }
                }
                if let Some(v_0296b347) =
                    <pg_crud::VecOfI8AsNnArrOfNnJsonbNbr as pg_crud::PgJsonTestCases>::opt_vec_cr()
                {
                    for el_37154498 in v_0296b347 {
                        let v = <Self as pg_crud::PgJson>::Cr::new(
                            pg_crud::DfltSomeOneEl::dflt_some_one_el(),
                            pg_crud::DfltSomeOneEl::dflt_some_one_el(),
                            el_37154498,
                        );
                        if !acc_ccd79a32.contains(&v) {
                            acc_ccd79a32.push(v);
                        }
                    }
                }
                acc_ccd79a32
            })
        }
        fn rd_ids_to_2_dims_vec_rd_inn(
            rd_ids: &<Self::PgJson as pg_crud::PgJson>::RdIds,
        ) -> Vec<Vec<<Self::PgJson as pg_crud::PgJson>::RdInn>> {
            let mut acc_ef081dc3 = Vec::new();
            let mut field_0_last =
                <pg_crud::I8AsNnJsonbNbr as pg_crud::PgJsonTestCases>::rd_ids_into_opt_v_rd_inn(
                    rd_ids.0.v.field_0.clone(),
                );
            let mut field_1_last =
                <pg_crud::OptI8AsNlJsonbNbr as pg_crud::PgJsonTestCases>::rd_ids_into_opt_v_rd_inn(
                    rd_ids.0.v.field_1.clone(),
                );
            let mut field_2_last = < pg_crud :: VecOfI8AsNnArrOfNnJsonbNbr as pg_crud :: PgJsonTestCases > :: rd_ids_into_opt_v_rd_inn (rd_ids . 0. v . field_2 . clone ()) ;
            for el_7bf83754 in
                <pg_crud::I8AsNnJsonbNbr as pg_crud::PgJsonTestCases>::rd_ids_to_2_dims_vec_rd_inn(
                    &rd_ids.0.v.field_0,
                )
            {
                for el_2720df8a in el_7bf83754 {
                    let field_0_crnt = Some(pg_crud::V { v: el_2720df8a });
                    field_0_last.clone_from(&field_0_crnt);
                    acc_ef081dc3.push(vec![ObjExampleAsNnJsonbObjRdInn {
                        field_0: field_0_crnt.clone(),
                        field_1: field_1_last.clone(),
                        field_2: field_2_last.clone(),
                    }]);
                }
            }
            for el_7bf83754 in < pg_crud :: OptI8AsNlJsonbNbr as pg_crud :: PgJsonTestCases > :: rd_ids_to_2_dims_vec_rd_inn (& rd_ids . 0. v . field_1) { for el_2720df8a in el_7bf83754 { let field_1_crnt = Some (pg_crud :: V { v : el_2720df8a }) ; field_1_last . clone_from (& field_1_crnt) ; acc_ef081dc3 . push (vec ! [ObjExampleAsNnJsonbObjRdInn { field_0 : field_0_last . clone () , field_1 : field_1_crnt . clone () , field_2 : field_2_last . clone () }]) ; } }
            for el_7bf83754 in < pg_crud :: VecOfI8AsNnArrOfNnJsonbNbr as pg_crud :: PgJsonTestCases > :: rd_ids_to_2_dims_vec_rd_inn (& rd_ids . 0. v . field_2) { for el_2720df8a in el_7bf83754 { let field_2_crnt = Some (pg_crud :: V { v : el_2720df8a }) ; field_2_last . clone_from (& field_2_crnt) ; acc_ef081dc3 . push (vec ! [ObjExampleAsNnJsonbObjRdInn { field_0 : field_0_last . clone () , field_1 : field_1_last . clone () , field_2 : field_2_crnt . clone () }]) ; } }
            acc_ef081dc3
        }
        fn rd_inn_into_rd_with_new_or_try_new_unwraped(
            v: ObjExampleAsNnJsonbObjRdInn,
        ) -> <Self::PgJson as pg_crud::PgJson>::Rd {
            < Self :: PgJson as pg_crud :: PgType > :: Rd :: try_new (v . field_0 . map (| v_8ff65e09 | pg_crud :: V { v : < pg_crud :: I8AsNnJsonbNbr as pg_crud :: PgJsonTestCases > :: rd_inn_into_rd_with_new_or_try_new_unwraped (v_8ff65e09 . v) }) , v . field_1 . map (| v_8ff65e09 | pg_crud :: V { v : < pg_crud :: OptI8AsNlJsonbNbr as pg_crud :: PgJsonTestCases > :: rd_inn_into_rd_with_new_or_try_new_unwraped (v_8ff65e09 . v) }) , v . field_2 . map (| v_8ff65e09 | pg_crud :: V { v : < pg_crud :: VecOfI8AsNnArrOfNnJsonbNbr as pg_crud :: PgJsonTestCases > :: rd_inn_into_rd_with_new_or_try_new_unwraped (v_8ff65e09 . v) })) . expect ("3aeeabba")
        }
        fn rd_inn_into_upd_with_new_or_try_new_unwraped(
            v: ObjExampleAsNnJsonbObjRdInn,
        ) -> <Self::PgJson as pg_crud::PgJson>::Upd {
            < Self :: PgJson as pg_crud :: PgType > :: Upd :: new (pg_crud :: NotEmptyUnqVec :: try_new ({ let mut acc_ebea163e = Vec :: new () ; acc_ebea163e . extend (v . field_0 . map (| el_23bdfe1e | { ObjExampleAsNnJsonbObjUpdEl :: Field0 (pg_crud :: V { v : < pg_crud :: I8AsNnJsonbNbr as pg_crud :: PgJsonTestCases > :: rd_inn_into_upd_with_new_or_try_new_unwraped (el_23bdfe1e . v) }) })) ; acc_ebea163e . extend (v . field_1 . map (| el_23bdfe1e | { ObjExampleAsNnJsonbObjUpdEl :: Field1 (pg_crud :: V { v : < pg_crud :: OptI8AsNlJsonbNbr as pg_crud :: PgJsonTestCases > :: rd_inn_into_upd_with_new_or_try_new_unwraped (el_23bdfe1e . v) }) })) ; acc_ebea163e . extend (v . field_2 . map (| el_23bdfe1e | { ObjExampleAsNnJsonbObjUpdEl :: Field2 (pg_crud :: V { v : < pg_crud :: VecOfI8AsNnArrOfNnJsonbNbr as pg_crud :: PgJsonTestCases > :: rd_inn_into_upd_with_new_or_try_new_unwraped (el_23bdfe1e . v) }) })) ; acc_ebea163e }) . expect ("a06dbdc5"))
        }
        fn rd_ids_into_opt_v_rd_inn(
            v: <Self::PgJson as pg_crud::PgJson>::RdIds,
        ) -> Option<pg_crud::V<<Self::PgJson as pg_crud::PgJson>::RdInn>> {
            Some (pg_crud :: V { v : ObjExampleAsNnJsonbObjRdInn { field_0 : < pg_crud :: I8AsNnJsonbNbr as pg_crud :: PgJsonTestCases > :: rd_ids_into_opt_v_rd_inn (v . 0. v . field_0) . map_or_else (|| Some (pg_crud :: V { v : < pg_crud :: I8AsNnJsonbNbr as pg_crud :: PgJson > :: into_inn (< < pg_crud :: I8AsNnJsonbNbr as pg_crud :: PgJson > :: Rd as pg_crud :: DfltSomeOneEl > :: dflt_some_one_el ()) }) , Some) , field_1 : < pg_crud :: OptI8AsNlJsonbNbr as pg_crud :: PgJsonTestCases > :: rd_ids_into_opt_v_rd_inn (v . 0. v . field_1) . map_or_else (|| Some (pg_crud :: V { v : < pg_crud :: OptI8AsNlJsonbNbr as pg_crud :: PgJson > :: into_inn (< < pg_crud :: OptI8AsNlJsonbNbr as pg_crud :: PgJson > :: Rd as pg_crud :: DfltSomeOneEl > :: dflt_some_one_el ()) }) , Some) , field_2 : < pg_crud :: VecOfI8AsNnArrOfNnJsonbNbr as pg_crud :: PgJsonTestCases > :: rd_ids_into_opt_v_rd_inn (v . 0. v . field_2) . map_or_else (|| Some (pg_crud :: V { v : < pg_crud :: VecOfI8AsNnArrOfNnJsonbNbr as pg_crud :: PgJson > :: into_inn (< < pg_crud :: VecOfI8AsNnArrOfNnJsonbNbr as pg_crud :: PgJson > :: Rd as pg_crud :: DfltSomeOneEl > :: dflt_some_one_el ()) }) , Some) } })
        }
        fn upd_to_rd_ids(
            v: &<Self::PgJson as pg_crud::PgJson>::Upd,
        ) -> <Self::PgJson as pg_crud::PgJson>::RdIds {
            ObjExampleAsNnJsonbObjRdIds({
                let mut field_0 = None;
                let mut field_1 = None;
                let mut field_2 = None;
                for el_b3974846 in v.0.to_vec() {
                    match el_b3974846 {
                        ObjExampleAsNnJsonbObjUpdEl::Field0(v_0f4d677e) => {
                            field_0 = Some (< pg_crud :: I8AsNnJsonbNbr as pg_crud :: PgJsonTestCases > :: upd_to_rd_ids (& v_0f4d677e . v)) ;
                        }
                        ObjExampleAsNnJsonbObjUpdEl::Field1(v_0f4d677e) => {
                            field_1 = Some (< pg_crud :: OptI8AsNlJsonbNbr as pg_crud :: PgJsonTestCases > :: upd_to_rd_ids (& v_0f4d677e . v)) ;
                        }
                        ObjExampleAsNnJsonbObjUpdEl::Field2(v_0f4d677e) => {
                            field_2 = Some (< pg_crud :: VecOfI8AsNnArrOfNnJsonbNbr as pg_crud :: PgJsonTestCases > :: upd_to_rd_ids (& v_0f4d677e . v)) ;
                        }
                    }
                }
                pg_crud::V {
                    v: ObjExampleAsNnJsonbObjRdIdsH {
                        field_0: field_0.expect("106f16f2"),
                        field_1: field_1.expect("106f16f2"),
                        field_2: field_2.expect("106f16f2"),
                    },
                }
            })
        }
        fn rd_ids_to_opt_v_rd_dflt_some_one_el(
            v: &<Self::PgJson as pg_crud::PgJson>::RdIds,
        ) -> Option<pg_crud::V<<Self::PgJson as pg_crud::PgJson>::Rd>> {
            Some (pg_crud :: V { v : ObjExampleAsNnJsonbObjRd :: try_new (< pg_crud :: I8AsNnJsonbNbr as pg_crud :: PgJsonTestCases > :: rd_ids_to_opt_v_rd_dflt_some_one_el (& v . 0. v . field_0) , < pg_crud :: OptI8AsNlJsonbNbr as pg_crud :: PgJsonTestCases > :: rd_ids_to_opt_v_rd_dflt_some_one_el (& v . 0. v . field_1) , < pg_crud :: VecOfI8AsNnArrOfNnJsonbNbr as pg_crud :: PgJsonTestCases > :: rd_ids_to_opt_v_rd_dflt_some_one_el (& v . 0. v . field_2)) . expect ("57820868") })
        }
        fn previous_rd_and_opt_upd_into_rd(
            rd: <Self::PgJson as pg_crud::PgJson>::Rd,
            opt_upd: Option<<Self::PgJson as pg_crud::PgJson>::Upd>,
        ) -> <Self::PgJson as pg_crud::PgJson>::Rd {
            match opt_upd {
                Some(v_e5377436) => {
                    let mut field_0 = None;
                    let mut field_1 = None;
                    let mut field_2 = None;
                    for el_629b1544 in v_e5377436.0.into_vec() {
                        match el_629b1544 {
                            ObjExampleAsNnJsonbObjUpdEl::Field0(v_730a4dde) => {
                                field_0 = Some(v_730a4dde.v);
                            }
                            ObjExampleAsNnJsonbObjUpdEl::Field1(v_730a4dde) => {
                                field_1 = Some(v_730a4dde.v);
                            }
                            ObjExampleAsNnJsonbObjUpdEl::Field2(v_730a4dde) => {
                                field_2 = Some(v_730a4dde.v);
                            }
                        }
                    }
                    ObjExampleAsNnJsonbObjRd { field_0 : Some (pg_crud :: V { v : < pg_crud :: I8AsNnJsonbNbr as pg_crud :: PgJsonTestCases > :: previous_rd_and_opt_upd_into_rd (rd . field_0 . expect ("a2d26e36") . v , field_0) }) , field_1 : Some (pg_crud :: V { v : < pg_crud :: OptI8AsNlJsonbNbr as pg_crud :: PgJsonTestCases > :: previous_rd_and_opt_upd_into_rd (rd . field_1 . expect ("a2d26e36") . v , field_1) }) , field_2 : Some (pg_crud :: V { v : < pg_crud :: VecOfI8AsNnArrOfNnJsonbNbr as pg_crud :: PgJsonTestCases > :: previous_rd_and_opt_upd_into_rd (rd . field_2 . expect ("a2d26e36") . v , field_2) }) }
                }
                None => rd,
            }
        }
        fn rd_ids_and_cr_into_rd(
            rd_ids: <Self::PgJson as pg_crud::PgJson>::RdIds,
            cr: <Self::PgJson as pg_crud::PgJson>::Cr,
        ) -> <Self::PgJson as pg_crud::PgJson>::Rd {
            ObjExampleAsNnJsonbObjRd :: try_new (< pg_crud :: I8AsNnJsonbNbr as pg_crud :: PgJsonTestCases > :: rd_ids_and_cr_into_opt_v_rd (rd_ids . 0. v . field_0 , cr . field_0) , < pg_crud :: OptI8AsNlJsonbNbr as pg_crud :: PgJsonTestCases > :: rd_ids_and_cr_into_opt_v_rd (rd_ids . 0. v . field_1 , cr . field_1) , < pg_crud :: VecOfI8AsNnArrOfNnJsonbNbr as pg_crud :: PgJsonTestCases > :: rd_ids_and_cr_into_opt_v_rd (rd_ids . 0. v . field_2 , cr . field_2)) . expect ("52ad3994")
        }
        fn rd_ids_and_cr_into_opt_v_rd(
            rd_ids: <Self::PgJson as pg_crud::PgJson>::RdIds,
            cr: <Self::PgJson as pg_crud::PgJson>::Cr,
        ) -> Option<pg_crud::V<<Self::PgJson as pg_crud::PgJson>::Rd>> {
            Some(pg_crud::V {
                v: <Self as pg_crud::PgJsonTestCases>::rd_ids_and_cr_into_rd(rd_ids, cr),
            })
        }
        fn rd_ids_and_cr_into_tt(
            rd_ids: <Self::PgJson as pg_crud::PgJson>::RdIds,
            cr: <Self::PgJson as pg_crud::PgJson>::Cr,
        ) -> <Self::PgJson as pg_crud::PgJson>::Tt {
            ObjExampleAsNnJsonbObjTt :: new (< pg_crud :: I8AsNnJsonbNbr as pg_crud :: PgJsonTestCases > :: rd_ids_and_cr_into_tt (rd_ids . 0. v . field_0 , cr . field_0) , < pg_crud :: OptI8AsNlJsonbNbr as pg_crud :: PgJsonTestCases > :: rd_ids_and_cr_into_tt (rd_ids . 0. v . field_1 , cr . field_1) , < pg_crud :: VecOfI8AsNnArrOfNnJsonbNbr as pg_crud :: PgJsonTestCases > :: rd_ids_and_cr_into_tt (rd_ids . 0. v . field_2 , cr . field_2))
        }
        fn rd_ids_and_cr_into_wh_eq(
            rd_ids: <Self::PgJson as pg_crud::PgJson>::RdIds,
            cr: <Self::PgJson as pg_crud::PgJson>::Cr,
        ) -> <Self::PgJson as pg_crud::PgJson>::Wh {
            ObjExampleAsNnJsonbObjWh :: Eq (pg_crud :: PgJsonWhEq { oprtr : pg_crud :: Oprtr :: Or , v : ObjExampleAsNnJsonbObjTt :: new (< pg_crud :: I8AsNnJsonbNbr as pg_crud :: PgJsonTestCases > :: rd_ids_and_cr_into_tt (rd_ids . 0. v . field_0 , cr . field_0) , < pg_crud :: OptI8AsNlJsonbNbr as pg_crud :: PgJsonTestCases > :: rd_ids_and_cr_into_tt (rd_ids . 0. v . field_1 , cr . field_1) , < pg_crud :: VecOfI8AsNnArrOfNnJsonbNbr as pg_crud :: PgJsonTestCases > :: rd_ids_and_cr_into_tt (rd_ids . 0. v . field_2 , cr . field_2)) })
        }
        fn rd_ids_and_cr_into_vec_wh_eq_using_fields(
            rd_ids: <Self::PgJson as pg_crud::PgJson>::RdIds,
            cr: <Self::PgJson as pg_crud::PgJson>::Cr,
        ) -> pg_crud::NotEmptyUnqVec<<Self::PgJson as pg_crud::PgJson>::Wh> {
            pg_crud :: NotEmptyUnqVec :: try_new (vec ! [ObjExampleAsNnJsonbObjWh :: Field0 (pg_crud :: PgTypeWh :: new (pg_crud :: Oprtr :: And , < pg_crud :: I8AsNnJsonbNbr as pg_crud :: PgJsonTestCases > :: rd_ids_and_cr_into_vec_wh_eq_using_fields (rd_ids . 0. v . field_0 , cr . field_0))) , ObjExampleAsNnJsonbObjWh :: Field1 (pg_crud :: PgTypeWh :: new (pg_crud :: Oprtr :: And , < pg_crud :: OptI8AsNlJsonbNbr as pg_crud :: PgJsonTestCases > :: rd_ids_and_cr_into_vec_wh_eq_using_fields (rd_ids . 0. v . field_1 , cr . field_1))) , ObjExampleAsNnJsonbObjWh :: Field2 (pg_crud :: PgTypeWh :: new (pg_crud :: Oprtr :: And , < pg_crud :: VecOfI8AsNnArrOfNnJsonbNbr as pg_crud :: PgJsonTestCases > :: rd_ids_and_cr_into_vec_wh_eq_using_fields (rd_ids . 0. v . field_2 , cr . field_2)))]) . expect ("ba9c52c1")
        }
        fn rd_ids_and_cr_into_vec_wh_eq_to_json_field(
            rd_ids: <Self::PgJson as pg_crud::PgJson>::RdIds,
            cr: <Self::PgJson as pg_crud::PgJson>::Cr,
        ) -> pg_crud::NotEmptyUnqVec<<Self::PgJson as pg_crud::PgJson>::Wh> {
            pg_crud :: NotEmptyUnqVec :: try_new ({ let mut acc_89ec072c = Vec :: new () ; for el_d830c061 in < pg_crud :: I8AsNnJsonbNbr as pg_crud :: PgJsonTestCases > :: rd_ids_and_cr_into_vec_wh_eq_to_json_field (rd_ids . 0. v . field_0 , cr . field_0) . into_vec () { acc_89ec072c . push (ObjExampleAsNnJsonbObjWh :: Field0 (pg_crud :: PgTypeWh :: try_new (pg_crud :: Oprtr :: Or , vec ! [el_d830c061] ,) . expect ("0c6ccad1") ,)) ; } for el_d830c061 in < pg_crud :: OptI8AsNlJsonbNbr as pg_crud :: PgJsonTestCases > :: rd_ids_and_cr_into_vec_wh_eq_to_json_field (rd_ids . 0. v . field_1 , cr . field_1) . into_vec () { acc_89ec072c . push (ObjExampleAsNnJsonbObjWh :: Field1 (pg_crud :: PgTypeWh :: try_new (pg_crud :: Oprtr :: Or , vec ! [el_d830c061] ,) . expect ("0c6ccad1") ,)) ; } for el_d830c061 in < pg_crud :: VecOfI8AsNnArrOfNnJsonbNbr as pg_crud :: PgJsonTestCases > :: rd_ids_and_cr_into_vec_wh_eq_to_json_field (rd_ids . 0. v . field_2 , cr . field_2) . into_vec () { acc_89ec072c . push (ObjExampleAsNnJsonbObjWh :: Field2 (pg_crud :: PgTypeWh :: try_new (pg_crud :: Oprtr :: Or , vec ! [el_d830c061] ,) . expect ("0c6ccad1") ,)) ; } acc_89ec072c }) . expect ("9c50391c")
        }
        fn rd_ids_and_cr_into_pg_json_opt_vec_wh_dim_one_eq(
            rd_ids: <Self::PgJson as pg_crud::PgJson>::RdIds,
            cr: <Self::PgJson as pg_crud::PgJson>::Cr,
        ) -> Option<pg_crud::NotEmptyUnqVec<<Self::PgJson as pg_crud::PgJson>::Wh>> {
            match pg_crud::NotEmptyUnqVec::try_new({
                let mut acc_2fe1cca8 = Vec::new();
                if let Some (v_2bbd2c96) = < pg_crud :: I8AsNnJsonbNbr as pg_crud :: PgJsonTestCases > :: rd_ids_and_cr_into_pg_json_opt_vec_wh_dim_one_eq (rd_ids . 0. v . field_0 , cr . field_0) { for el in v_2bbd2c96 . clone () . into_vec () { acc_2fe1cca8 . push (ObjExampleAsNnJsonbObjWh :: Field0 (pg_crud :: PgTypeWh :: try_new (pg_crud :: Oprtr :: And , vec ! [el]) . expect ("9a25e058"))) ; } let v_c45bab0d = ObjExampleAsNnJsonbObjWh :: Field0 (pg_crud :: PgTypeWh :: new (pg_crud :: Oprtr :: And , v_2bbd2c96)) ; if ! acc_2fe1cca8 . contains (& v_c45bab0d) { acc_2fe1cca8 . push (v_c45bab0d) ; } }
                if let Some (v_2bbd2c96) = < pg_crud :: OptI8AsNlJsonbNbr as pg_crud :: PgJsonTestCases > :: rd_ids_and_cr_into_pg_json_opt_vec_wh_dim_one_eq (rd_ids . 0. v . field_1 , cr . field_1) { for el in v_2bbd2c96 . clone () . into_vec () { acc_2fe1cca8 . push (ObjExampleAsNnJsonbObjWh :: Field1 (pg_crud :: PgTypeWh :: try_new (pg_crud :: Oprtr :: And , vec ! [el]) . expect ("9a25e058"))) ; } let v_c45bab0d = ObjExampleAsNnJsonbObjWh :: Field1 (pg_crud :: PgTypeWh :: new (pg_crud :: Oprtr :: And , v_2bbd2c96)) ; if ! acc_2fe1cca8 . contains (& v_c45bab0d) { acc_2fe1cca8 . push (v_c45bab0d) ; } }
                if let Some (v_2bbd2c96) = < pg_crud :: VecOfI8AsNnArrOfNnJsonbNbr as pg_crud :: PgJsonTestCases > :: rd_ids_and_cr_into_pg_json_opt_vec_wh_dim_one_eq (rd_ids . 0. v . field_2 , cr . field_2) { for el in v_2bbd2c96 . clone () . into_vec () { acc_2fe1cca8 . push (ObjExampleAsNnJsonbObjWh :: Field2 (pg_crud :: PgTypeWh :: try_new (pg_crud :: Oprtr :: And , vec ! [el]) . expect ("9a25e058"))) ; } let v_c45bab0d = ObjExampleAsNnJsonbObjWh :: Field2 (pg_crud :: PgTypeWh :: new (pg_crud :: Oprtr :: And , v_2bbd2c96)) ; if ! acc_2fe1cca8 . contains (& v_c45bab0d) { acc_2fe1cca8 . push (v_c45bab0d) ; } }
                acc_2fe1cca8
            }) {
                Ok(v_a5fa471d) => Some(v_a5fa471d),
                Err(er) => match er {
                    pg_crud::NotEmptyUnqVecTryNewEr::IsEmpty { .. } => None,
                    pg_crud::NotEmptyUnqVecTryNewEr::NotUnq { .. } => panic!("89e719cf"),
                },
            }
        }
        fn rd_ids_and_cr_into_pg_json_opt_vec_wh_dim_two_eq(
            rd_ids: <Self::PgJson as pg_crud::PgJson>::RdIds,
            cr: <Self::PgJson as pg_crud::PgJson>::Cr,
        ) -> Option<pg_crud::NotEmptyUnqVec<<Self::PgJson as pg_crud::PgJson>::Wh>> {
            match pg_crud::NotEmptyUnqVec::try_new({
                let mut acc_2fe1cca8 = Vec::new();
                if let Some (v_2bbd2c96) = < pg_crud :: I8AsNnJsonbNbr as pg_crud :: PgJsonTestCases > :: rd_ids_and_cr_into_pg_json_opt_vec_wh_dim_two_eq (rd_ids . 0. v . field_0 , cr . field_0) { for el in v_2bbd2c96 . clone () . into_vec () { acc_2fe1cca8 . push (ObjExampleAsNnJsonbObjWh :: Field0 (pg_crud :: PgTypeWh :: try_new (pg_crud :: Oprtr :: And , vec ! [el]) . expect ("9a25e058"))) ; } let v_c45bab0d = ObjExampleAsNnJsonbObjWh :: Field0 (pg_crud :: PgTypeWh :: new (pg_crud :: Oprtr :: And , v_2bbd2c96)) ; if ! acc_2fe1cca8 . contains (& v_c45bab0d) { acc_2fe1cca8 . push (v_c45bab0d) ; } }
                if let Some (v_2bbd2c96) = < pg_crud :: OptI8AsNlJsonbNbr as pg_crud :: PgJsonTestCases > :: rd_ids_and_cr_into_pg_json_opt_vec_wh_dim_two_eq (rd_ids . 0. v . field_1 , cr . field_1) { for el in v_2bbd2c96 . clone () . into_vec () { acc_2fe1cca8 . push (ObjExampleAsNnJsonbObjWh :: Field1 (pg_crud :: PgTypeWh :: try_new (pg_crud :: Oprtr :: And , vec ! [el]) . expect ("9a25e058"))) ; } let v_c45bab0d = ObjExampleAsNnJsonbObjWh :: Field1 (pg_crud :: PgTypeWh :: new (pg_crud :: Oprtr :: And , v_2bbd2c96)) ; if ! acc_2fe1cca8 . contains (& v_c45bab0d) { acc_2fe1cca8 . push (v_c45bab0d) ; } }
                if let Some (v_2bbd2c96) = < pg_crud :: VecOfI8AsNnArrOfNnJsonbNbr as pg_crud :: PgJsonTestCases > :: rd_ids_and_cr_into_pg_json_opt_vec_wh_dim_two_eq (rd_ids . 0. v . field_2 , cr . field_2) { for el in v_2bbd2c96 . clone () . into_vec () { acc_2fe1cca8 . push (ObjExampleAsNnJsonbObjWh :: Field2 (pg_crud :: PgTypeWh :: try_new (pg_crud :: Oprtr :: And , vec ! [el]) . expect ("9a25e058"))) ; } let v_c45bab0d = ObjExampleAsNnJsonbObjWh :: Field2 (pg_crud :: PgTypeWh :: new (pg_crud :: Oprtr :: And , v_2bbd2c96)) ; if ! acc_2fe1cca8 . contains (& v_c45bab0d) { acc_2fe1cca8 . push (v_c45bab0d) ; } }
                acc_2fe1cca8
            }) {
                Ok(v_a5fa471d) => Some(v_a5fa471d),
                Err(er) => match er {
                    pg_crud::NotEmptyUnqVecTryNewEr::IsEmpty { .. } => None,
                    pg_crud::NotEmptyUnqVecTryNewEr::NotUnq { .. } => panic!("89e719cf"),
                },
            }
        }
        fn rd_ids_and_cr_into_pg_json_opt_vec_wh_dim_three_eq(
            rd_ids: <Self::PgJson as pg_crud::PgJson>::RdIds,
            cr: <Self::PgJson as pg_crud::PgJson>::Cr,
        ) -> Option<pg_crud::NotEmptyUnqVec<<Self::PgJson as pg_crud::PgJson>::Wh>> {
            match pg_crud::NotEmptyUnqVec::try_new({
                let mut acc_2fe1cca8 = Vec::new();
                if let Some (v_2bbd2c96) = < pg_crud :: I8AsNnJsonbNbr as pg_crud :: PgJsonTestCases > :: rd_ids_and_cr_into_pg_json_opt_vec_wh_dim_three_eq (rd_ids . 0. v . field_0 , cr . field_0) { for el in v_2bbd2c96 . clone () . into_vec () { acc_2fe1cca8 . push (ObjExampleAsNnJsonbObjWh :: Field0 (pg_crud :: PgTypeWh :: try_new (pg_crud :: Oprtr :: And , vec ! [el]) . expect ("9a25e058"))) ; } let v_c45bab0d = ObjExampleAsNnJsonbObjWh :: Field0 (pg_crud :: PgTypeWh :: new (pg_crud :: Oprtr :: And , v_2bbd2c96)) ; if ! acc_2fe1cca8 . contains (& v_c45bab0d) { acc_2fe1cca8 . push (v_c45bab0d) ; } }
                if let Some (v_2bbd2c96) = < pg_crud :: OptI8AsNlJsonbNbr as pg_crud :: PgJsonTestCases > :: rd_ids_and_cr_into_pg_json_opt_vec_wh_dim_three_eq (rd_ids . 0. v . field_1 , cr . field_1) { for el in v_2bbd2c96 . clone () . into_vec () { acc_2fe1cca8 . push (ObjExampleAsNnJsonbObjWh :: Field1 (pg_crud :: PgTypeWh :: try_new (pg_crud :: Oprtr :: And , vec ! [el]) . expect ("9a25e058"))) ; } let v_c45bab0d = ObjExampleAsNnJsonbObjWh :: Field1 (pg_crud :: PgTypeWh :: new (pg_crud :: Oprtr :: And , v_2bbd2c96)) ; if ! acc_2fe1cca8 . contains (& v_c45bab0d) { acc_2fe1cca8 . push (v_c45bab0d) ; } }
                if let Some (v_2bbd2c96) = < pg_crud :: VecOfI8AsNnArrOfNnJsonbNbr as pg_crud :: PgJsonTestCases > :: rd_ids_and_cr_into_pg_json_opt_vec_wh_dim_three_eq (rd_ids . 0. v . field_2 , cr . field_2) { for el in v_2bbd2c96 . clone () . into_vec () { acc_2fe1cca8 . push (ObjExampleAsNnJsonbObjWh :: Field2 (pg_crud :: PgTypeWh :: try_new (pg_crud :: Oprtr :: And , vec ! [el]) . expect ("9a25e058"))) ; } let v_c45bab0d = ObjExampleAsNnJsonbObjWh :: Field2 (pg_crud :: PgTypeWh :: new (pg_crud :: Oprtr :: And , v_2bbd2c96)) ; if ! acc_2fe1cca8 . contains (& v_c45bab0d) { acc_2fe1cca8 . push (v_c45bab0d) ; } }
                acc_2fe1cca8
            }) {
                Ok(v_a5fa471d) => Some(v_a5fa471d),
                Err(er) => match er {
                    pg_crud::NotEmptyUnqVecTryNewEr::IsEmpty { .. } => None,
                    pg_crud::NotEmptyUnqVecTryNewEr::NotUnq { .. } => panic!("89e719cf"),
                },
            }
        }
        fn rd_ids_and_cr_into_pg_json_opt_vec_wh_dim_four_eq(
            rd_ids: <Self::PgJson as pg_crud::PgJson>::RdIds,
            cr: <Self::PgJson as pg_crud::PgJson>::Cr,
        ) -> Option<pg_crud::NotEmptyUnqVec<<Self::PgJson as pg_crud::PgJson>::Wh>> {
            match pg_crud::NotEmptyUnqVec::try_new({
                let mut acc_2fe1cca8 = Vec::new();
                if let Some (v_2bbd2c96) = < pg_crud :: I8AsNnJsonbNbr as pg_crud :: PgJsonTestCases > :: rd_ids_and_cr_into_pg_json_opt_vec_wh_dim_four_eq (rd_ids . 0. v . field_0 , cr . field_0) { for el in v_2bbd2c96 . clone () . into_vec () { acc_2fe1cca8 . push (ObjExampleAsNnJsonbObjWh :: Field0 (pg_crud :: PgTypeWh :: try_new (pg_crud :: Oprtr :: And , vec ! [el]) . expect ("9a25e058"))) ; } let v_c45bab0d = ObjExampleAsNnJsonbObjWh :: Field0 (pg_crud :: PgTypeWh :: new (pg_crud :: Oprtr :: And , v_2bbd2c96)) ; if ! acc_2fe1cca8 . contains (& v_c45bab0d) { acc_2fe1cca8 . push (v_c45bab0d) ; } }
                if let Some (v_2bbd2c96) = < pg_crud :: OptI8AsNlJsonbNbr as pg_crud :: PgJsonTestCases > :: rd_ids_and_cr_into_pg_json_opt_vec_wh_dim_four_eq (rd_ids . 0. v . field_1 , cr . field_1) { for el in v_2bbd2c96 . clone () . into_vec () { acc_2fe1cca8 . push (ObjExampleAsNnJsonbObjWh :: Field1 (pg_crud :: PgTypeWh :: try_new (pg_crud :: Oprtr :: And , vec ! [el]) . expect ("9a25e058"))) ; } let v_c45bab0d = ObjExampleAsNnJsonbObjWh :: Field1 (pg_crud :: PgTypeWh :: new (pg_crud :: Oprtr :: And , v_2bbd2c96)) ; if ! acc_2fe1cca8 . contains (& v_c45bab0d) { acc_2fe1cca8 . push (v_c45bab0d) ; } }
                if let Some (v_2bbd2c96) = < pg_crud :: VecOfI8AsNnArrOfNnJsonbNbr as pg_crud :: PgJsonTestCases > :: rd_ids_and_cr_into_pg_json_opt_vec_wh_dim_four_eq (rd_ids . 0. v . field_2 , cr . field_2) { for el in v_2bbd2c96 . clone () . into_vec () { acc_2fe1cca8 . push (ObjExampleAsNnJsonbObjWh :: Field2 (pg_crud :: PgTypeWh :: try_new (pg_crud :: Oprtr :: And , vec ! [el]) . expect ("9a25e058"))) ; } let v_c45bab0d = ObjExampleAsNnJsonbObjWh :: Field2 (pg_crud :: PgTypeWh :: new (pg_crud :: Oprtr :: And , v_2bbd2c96)) ; if ! acc_2fe1cca8 . contains (& v_c45bab0d) { acc_2fe1cca8 . push (v_c45bab0d) ; } }
                acc_2fe1cca8
            }) {
                Ok(v_a5fa471d) => Some(v_a5fa471d),
                Err(er) => match er {
                    pg_crud::NotEmptyUnqVecTryNewEr::IsEmpty { .. } => None,
                    pg_crud::NotEmptyUnqVecTryNewEr::NotUnq { .. } => panic!("89e719cf"),
                },
            }
        }
        fn cr_into_pg_json_opt_vec_wh_len_eq(
            cr: <Self::PgJson as pg_crud::PgJson>::Cr,
        ) -> Option<pg_crud::NotEmptyUnqVec<<Self::PgJson as pg_crud::PgJson>::Wh>> {
            match pg_crud::NotEmptyUnqVec::try_new({
                let mut acc_587bf907 = Vec::new();
                if let Some (v_927601a4) = < pg_crud :: I8AsNnJsonbNbr as pg_crud :: PgJsonTestCases > :: cr_into_pg_json_opt_vec_wh_len_eq (cr . field_0) { for el_194a660a in v_927601a4 . clone () . into_vec () { acc_587bf907 . push (ObjExampleAsNnJsonbObjWh :: Field0 (pg_crud :: PgTypeWh :: try_new (pg_crud :: Oprtr :: And , vec ! [el_194a660a]) . expect ("2f437949"))) ; } let v_84ea8e4c = ObjExampleAsNnJsonbObjWh :: Field0 (pg_crud :: PgTypeWh :: new (pg_crud :: Oprtr :: And , v_927601a4)) ; if ! acc_587bf907 . contains (& v_84ea8e4c) { acc_587bf907 . push (v_84ea8e4c) ; } }
                if let Some (v_927601a4) = < pg_crud :: OptI8AsNlJsonbNbr as pg_crud :: PgJsonTestCases > :: cr_into_pg_json_opt_vec_wh_len_eq (cr . field_1) { for el_194a660a in v_927601a4 . clone () . into_vec () { acc_587bf907 . push (ObjExampleAsNnJsonbObjWh :: Field1 (pg_crud :: PgTypeWh :: try_new (pg_crud :: Oprtr :: And , vec ! [el_194a660a]) . expect ("2f437949"))) ; } let v_84ea8e4c = ObjExampleAsNnJsonbObjWh :: Field1 (pg_crud :: PgTypeWh :: new (pg_crud :: Oprtr :: And , v_927601a4)) ; if ! acc_587bf907 . contains (& v_84ea8e4c) { acc_587bf907 . push (v_84ea8e4c) ; } }
                if let Some (v_927601a4) = < pg_crud :: VecOfI8AsNnArrOfNnJsonbNbr as pg_crud :: PgJsonTestCases > :: cr_into_pg_json_opt_vec_wh_len_eq (cr . field_2) { for el_194a660a in v_927601a4 . clone () . into_vec () { acc_587bf907 . push (ObjExampleAsNnJsonbObjWh :: Field2 (pg_crud :: PgTypeWh :: try_new (pg_crud :: Oprtr :: And , vec ! [el_194a660a]) . expect ("2f437949"))) ; } let v_84ea8e4c = ObjExampleAsNnJsonbObjWh :: Field2 (pg_crud :: PgTypeWh :: new (pg_crud :: Oprtr :: And , v_927601a4)) ; if ! acc_587bf907 . contains (& v_84ea8e4c) { acc_587bf907 . push (v_84ea8e4c) ; } }
                acc_587bf907
            }) {
                Ok(v_ea661a62) => Some(v_ea661a62),
                Err(er) => match er {
                    pg_crud::NotEmptyUnqVecTryNewEr::IsEmpty { .. } => None,
                    pg_crud::NotEmptyUnqVecTryNewEr::NotUnq { .. } => panic!("7786dfd4"),
                },
            }
        }
        fn cr_into_pg_json_opt_vec_wh_len_greater_than(
            cr: <Self::PgJson as pg_crud::PgJson>::Cr,
        ) -> Option<pg_crud::NotEmptyUnqVec<<Self::PgJson as pg_crud::PgJson>::Wh>> {
            match pg_crud::NotEmptyUnqVec::try_new({
                let mut acc_f5866fb6 = Vec::new();
                if let Some (v_3432b965) = < pg_crud :: I8AsNnJsonbNbr as pg_crud :: PgJsonTestCases > :: cr_into_pg_json_opt_vec_wh_len_greater_than (cr . field_0) { for el_9bbf8527 in v_3432b965 . clone () . into_vec () { acc_f5866fb6 . push (ObjExampleAsNnJsonbObjWh :: Field0 (pg_crud :: PgTypeWh :: try_new (pg_crud :: Oprtr :: And , vec ! [el_9bbf8527]) . expect ("479db858"))) ; } let el_4a00ab02 = ObjExampleAsNnJsonbObjWh :: Field0 (pg_crud :: PgTypeWh :: new (pg_crud :: Oprtr :: And , v_3432b965)) ; if ! acc_f5866fb6 . contains (& el_4a00ab02) { acc_f5866fb6 . push (el_4a00ab02) ; } }
                if let Some (v_3432b965) = < pg_crud :: OptI8AsNlJsonbNbr as pg_crud :: PgJsonTestCases > :: cr_into_pg_json_opt_vec_wh_len_greater_than (cr . field_1) { for el_9bbf8527 in v_3432b965 . clone () . into_vec () { acc_f5866fb6 . push (ObjExampleAsNnJsonbObjWh :: Field1 (pg_crud :: PgTypeWh :: try_new (pg_crud :: Oprtr :: And , vec ! [el_9bbf8527]) . expect ("479db858"))) ; } let el_4a00ab02 = ObjExampleAsNnJsonbObjWh :: Field1 (pg_crud :: PgTypeWh :: new (pg_crud :: Oprtr :: And , v_3432b965)) ; if ! acc_f5866fb6 . contains (& el_4a00ab02) { acc_f5866fb6 . push (el_4a00ab02) ; } }
                if let Some (v_3432b965) = < pg_crud :: VecOfI8AsNnArrOfNnJsonbNbr as pg_crud :: PgJsonTestCases > :: cr_into_pg_json_opt_vec_wh_len_greater_than (cr . field_2) { for el_9bbf8527 in v_3432b965 . clone () . into_vec () { acc_f5866fb6 . push (ObjExampleAsNnJsonbObjWh :: Field2 (pg_crud :: PgTypeWh :: try_new (pg_crud :: Oprtr :: And , vec ! [el_9bbf8527]) . expect ("479db858"))) ; } let el_4a00ab02 = ObjExampleAsNnJsonbObjWh :: Field2 (pg_crud :: PgTypeWh :: new (pg_crud :: Oprtr :: And , v_3432b965)) ; if ! acc_f5866fb6 . contains (& el_4a00ab02) { acc_f5866fb6 . push (el_4a00ab02) ; } }
                acc_f5866fb6
            }) {
                Ok(v_c4c01cd9) => Some(v_c4c01cd9),
                Err(er) => match er {
                    pg_crud::NotEmptyUnqVecTryNewEr::IsEmpty { .. } => None,
                    pg_crud::NotEmptyUnqVecTryNewEr::NotUnq { .. } => panic!("91d713b5"),
                },
            }
        }
        fn rd_ids_and_cr_into_pg_json_opt_vec_wh_greater_than(
            rd_ids: <Self::PgJson as pg_crud::PgJson>::RdIds,
            cr: <Self::PgJson as pg_crud::PgJson>::Cr,
        ) -> Option<
            pg_crud::NotEmptyUnqVec<
                pg_crud::SingleOrMultiple<<Self::PgJson as pg_crud::PgJson>::Wh>,
            >,
        > {
            match pg_crud::NotEmptyUnqVec::try_new({
                let mut acc_a94bd7fb = Vec::new();
                if let Some (v_a2900ac9) = < pg_crud :: I8AsNnJsonbNbr as pg_crud :: PgJsonTestCases > :: rd_ids_and_cr_into_pg_json_opt_vec_wh_greater_than (rd_ids . 0. v . field_0 , cr . field_0) { let and = pg_crud :: Oprtr :: And ; for el_3e86d33d in v_a2900ac9 . clone () . into_vec () { match el_3e86d33d { pg_crud :: SingleOrMultiple :: Multiple (multiple) => { acc_a94bd7fb . push (pg_crud :: SingleOrMultiple :: Single (ObjExampleAsNnJsonbObjWh :: Field0 (pg_crud :: PgTypeWh :: new (and , multiple)))) ; } , pg_crud :: SingleOrMultiple :: Single (single) => { acc_a94bd7fb . push (pg_crud :: SingleOrMultiple :: Single (ObjExampleAsNnJsonbObjWh :: Field0 (pg_crud :: PgTypeWh :: try_new (and , vec ! [single]) . expect ("2635ede5")))) ; } , } } let v_3e75a2f2 = pg_crud :: SingleOrMultiple :: Single (ObjExampleAsNnJsonbObjWh :: Field0 (pg_crud :: PgTypeWh :: try_new (and , v_a2900ac9 . into_vec () . into_iter () . flat_map (| el_9efefcdc | match el_9efefcdc { pg_crud :: SingleOrMultiple :: Multiple (multiple) => multiple . into_vec () , pg_crud :: SingleOrMultiple :: Single (single) => { std :: iter :: once (single) . collect () } }) . fold (Vec :: new () , | mut acc_be2a6606 , el_7ae146ee | { if ! acc_be2a6606 . contains (& el_7ae146ee) { acc_be2a6606 . push (el_7ae146ee) ; } acc_be2a6606 })) . expect ("e3e5b4ab"))) ; if ! acc_a94bd7fb . contains (& v_3e75a2f2) { acc_a94bd7fb . push (v_3e75a2f2) ; } }
                if let Some (v_a2900ac9) = < pg_crud :: OptI8AsNlJsonbNbr as pg_crud :: PgJsonTestCases > :: rd_ids_and_cr_into_pg_json_opt_vec_wh_greater_than (rd_ids . 0. v . field_1 , cr . field_1) { let and = pg_crud :: Oprtr :: And ; for el_3e86d33d in v_a2900ac9 . clone () . into_vec () { match el_3e86d33d { pg_crud :: SingleOrMultiple :: Multiple (multiple) => { acc_a94bd7fb . push (pg_crud :: SingleOrMultiple :: Single (ObjExampleAsNnJsonbObjWh :: Field1 (pg_crud :: PgTypeWh :: new (and , multiple)))) ; } , pg_crud :: SingleOrMultiple :: Single (single) => { acc_a94bd7fb . push (pg_crud :: SingleOrMultiple :: Single (ObjExampleAsNnJsonbObjWh :: Field1 (pg_crud :: PgTypeWh :: try_new (and , vec ! [single]) . expect ("2635ede5")))) ; } , } } let v_3e75a2f2 = pg_crud :: SingleOrMultiple :: Single (ObjExampleAsNnJsonbObjWh :: Field1 (pg_crud :: PgTypeWh :: try_new (and , v_a2900ac9 . into_vec () . into_iter () . flat_map (| el_9efefcdc | match el_9efefcdc { pg_crud :: SingleOrMultiple :: Multiple (multiple) => multiple . into_vec () , pg_crud :: SingleOrMultiple :: Single (single) => { std :: iter :: once (single) . collect () } }) . fold (Vec :: new () , | mut acc_be2a6606 , el_7ae146ee | { if ! acc_be2a6606 . contains (& el_7ae146ee) { acc_be2a6606 . push (el_7ae146ee) ; } acc_be2a6606 })) . expect ("e3e5b4ab"))) ; if ! acc_a94bd7fb . contains (& v_3e75a2f2) { acc_a94bd7fb . push (v_3e75a2f2) ; } }
                if let Some (v_a2900ac9) = < pg_crud :: VecOfI8AsNnArrOfNnJsonbNbr as pg_crud :: PgJsonTestCases > :: rd_ids_and_cr_into_pg_json_opt_vec_wh_greater_than (rd_ids . 0. v . field_2 , cr . field_2) { let and = pg_crud :: Oprtr :: And ; for el_3e86d33d in v_a2900ac9 . clone () . into_vec () { match el_3e86d33d { pg_crud :: SingleOrMultiple :: Multiple (multiple) => { acc_a94bd7fb . push (pg_crud :: SingleOrMultiple :: Single (ObjExampleAsNnJsonbObjWh :: Field2 (pg_crud :: PgTypeWh :: new (and , multiple)))) ; } , pg_crud :: SingleOrMultiple :: Single (single) => { acc_a94bd7fb . push (pg_crud :: SingleOrMultiple :: Single (ObjExampleAsNnJsonbObjWh :: Field2 (pg_crud :: PgTypeWh :: try_new (and , vec ! [single]) . expect ("2635ede5")))) ; } , } } let v_3e75a2f2 = pg_crud :: SingleOrMultiple :: Single (ObjExampleAsNnJsonbObjWh :: Field2 (pg_crud :: PgTypeWh :: try_new (and , v_a2900ac9 . into_vec () . into_iter () . flat_map (| el_9efefcdc | match el_9efefcdc { pg_crud :: SingleOrMultiple :: Multiple (multiple) => multiple . into_vec () , pg_crud :: SingleOrMultiple :: Single (single) => { std :: iter :: once (single) . collect () } }) . fold (Vec :: new () , | mut acc_be2a6606 , el_7ae146ee | { if ! acc_be2a6606 . contains (& el_7ae146ee) { acc_be2a6606 . push (el_7ae146ee) ; } acc_be2a6606 })) . expect ("e3e5b4ab"))) ; if ! acc_a94bd7fb . contains (& v_3e75a2f2) { acc_a94bd7fb . push (v_3e75a2f2) ; } }
                acc_a94bd7fb
            }) {
                Ok(v_ebe930f0) => Some(v_ebe930f0),
                Err(er) => match er {
                    pg_crud::NotEmptyUnqVecTryNewEr::IsEmpty { .. } => None,
                    pg_crud::NotEmptyUnqVecTryNewEr::NotUnq { .. } => panic!("b877e9c0"),
                },
            }
        }
        fn rd_ids_and_cr_into_pg_json_opt_vec_wh_btwn(
            rd_ids: <Self::PgJson as pg_crud::PgJson>::RdIds,
            cr: <Self::PgJson as pg_crud::PgJson>::Cr,
        ) -> Option<
            pg_crud::NotEmptyUnqVec<
                pg_crud::SingleOrMultiple<<Self::PgJson as pg_crud::PgJson>::Wh>,
            >,
        > {
            match pg_crud::NotEmptyUnqVec::try_new({
                let mut acc_a94bd7fb = Vec::new();
                if let Some (v_a2900ac9) = < pg_crud :: I8AsNnJsonbNbr as pg_crud :: PgJsonTestCases > :: rd_ids_and_cr_into_pg_json_opt_vec_wh_btwn (rd_ids . 0. v . field_0 , cr . field_0) { let and = pg_crud :: Oprtr :: And ; for el_3e86d33d in v_a2900ac9 . clone () . into_vec () { match el_3e86d33d { pg_crud :: SingleOrMultiple :: Multiple (multiple) => { acc_a94bd7fb . push (pg_crud :: SingleOrMultiple :: Single (ObjExampleAsNnJsonbObjWh :: Field0 (pg_crud :: PgTypeWh :: new (and , multiple)))) ; } , pg_crud :: SingleOrMultiple :: Single (single) => { acc_a94bd7fb . push (pg_crud :: SingleOrMultiple :: Single (ObjExampleAsNnJsonbObjWh :: Field0 (pg_crud :: PgTypeWh :: try_new (and , vec ! [single]) . expect ("2635ede5")))) ; } , } } let v_3e75a2f2 = pg_crud :: SingleOrMultiple :: Single (ObjExampleAsNnJsonbObjWh :: Field0 (pg_crud :: PgTypeWh :: try_new (and , v_a2900ac9 . into_vec () . into_iter () . flat_map (| el_9efefcdc | match el_9efefcdc { pg_crud :: SingleOrMultiple :: Multiple (multiple) => multiple . into_vec () , pg_crud :: SingleOrMultiple :: Single (single) => { std :: iter :: once (single) . collect () } }) . fold (Vec :: new () , | mut acc_be2a6606 , el_7ae146ee | { if ! acc_be2a6606 . contains (& el_7ae146ee) { acc_be2a6606 . push (el_7ae146ee) ; } acc_be2a6606 })) . expect ("e3e5b4ab"))) ; if ! acc_a94bd7fb . contains (& v_3e75a2f2) { acc_a94bd7fb . push (v_3e75a2f2) ; } }
                if let Some (v_a2900ac9) = < pg_crud :: OptI8AsNlJsonbNbr as pg_crud :: PgJsonTestCases > :: rd_ids_and_cr_into_pg_json_opt_vec_wh_btwn (rd_ids . 0. v . field_1 , cr . field_1) { let and = pg_crud :: Oprtr :: And ; for el_3e86d33d in v_a2900ac9 . clone () . into_vec () { match el_3e86d33d { pg_crud :: SingleOrMultiple :: Multiple (multiple) => { acc_a94bd7fb . push (pg_crud :: SingleOrMultiple :: Single (ObjExampleAsNnJsonbObjWh :: Field1 (pg_crud :: PgTypeWh :: new (and , multiple)))) ; } , pg_crud :: SingleOrMultiple :: Single (single) => { acc_a94bd7fb . push (pg_crud :: SingleOrMultiple :: Single (ObjExampleAsNnJsonbObjWh :: Field1 (pg_crud :: PgTypeWh :: try_new (and , vec ! [single]) . expect ("2635ede5")))) ; } , } } let v_3e75a2f2 = pg_crud :: SingleOrMultiple :: Single (ObjExampleAsNnJsonbObjWh :: Field1 (pg_crud :: PgTypeWh :: try_new (and , v_a2900ac9 . into_vec () . into_iter () . flat_map (| el_9efefcdc | match el_9efefcdc { pg_crud :: SingleOrMultiple :: Multiple (multiple) => multiple . into_vec () , pg_crud :: SingleOrMultiple :: Single (single) => { std :: iter :: once (single) . collect () } }) . fold (Vec :: new () , | mut acc_be2a6606 , el_7ae146ee | { if ! acc_be2a6606 . contains (& el_7ae146ee) { acc_be2a6606 . push (el_7ae146ee) ; } acc_be2a6606 })) . expect ("e3e5b4ab"))) ; if ! acc_a94bd7fb . contains (& v_3e75a2f2) { acc_a94bd7fb . push (v_3e75a2f2) ; } }
                if let Some (v_a2900ac9) = < pg_crud :: VecOfI8AsNnArrOfNnJsonbNbr as pg_crud :: PgJsonTestCases > :: rd_ids_and_cr_into_pg_json_opt_vec_wh_btwn (rd_ids . 0. v . field_2 , cr . field_2) { let and = pg_crud :: Oprtr :: And ; for el_3e86d33d in v_a2900ac9 . clone () . into_vec () { match el_3e86d33d { pg_crud :: SingleOrMultiple :: Multiple (multiple) => { acc_a94bd7fb . push (pg_crud :: SingleOrMultiple :: Single (ObjExampleAsNnJsonbObjWh :: Field2 (pg_crud :: PgTypeWh :: new (and , multiple)))) ; } , pg_crud :: SingleOrMultiple :: Single (single) => { acc_a94bd7fb . push (pg_crud :: SingleOrMultiple :: Single (ObjExampleAsNnJsonbObjWh :: Field2 (pg_crud :: PgTypeWh :: try_new (and , vec ! [single]) . expect ("2635ede5")))) ; } , } } let v_3e75a2f2 = pg_crud :: SingleOrMultiple :: Single (ObjExampleAsNnJsonbObjWh :: Field2 (pg_crud :: PgTypeWh :: try_new (and , v_a2900ac9 . into_vec () . into_iter () . flat_map (| el_9efefcdc | match el_9efefcdc { pg_crud :: SingleOrMultiple :: Multiple (multiple) => multiple . into_vec () , pg_crud :: SingleOrMultiple :: Single (single) => { std :: iter :: once (single) . collect () } }) . fold (Vec :: new () , | mut acc_be2a6606 , el_7ae146ee | { if ! acc_be2a6606 . contains (& el_7ae146ee) { acc_be2a6606 . push (el_7ae146ee) ; } acc_be2a6606 })) . expect ("e3e5b4ab"))) ; if ! acc_a94bd7fb . contains (& v_3e75a2f2) { acc_a94bd7fb . push (v_3e75a2f2) ; } }
                acc_a94bd7fb
            }) {
                Ok(v_ebe930f0) => Some(v_ebe930f0),
                Err(er) => match er {
                    pg_crud::NotEmptyUnqVecTryNewEr::IsEmpty { .. } => None,
                    pg_crud::NotEmptyUnqVecTryNewEr::NotUnq { .. } => panic!("b877e9c0"),
                },
            }
        }
        fn rd_ids_and_cr_into_pg_json_opt_vec_wh_in(
            rd_ids: <Self::PgJson as pg_crud::PgJson>::RdIds,
            cr: <Self::PgJson as pg_crud::PgJson>::Cr,
        ) -> Option<
            pg_crud::NotEmptyUnqVec<
                pg_crud::SingleOrMultiple<<Self::PgJson as pg_crud::PgJson>::Wh>,
            >,
        > {
            match pg_crud::NotEmptyUnqVec::try_new({
                let mut acc_a94bd7fb = Vec::new();
                if let Some (v_a2900ac9) = < pg_crud :: I8AsNnJsonbNbr as pg_crud :: PgJsonTestCases > :: rd_ids_and_cr_into_pg_json_opt_vec_wh_in (rd_ids . 0. v . field_0 , cr . field_0) { let and = pg_crud :: Oprtr :: And ; for el_3e86d33d in v_a2900ac9 . clone () . into_vec () { match el_3e86d33d { pg_crud :: SingleOrMultiple :: Multiple (multiple) => { acc_a94bd7fb . push (pg_crud :: SingleOrMultiple :: Single (ObjExampleAsNnJsonbObjWh :: Field0 (pg_crud :: PgTypeWh :: new (and , multiple)))) ; } , pg_crud :: SingleOrMultiple :: Single (single) => { acc_a94bd7fb . push (pg_crud :: SingleOrMultiple :: Single (ObjExampleAsNnJsonbObjWh :: Field0 (pg_crud :: PgTypeWh :: try_new (and , vec ! [single]) . expect ("2635ede5")))) ; } , } } let v_3e75a2f2 = pg_crud :: SingleOrMultiple :: Single (ObjExampleAsNnJsonbObjWh :: Field0 (pg_crud :: PgTypeWh :: try_new (and , v_a2900ac9 . into_vec () . into_iter () . flat_map (| el_9efefcdc | match el_9efefcdc { pg_crud :: SingleOrMultiple :: Multiple (multiple) => multiple . into_vec () , pg_crud :: SingleOrMultiple :: Single (single) => { std :: iter :: once (single) . collect () } }) . fold (Vec :: new () , | mut acc_be2a6606 , el_7ae146ee | { if ! acc_be2a6606 . contains (& el_7ae146ee) { acc_be2a6606 . push (el_7ae146ee) ; } acc_be2a6606 })) . expect ("e3e5b4ab"))) ; if ! acc_a94bd7fb . contains (& v_3e75a2f2) { acc_a94bd7fb . push (v_3e75a2f2) ; } }
                if let Some (v_a2900ac9) = < pg_crud :: OptI8AsNlJsonbNbr as pg_crud :: PgJsonTestCases > :: rd_ids_and_cr_into_pg_json_opt_vec_wh_in (rd_ids . 0. v . field_1 , cr . field_1) { let and = pg_crud :: Oprtr :: And ; for el_3e86d33d in v_a2900ac9 . clone () . into_vec () { match el_3e86d33d { pg_crud :: SingleOrMultiple :: Multiple (multiple) => { acc_a94bd7fb . push (pg_crud :: SingleOrMultiple :: Single (ObjExampleAsNnJsonbObjWh :: Field1 (pg_crud :: PgTypeWh :: new (and , multiple)))) ; } , pg_crud :: SingleOrMultiple :: Single (single) => { acc_a94bd7fb . push (pg_crud :: SingleOrMultiple :: Single (ObjExampleAsNnJsonbObjWh :: Field1 (pg_crud :: PgTypeWh :: try_new (and , vec ! [single]) . expect ("2635ede5")))) ; } , } } let v_3e75a2f2 = pg_crud :: SingleOrMultiple :: Single (ObjExampleAsNnJsonbObjWh :: Field1 (pg_crud :: PgTypeWh :: try_new (and , v_a2900ac9 . into_vec () . into_iter () . flat_map (| el_9efefcdc | match el_9efefcdc { pg_crud :: SingleOrMultiple :: Multiple (multiple) => multiple . into_vec () , pg_crud :: SingleOrMultiple :: Single (single) => { std :: iter :: once (single) . collect () } }) . fold (Vec :: new () , | mut acc_be2a6606 , el_7ae146ee | { if ! acc_be2a6606 . contains (& el_7ae146ee) { acc_be2a6606 . push (el_7ae146ee) ; } acc_be2a6606 })) . expect ("e3e5b4ab"))) ; if ! acc_a94bd7fb . contains (& v_3e75a2f2) { acc_a94bd7fb . push (v_3e75a2f2) ; } }
                if let Some (v_a2900ac9) = < pg_crud :: VecOfI8AsNnArrOfNnJsonbNbr as pg_crud :: PgJsonTestCases > :: rd_ids_and_cr_into_pg_json_opt_vec_wh_in (rd_ids . 0. v . field_2 , cr . field_2) { let and = pg_crud :: Oprtr :: And ; for el_3e86d33d in v_a2900ac9 . clone () . into_vec () { match el_3e86d33d { pg_crud :: SingleOrMultiple :: Multiple (multiple) => { acc_a94bd7fb . push (pg_crud :: SingleOrMultiple :: Single (ObjExampleAsNnJsonbObjWh :: Field2 (pg_crud :: PgTypeWh :: new (and , multiple)))) ; } , pg_crud :: SingleOrMultiple :: Single (single) => { acc_a94bd7fb . push (pg_crud :: SingleOrMultiple :: Single (ObjExampleAsNnJsonbObjWh :: Field2 (pg_crud :: PgTypeWh :: try_new (and , vec ! [single]) . expect ("2635ede5")))) ; } , } } let v_3e75a2f2 = pg_crud :: SingleOrMultiple :: Single (ObjExampleAsNnJsonbObjWh :: Field2 (pg_crud :: PgTypeWh :: try_new (and , v_a2900ac9 . into_vec () . into_iter () . flat_map (| el_9efefcdc | match el_9efefcdc { pg_crud :: SingleOrMultiple :: Multiple (multiple) => multiple . into_vec () , pg_crud :: SingleOrMultiple :: Single (single) => { std :: iter :: once (single) . collect () } }) . fold (Vec :: new () , | mut acc_be2a6606 , el_7ae146ee | { if ! acc_be2a6606 . contains (& el_7ae146ee) { acc_be2a6606 . push (el_7ae146ee) ; } acc_be2a6606 })) . expect ("e3e5b4ab"))) ; if ! acc_a94bd7fb . contains (& v_3e75a2f2) { acc_a94bd7fb . push (v_3e75a2f2) ; } }
                acc_a94bd7fb
            }) {
                Ok(v_ebe930f0) => Some(v_ebe930f0),
                Err(er) => match er {
                    pg_crud::NotEmptyUnqVecTryNewEr::IsEmpty { .. } => None,
                    pg_crud::NotEmptyUnqVecTryNewEr::NotUnq { .. } => panic!("b877e9c0"),
                },
            }
        }
        fn rd_ids_and_cr_into_pg_json_opt_vec_wh_rgx(
            rd_ids: <Self::PgJson as pg_crud::PgJson>::RdIds,
            cr: <Self::PgJson as pg_crud::PgJson>::Cr,
        ) -> Option<
            pg_crud::NotEmptyUnqVec<
                pg_crud::SingleOrMultiple<<Self::PgJson as pg_crud::PgJson>::Wh>,
            >,
        > {
            match pg_crud::NotEmptyUnqVec::try_new({
                let mut acc_a94bd7fb = Vec::new();
                if let Some (v_a2900ac9) = < pg_crud :: I8AsNnJsonbNbr as pg_crud :: PgJsonTestCases > :: rd_ids_and_cr_into_pg_json_opt_vec_wh_rgx (rd_ids . 0. v . field_0 , cr . field_0) { let and = pg_crud :: Oprtr :: And ; for el_3e86d33d in v_a2900ac9 . clone () . into_vec () { match el_3e86d33d { pg_crud :: SingleOrMultiple :: Multiple (multiple) => { acc_a94bd7fb . push (pg_crud :: SingleOrMultiple :: Single (ObjExampleAsNnJsonbObjWh :: Field0 (pg_crud :: PgTypeWh :: new (and , multiple)))) ; } , pg_crud :: SingleOrMultiple :: Single (single) => { acc_a94bd7fb . push (pg_crud :: SingleOrMultiple :: Single (ObjExampleAsNnJsonbObjWh :: Field0 (pg_crud :: PgTypeWh :: try_new (and , vec ! [single]) . expect ("2635ede5")))) ; } , } } let v_3e75a2f2 = pg_crud :: SingleOrMultiple :: Single (ObjExampleAsNnJsonbObjWh :: Field0 (pg_crud :: PgTypeWh :: try_new (and , v_a2900ac9 . into_vec () . into_iter () . flat_map (| el_9efefcdc | match el_9efefcdc { pg_crud :: SingleOrMultiple :: Multiple (multiple) => multiple . into_vec () , pg_crud :: SingleOrMultiple :: Single (single) => { std :: iter :: once (single) . collect () } }) . fold (Vec :: new () , | mut acc_be2a6606 , el_7ae146ee | { if ! acc_be2a6606 . contains (& el_7ae146ee) { acc_be2a6606 . push (el_7ae146ee) ; } acc_be2a6606 })) . expect ("e3e5b4ab"))) ; if ! acc_a94bd7fb . contains (& v_3e75a2f2) { acc_a94bd7fb . push (v_3e75a2f2) ; } }
                if let Some (v_a2900ac9) = < pg_crud :: OptI8AsNlJsonbNbr as pg_crud :: PgJsonTestCases > :: rd_ids_and_cr_into_pg_json_opt_vec_wh_rgx (rd_ids . 0. v . field_1 , cr . field_1) { let and = pg_crud :: Oprtr :: And ; for el_3e86d33d in v_a2900ac9 . clone () . into_vec () { match el_3e86d33d { pg_crud :: SingleOrMultiple :: Multiple (multiple) => { acc_a94bd7fb . push (pg_crud :: SingleOrMultiple :: Single (ObjExampleAsNnJsonbObjWh :: Field1 (pg_crud :: PgTypeWh :: new (and , multiple)))) ; } , pg_crud :: SingleOrMultiple :: Single (single) => { acc_a94bd7fb . push (pg_crud :: SingleOrMultiple :: Single (ObjExampleAsNnJsonbObjWh :: Field1 (pg_crud :: PgTypeWh :: try_new (and , vec ! [single]) . expect ("2635ede5")))) ; } , } } let v_3e75a2f2 = pg_crud :: SingleOrMultiple :: Single (ObjExampleAsNnJsonbObjWh :: Field1 (pg_crud :: PgTypeWh :: try_new (and , v_a2900ac9 . into_vec () . into_iter () . flat_map (| el_9efefcdc | match el_9efefcdc { pg_crud :: SingleOrMultiple :: Multiple (multiple) => multiple . into_vec () , pg_crud :: SingleOrMultiple :: Single (single) => { std :: iter :: once (single) . collect () } }) . fold (Vec :: new () , | mut acc_be2a6606 , el_7ae146ee | { if ! acc_be2a6606 . contains (& el_7ae146ee) { acc_be2a6606 . push (el_7ae146ee) ; } acc_be2a6606 })) . expect ("e3e5b4ab"))) ; if ! acc_a94bd7fb . contains (& v_3e75a2f2) { acc_a94bd7fb . push (v_3e75a2f2) ; } }
                if let Some (v_a2900ac9) = < pg_crud :: VecOfI8AsNnArrOfNnJsonbNbr as pg_crud :: PgJsonTestCases > :: rd_ids_and_cr_into_pg_json_opt_vec_wh_rgx (rd_ids . 0. v . field_2 , cr . field_2) { let and = pg_crud :: Oprtr :: And ; for el_3e86d33d in v_a2900ac9 . clone () . into_vec () { match el_3e86d33d { pg_crud :: SingleOrMultiple :: Multiple (multiple) => { acc_a94bd7fb . push (pg_crud :: SingleOrMultiple :: Single (ObjExampleAsNnJsonbObjWh :: Field2 (pg_crud :: PgTypeWh :: new (and , multiple)))) ; } , pg_crud :: SingleOrMultiple :: Single (single) => { acc_a94bd7fb . push (pg_crud :: SingleOrMultiple :: Single (ObjExampleAsNnJsonbObjWh :: Field2 (pg_crud :: PgTypeWh :: try_new (and , vec ! [single]) . expect ("2635ede5")))) ; } , } } let v_3e75a2f2 = pg_crud :: SingleOrMultiple :: Single (ObjExampleAsNnJsonbObjWh :: Field2 (pg_crud :: PgTypeWh :: try_new (and , v_a2900ac9 . into_vec () . into_iter () . flat_map (| el_9efefcdc | match el_9efefcdc { pg_crud :: SingleOrMultiple :: Multiple (multiple) => multiple . into_vec () , pg_crud :: SingleOrMultiple :: Single (single) => { std :: iter :: once (single) . collect () } }) . fold (Vec :: new () , | mut acc_be2a6606 , el_7ae146ee | { if ! acc_be2a6606 . contains (& el_7ae146ee) { acc_be2a6606 . push (el_7ae146ee) ; } acc_be2a6606 })) . expect ("e3e5b4ab"))) ; if ! acc_a94bd7fb . contains (& v_3e75a2f2) { acc_a94bd7fb . push (v_3e75a2f2) ; } }
                acc_a94bd7fb
            }) {
                Ok(v_ebe930f0) => Some(v_ebe930f0),
                Err(er) => match er {
                    pg_crud::NotEmptyUnqVecTryNewEr::IsEmpty { .. } => None,
                    pg_crud::NotEmptyUnqVecTryNewEr::NotUnq { .. } => panic!("b877e9c0"),
                },
            }
        }
        fn rd_ids_and_cr_into_pg_json_opt_vec_wh_contains_el_greater_than(
            rd_ids: <Self::PgJson as pg_crud::PgJson>::RdIds,
            cr: <Self::PgJson as pg_crud::PgJson>::Cr,
        ) -> Option<
            pg_crud::NotEmptyUnqVec<
                pg_crud::SingleOrMultiple<<Self::PgJson as pg_crud::PgJson>::Wh>,
            >,
        > {
            match pg_crud::NotEmptyUnqVec::try_new({
                let mut acc_a94bd7fb = Vec::new();
                if let Some (v_a2900ac9) = < pg_crud :: I8AsNnJsonbNbr as pg_crud :: PgJsonTestCases > :: rd_ids_and_cr_into_pg_json_opt_vec_wh_contains_el_greater_than (rd_ids . 0. v . field_0 , cr . field_0) { let and = pg_crud :: Oprtr :: And ; for el_3e86d33d in v_a2900ac9 . clone () . into_vec () { match el_3e86d33d { pg_crud :: SingleOrMultiple :: Multiple (multiple) => { acc_a94bd7fb . push (pg_crud :: SingleOrMultiple :: Single (ObjExampleAsNnJsonbObjWh :: Field0 (pg_crud :: PgTypeWh :: new (and , multiple)))) ; } , pg_crud :: SingleOrMultiple :: Single (single) => { acc_a94bd7fb . push (pg_crud :: SingleOrMultiple :: Single (ObjExampleAsNnJsonbObjWh :: Field0 (pg_crud :: PgTypeWh :: try_new (and , vec ! [single]) . expect ("2635ede5")))) ; } , } } let v_3e75a2f2 = pg_crud :: SingleOrMultiple :: Single (ObjExampleAsNnJsonbObjWh :: Field0 (pg_crud :: PgTypeWh :: try_new (and , v_a2900ac9 . into_vec () . into_iter () . flat_map (| el_9efefcdc | match el_9efefcdc { pg_crud :: SingleOrMultiple :: Multiple (multiple) => multiple . into_vec () , pg_crud :: SingleOrMultiple :: Single (single) => { std :: iter :: once (single) . collect () } }) . fold (Vec :: new () , | mut acc_be2a6606 , el_7ae146ee | { if ! acc_be2a6606 . contains (& el_7ae146ee) { acc_be2a6606 . push (el_7ae146ee) ; } acc_be2a6606 })) . expect ("e3e5b4ab"))) ; if ! acc_a94bd7fb . contains (& v_3e75a2f2) { acc_a94bd7fb . push (v_3e75a2f2) ; } }
                if let Some (v_a2900ac9) = < pg_crud :: OptI8AsNlJsonbNbr as pg_crud :: PgJsonTestCases > :: rd_ids_and_cr_into_pg_json_opt_vec_wh_contains_el_greater_than (rd_ids . 0. v . field_1 , cr . field_1) { let and = pg_crud :: Oprtr :: And ; for el_3e86d33d in v_a2900ac9 . clone () . into_vec () { match el_3e86d33d { pg_crud :: SingleOrMultiple :: Multiple (multiple) => { acc_a94bd7fb . push (pg_crud :: SingleOrMultiple :: Single (ObjExampleAsNnJsonbObjWh :: Field1 (pg_crud :: PgTypeWh :: new (and , multiple)))) ; } , pg_crud :: SingleOrMultiple :: Single (single) => { acc_a94bd7fb . push (pg_crud :: SingleOrMultiple :: Single (ObjExampleAsNnJsonbObjWh :: Field1 (pg_crud :: PgTypeWh :: try_new (and , vec ! [single]) . expect ("2635ede5")))) ; } , } } let v_3e75a2f2 = pg_crud :: SingleOrMultiple :: Single (ObjExampleAsNnJsonbObjWh :: Field1 (pg_crud :: PgTypeWh :: try_new (and , v_a2900ac9 . into_vec () . into_iter () . flat_map (| el_9efefcdc | match el_9efefcdc { pg_crud :: SingleOrMultiple :: Multiple (multiple) => multiple . into_vec () , pg_crud :: SingleOrMultiple :: Single (single) => { std :: iter :: once (single) . collect () } }) . fold (Vec :: new () , | mut acc_be2a6606 , el_7ae146ee | { if ! acc_be2a6606 . contains (& el_7ae146ee) { acc_be2a6606 . push (el_7ae146ee) ; } acc_be2a6606 })) . expect ("e3e5b4ab"))) ; if ! acc_a94bd7fb . contains (& v_3e75a2f2) { acc_a94bd7fb . push (v_3e75a2f2) ; } }
                if let Some (v_a2900ac9) = < pg_crud :: VecOfI8AsNnArrOfNnJsonbNbr as pg_crud :: PgJsonTestCases > :: rd_ids_and_cr_into_pg_json_opt_vec_wh_contains_el_greater_than (rd_ids . 0. v . field_2 , cr . field_2) { let and = pg_crud :: Oprtr :: And ; for el_3e86d33d in v_a2900ac9 . clone () . into_vec () { match el_3e86d33d { pg_crud :: SingleOrMultiple :: Multiple (multiple) => { acc_a94bd7fb . push (pg_crud :: SingleOrMultiple :: Single (ObjExampleAsNnJsonbObjWh :: Field2 (pg_crud :: PgTypeWh :: new (and , multiple)))) ; } , pg_crud :: SingleOrMultiple :: Single (single) => { acc_a94bd7fb . push (pg_crud :: SingleOrMultiple :: Single (ObjExampleAsNnJsonbObjWh :: Field2 (pg_crud :: PgTypeWh :: try_new (and , vec ! [single]) . expect ("2635ede5")))) ; } , } } let v_3e75a2f2 = pg_crud :: SingleOrMultiple :: Single (ObjExampleAsNnJsonbObjWh :: Field2 (pg_crud :: PgTypeWh :: try_new (and , v_a2900ac9 . into_vec () . into_iter () . flat_map (| el_9efefcdc | match el_9efefcdc { pg_crud :: SingleOrMultiple :: Multiple (multiple) => multiple . into_vec () , pg_crud :: SingleOrMultiple :: Single (single) => { std :: iter :: once (single) . collect () } }) . fold (Vec :: new () , | mut acc_be2a6606 , el_7ae146ee | { if ! acc_be2a6606 . contains (& el_7ae146ee) { acc_be2a6606 . push (el_7ae146ee) ; } acc_be2a6606 })) . expect ("e3e5b4ab"))) ; if ! acc_a94bd7fb . contains (& v_3e75a2f2) { acc_a94bd7fb . push (v_3e75a2f2) ; } }
                acc_a94bd7fb
            }) {
                Ok(v_ebe930f0) => Some(v_ebe930f0),
                Err(er) => match er {
                    pg_crud::NotEmptyUnqVecTryNewEr::IsEmpty { .. } => None,
                    pg_crud::NotEmptyUnqVecTryNewEr::NotUnq { .. } => panic!("b877e9c0"),
                },
            }
        }
        fn rd_ids_and_cr_into_pg_json_opt_vec_wh_contains_el_rgx(
            rd_ids: <Self::PgJson as pg_crud::PgJson>::RdIds,
            cr: <Self::PgJson as pg_crud::PgJson>::Cr,
        ) -> Option<
            pg_crud::NotEmptyUnqVec<
                pg_crud::SingleOrMultiple<<Self::PgJson as pg_crud::PgJson>::Wh>,
            >,
        > {
            match pg_crud::NotEmptyUnqVec::try_new({
                let mut acc_a94bd7fb = Vec::new();
                if let Some (v_a2900ac9) = < pg_crud :: I8AsNnJsonbNbr as pg_crud :: PgJsonTestCases > :: rd_ids_and_cr_into_pg_json_opt_vec_wh_contains_el_rgx (rd_ids . 0. v . field_0 , cr . field_0) { let and = pg_crud :: Oprtr :: And ; for el_3e86d33d in v_a2900ac9 . clone () . into_vec () { match el_3e86d33d { pg_crud :: SingleOrMultiple :: Multiple (multiple) => { acc_a94bd7fb . push (pg_crud :: SingleOrMultiple :: Single (ObjExampleAsNnJsonbObjWh :: Field0 (pg_crud :: PgTypeWh :: new (and , multiple)))) ; } , pg_crud :: SingleOrMultiple :: Single (single) => { acc_a94bd7fb . push (pg_crud :: SingleOrMultiple :: Single (ObjExampleAsNnJsonbObjWh :: Field0 (pg_crud :: PgTypeWh :: try_new (and , vec ! [single]) . expect ("2635ede5")))) ; } , } } let v_3e75a2f2 = pg_crud :: SingleOrMultiple :: Single (ObjExampleAsNnJsonbObjWh :: Field0 (pg_crud :: PgTypeWh :: try_new (and , v_a2900ac9 . into_vec () . into_iter () . flat_map (| el_9efefcdc | match el_9efefcdc { pg_crud :: SingleOrMultiple :: Multiple (multiple) => multiple . into_vec () , pg_crud :: SingleOrMultiple :: Single (single) => { std :: iter :: once (single) . collect () } }) . fold (Vec :: new () , | mut acc_be2a6606 , el_7ae146ee | { if ! acc_be2a6606 . contains (& el_7ae146ee) { acc_be2a6606 . push (el_7ae146ee) ; } acc_be2a6606 })) . expect ("e3e5b4ab"))) ; if ! acc_a94bd7fb . contains (& v_3e75a2f2) { acc_a94bd7fb . push (v_3e75a2f2) ; } }
                if let Some (v_a2900ac9) = < pg_crud :: OptI8AsNlJsonbNbr as pg_crud :: PgJsonTestCases > :: rd_ids_and_cr_into_pg_json_opt_vec_wh_contains_el_rgx (rd_ids . 0. v . field_1 , cr . field_1) { let and = pg_crud :: Oprtr :: And ; for el_3e86d33d in v_a2900ac9 . clone () . into_vec () { match el_3e86d33d { pg_crud :: SingleOrMultiple :: Multiple (multiple) => { acc_a94bd7fb . push (pg_crud :: SingleOrMultiple :: Single (ObjExampleAsNnJsonbObjWh :: Field1 (pg_crud :: PgTypeWh :: new (and , multiple)))) ; } , pg_crud :: SingleOrMultiple :: Single (single) => { acc_a94bd7fb . push (pg_crud :: SingleOrMultiple :: Single (ObjExampleAsNnJsonbObjWh :: Field1 (pg_crud :: PgTypeWh :: try_new (and , vec ! [single]) . expect ("2635ede5")))) ; } , } } let v_3e75a2f2 = pg_crud :: SingleOrMultiple :: Single (ObjExampleAsNnJsonbObjWh :: Field1 (pg_crud :: PgTypeWh :: try_new (and , v_a2900ac9 . into_vec () . into_iter () . flat_map (| el_9efefcdc | match el_9efefcdc { pg_crud :: SingleOrMultiple :: Multiple (multiple) => multiple . into_vec () , pg_crud :: SingleOrMultiple :: Single (single) => { std :: iter :: once (single) . collect () } }) . fold (Vec :: new () , | mut acc_be2a6606 , el_7ae146ee | { if ! acc_be2a6606 . contains (& el_7ae146ee) { acc_be2a6606 . push (el_7ae146ee) ; } acc_be2a6606 })) . expect ("e3e5b4ab"))) ; if ! acc_a94bd7fb . contains (& v_3e75a2f2) { acc_a94bd7fb . push (v_3e75a2f2) ; } }
                if let Some (v_a2900ac9) = < pg_crud :: VecOfI8AsNnArrOfNnJsonbNbr as pg_crud :: PgJsonTestCases > :: rd_ids_and_cr_into_pg_json_opt_vec_wh_contains_el_rgx (rd_ids . 0. v . field_2 , cr . field_2) { let and = pg_crud :: Oprtr :: And ; for el_3e86d33d in v_a2900ac9 . clone () . into_vec () { match el_3e86d33d { pg_crud :: SingleOrMultiple :: Multiple (multiple) => { acc_a94bd7fb . push (pg_crud :: SingleOrMultiple :: Single (ObjExampleAsNnJsonbObjWh :: Field2 (pg_crud :: PgTypeWh :: new (and , multiple)))) ; } , pg_crud :: SingleOrMultiple :: Single (single) => { acc_a94bd7fb . push (pg_crud :: SingleOrMultiple :: Single (ObjExampleAsNnJsonbObjWh :: Field2 (pg_crud :: PgTypeWh :: try_new (and , vec ! [single]) . expect ("2635ede5")))) ; } , } } let v_3e75a2f2 = pg_crud :: SingleOrMultiple :: Single (ObjExampleAsNnJsonbObjWh :: Field2 (pg_crud :: PgTypeWh :: try_new (and , v_a2900ac9 . into_vec () . into_iter () . flat_map (| el_9efefcdc | match el_9efefcdc { pg_crud :: SingleOrMultiple :: Multiple (multiple) => multiple . into_vec () , pg_crud :: SingleOrMultiple :: Single (single) => { std :: iter :: once (single) . collect () } }) . fold (Vec :: new () , | mut acc_be2a6606 , el_7ae146ee | { if ! acc_be2a6606 . contains (& el_7ae146ee) { acc_be2a6606 . push (el_7ae146ee) ; } acc_be2a6606 })) . expect ("e3e5b4ab"))) ; if ! acc_a94bd7fb . contains (& v_3e75a2f2) { acc_a94bd7fb . push (v_3e75a2f2) ; } }
                acc_a94bd7fb
            }) {
                Ok(v_ebe930f0) => Some(v_ebe930f0),
                Err(er) => match er {
                    pg_crud::NotEmptyUnqVecTryNewEr::IsEmpty { .. } => None,
                    pg_crud::NotEmptyUnqVecTryNewEr::NotUnq { .. } => panic!("b877e9c0"),
                },
            }
        }
    }
    #[allow(unused_qualifications)]
    #[allow(clippy::absolute_paths)]
    #[allow(clippy::arbitrary_source_item_ordering)]
    #[cfg(feature = "test-utils")]
    #[allow(clippy::float_arithmetic)]
    impl pg_crud::PgTypeTestCases for ObjExampleAsNnJsonbObj {
        type PgType = Self;
        type Sel = ObjExampleAsNnJsonbObjSel;
        fn opt_vec_cr() -> Option<Vec<<Self::PgType as pg_crud::PgType>::Cr>> {
            <Self as pg_crud::PgJsonTestCases>::opt_vec_cr()
        }
        fn rd_ids_to_2_dims_vec_rd_inn(
            rd_ids: &<Self::PgType as pg_crud::PgType>::RdIds,
        ) -> Vec<Vec<<Self::PgType as pg_crud::PgType>::RdInn>> {
            <Self as pg_crud::PgJsonTestCases>::rd_ids_to_2_dims_vec_rd_inn(rd_ids)
        }
        fn rd_inn_into_rd_with_new_or_try_new_unwraped(
            v: ObjExampleAsNnJsonbObjRdInn,
        ) -> <Self::PgType as pg_crud::PgType>::Rd {
            <Self as pg_crud::PgJsonTestCases>::rd_inn_into_rd_with_new_or_try_new_unwraped(v)
        }
        fn rd_inn_into_upd_with_new_or_try_new_unwraped(
            v: ObjExampleAsNnJsonbObjRdInn,
        ) -> <Self::PgType as pg_crud::PgType>::Upd {
            <Self as pg_crud::PgJsonTestCases>::rd_inn_into_upd_with_new_or_try_new_unwraped(v)
        }
        fn upd_to_rd_ids(
            v: &<Self::PgType as pg_crud::PgType>::Upd,
        ) -> <Self::PgType as pg_crud::PgType>::RdIds {
            <Self as pg_crud::PgJsonTestCases>::upd_to_rd_ids(v)
        }
        fn rd_ids_to_opt_v_rd_dflt_some_one_el(
            v: &<Self::PgType as pg_crud::PgType>::RdIds,
        ) -> Option<pg_crud::V<<Self::PgType as pg_crud::PgType>::Rd>> {
            <Self as pg_crud::PgJsonTestCases>::rd_ids_to_opt_v_rd_dflt_some_one_el(v)
        }
        fn previous_rd_and_opt_upd_into_rd(
            rd: <Self::PgType as pg_crud::PgType>::Rd,
            opt_upd: Option<<Self::PgType as pg_crud::PgType>::Upd>,
        ) -> <Self::PgType as pg_crud::PgType>::Rd {
            <Self as pg_crud::PgJsonTestCases>::previous_rd_and_opt_upd_into_rd(rd, opt_upd)
        }
        fn rd_ids_and_cr_into_rd(
            rd_ids: <Self::PgType as pg_crud::PgType>::RdIds,
            cr: <Self::PgType as pg_crud::PgType>::Cr,
        ) -> <Self::PgType as pg_crud::PgType>::Rd {
            <Self as pg_crud::PgJsonTestCases>::rd_ids_and_cr_into_rd(rd_ids, cr)
        }
        fn rd_ids_and_cr_into_opt_v_rd(
            rd_ids: <Self::PgType as pg_crud::PgType>::RdIds,
            cr: <Self::PgType as pg_crud::PgType>::Cr,
        ) -> Option<pg_crud::V<<Self::PgType as pg_crud::PgType>::Rd>> {
            <Self as pg_crud::PgJsonTestCases>::rd_ids_and_cr_into_opt_v_rd(rd_ids, cr)
        }
        fn rd_ids_and_cr_into_tt(
            rd_ids: <Self::PgType as pg_crud::PgType>::RdIds,
            cr: <Self::PgType as pg_crud::PgType>::Cr,
        ) -> <Self::PgType as pg_crud::PgType>::Tt {
            <Self as pg_crud::PgJsonTestCases>::rd_ids_and_cr_into_tt(rd_ids, cr)
        }
        fn rd_ids_and_cr_into_wh_eq(
            rd_ids: <Self::PgType as pg_crud::PgType>::RdIds,
            cr: <Self::PgType as pg_crud::PgType>::Cr,
        ) -> <Self::PgType as pg_crud::PgType>::Wh {
            <Self as pg_crud::PgJsonTestCases>::rd_ids_and_cr_into_wh_eq(rd_ids, cr)
        }
        fn rd_ids_and_cr_into_vec_wh_eq_using_fields(
            rd_ids: <Self::PgType as pg_crud::PgType>::RdIds,
            cr: <Self::PgType as pg_crud::PgType>::Cr,
        ) -> pg_crud::NotEmptyUnqVec<<Self::PgType as pg_crud::PgType>::Wh> {
            <Self as pg_crud::PgJsonTestCases>::rd_ids_and_cr_into_vec_wh_eq_using_fields(
                rd_ids, cr,
            )
        }
        fn rd_ids_and_cr_into_opt_vec_wh_eq_to_json_field(
            rd_ids: <Self::PgType as pg_crud::PgType>::RdIds,
            cr: <Self::PgType as pg_crud::PgType>::Cr,
        ) -> Option<pg_crud::NotEmptyUnqVec<<Self::PgType as pg_crud::PgType>::Wh>> {
            Some(
                <Self as pg_crud::PgJsonTestCases>::rd_ids_and_cr_into_vec_wh_eq_to_json_field(
                    rd_ids, cr,
                ),
            )
        }
        fn rd_ids_and_cr_into_pg_json_opt_vec_wh_dim_one_eq(
            rd_ids: <Self::PgType as pg_crud::PgType>::RdIds,
            cr: <Self::PgType as pg_crud::PgType>::Cr,
        ) -> Option<pg_crud::NotEmptyUnqVec<<Self::PgType as pg_crud::PgType>::Wh>> {
            <Self as pg_crud::PgJsonTestCases>::rd_ids_and_cr_into_pg_json_opt_vec_wh_dim_one_eq(
                rd_ids, cr,
            )
        }
        fn rd_ids_and_cr_into_pg_json_opt_vec_wh_dim_two_eq(
            rd_ids: <Self::PgType as pg_crud::PgType>::RdIds,
            cr: <Self::PgType as pg_crud::PgType>::Cr,
        ) -> Option<pg_crud::NotEmptyUnqVec<<Self::PgType as pg_crud::PgType>::Wh>> {
            <Self as pg_crud::PgJsonTestCases>::rd_ids_and_cr_into_pg_json_opt_vec_wh_dim_two_eq(
                rd_ids, cr,
            )
        }
        fn rd_ids_and_cr_into_pg_json_opt_vec_wh_dim_three_eq(
            rd_ids: <Self::PgType as pg_crud::PgType>::RdIds,
            cr: <Self::PgType as pg_crud::PgType>::Cr,
        ) -> Option<pg_crud::NotEmptyUnqVec<<Self::PgType as pg_crud::PgType>::Wh>> {
            <Self as pg_crud::PgJsonTestCases>::rd_ids_and_cr_into_pg_json_opt_vec_wh_dim_three_eq(
                rd_ids, cr,
            )
        }
        fn rd_ids_and_cr_into_pg_json_opt_vec_wh_dim_four_eq(
            rd_ids: <Self::PgType as pg_crud::PgType>::RdIds,
            cr: <Self::PgType as pg_crud::PgType>::Cr,
        ) -> Option<pg_crud::NotEmptyUnqVec<<Self::PgType as pg_crud::PgType>::Wh>> {
            <Self as pg_crud::PgJsonTestCases>::rd_ids_and_cr_into_pg_json_opt_vec_wh_dim_four_eq(
                rd_ids, cr,
            )
        }
        fn cr_into_pg_json_opt_vec_wh_len_eq(
            cr: <Self::PgType as pg_crud::PgType>::Cr,
        ) -> Option<pg_crud::NotEmptyUnqVec<<Self::PgType as pg_crud::PgType>::Wh>> {
            <Self as pg_crud::PgJsonTestCases>::cr_into_pg_json_opt_vec_wh_len_eq(cr)
        }
        fn cr_into_pg_json_opt_vec_wh_len_greater_than(
            cr: <Self::PgType as pg_crud::PgType>::Cr,
        ) -> Option<pg_crud::NotEmptyUnqVec<<Self::PgType as pg_crud::PgType>::Wh>> {
            <Self as pg_crud::PgJsonTestCases>::cr_into_pg_json_opt_vec_wh_len_greater_than(cr)
        }
        fn rd_ids_and_cr_into_pg_json_opt_vec_wh_greater_than(
            rd_ids: <Self::PgType as pg_crud::PgType>::RdIds,
            cr: <Self::PgType as pg_crud::PgType>::Cr,
        ) -> Option<
            pg_crud::NotEmptyUnqVec<
                pg_crud::SingleOrMultiple<<Self::PgType as pg_crud::PgType>::Wh>,
            >,
        > {
            <Self as pg_crud::PgJsonTestCases>::rd_ids_and_cr_into_pg_json_opt_vec_wh_greater_than(
                rd_ids, cr,
            )
        }
        fn rd_ids_and_cr_into_pg_json_opt_vec_wh_btwn(
            rd_ids: <Self::PgType as pg_crud::PgType>::RdIds,
            cr: <Self::PgType as pg_crud::PgType>::Cr,
        ) -> Option<
            pg_crud::NotEmptyUnqVec<
                pg_crud::SingleOrMultiple<<Self::PgType as pg_crud::PgType>::Wh>,
            >,
        > {
            <Self as pg_crud::PgJsonTestCases>::rd_ids_and_cr_into_pg_json_opt_vec_wh_btwn(
                rd_ids, cr,
            )
        }
        fn rd_ids_and_cr_into_pg_json_opt_vec_wh_in(
            rd_ids: <Self::PgType as pg_crud::PgType>::RdIds,
            cr: <Self::PgType as pg_crud::PgType>::Cr,
        ) -> Option<
            pg_crud::NotEmptyUnqVec<
                pg_crud::SingleOrMultiple<<Self::PgType as pg_crud::PgType>::Wh>,
            >,
        > {
            <Self as pg_crud::PgJsonTestCases>::rd_ids_and_cr_into_pg_json_opt_vec_wh_in(rd_ids, cr)
        }
        fn rd_ids_and_cr_into_pg_json_opt_vec_wh_rgx(
            rd_ids: <Self::PgType as pg_crud::PgType>::RdIds,
            cr: <Self::PgType as pg_crud::PgType>::Cr,
        ) -> Option<
            pg_crud::NotEmptyUnqVec<
                pg_crud::SingleOrMultiple<<Self::PgType as pg_crud::PgType>::Wh>,
            >,
        > {
            <Self as pg_crud::PgJsonTestCases>::rd_ids_and_cr_into_pg_json_opt_vec_wh_rgx(
                rd_ids, cr,
            )
        }
        fn rd_ids_and_cr_into_pg_json_opt_vec_wh_contains_el_greater_than(
            rd_ids: <Self::PgType as pg_crud::PgType>::RdIds,
            cr: <Self::PgType as pg_crud::PgType>::Cr,
        ) -> Option<
            pg_crud::NotEmptyUnqVec<
                pg_crud::SingleOrMultiple<<Self::PgType as pg_crud::PgType>::Wh>,
            >,
        > {
            < Self as pg_crud :: PgJsonTestCases > :: rd_ids_and_cr_into_pg_json_opt_vec_wh_contains_el_greater_than (rd_ids , cr)
        }
        fn rd_ids_and_cr_into_pg_json_opt_vec_wh_contains_el_rgx(
            rd_ids: <Self::PgType as pg_crud::PgType>::RdIds,
            cr: <Self::PgType as pg_crud::PgType>::Cr,
        ) -> Option<
            pg_crud::NotEmptyUnqVec<
                pg_crud::SingleOrMultiple<<Self::PgType as pg_crud::PgType>::Wh>,
            >,
        > {
            < Self as pg_crud :: PgJsonTestCases > :: rd_ids_and_cr_into_pg_json_opt_vec_wh_contains_el_rgx (rd_ids , cr)
        }
    }
    #[allow(clippy::arbitrary_source_item_ordering)]
    impl pg_crud::PgTypeNotPk for ObjExampleAsNnJsonbObj {
        type PgType = Self;
        type Cr = ObjExampleAsNnJsonbObjCr;
    }
    #[derive(Debug, Clone, Copy)]
    pub struct OptObjExampleAsNlJsonbObj;
    #[allow(clippy::arbitrary_source_item_ordering)]
    #[derive(
        Debug,
        Clone,
        PartialEq,
        serde :: Serialize,
        serde :: Deserialize,
        utoipa :: ToSchema,
        schemars :: JsonSchema,
    )]
    pub struct OptObjExampleAsNlJsonbObjTt(Option<ObjExampleAsNnJsonbObjTt>);
    impl OptObjExampleAsNlJsonbObjTt {
        #[must_use]
        pub const fn new(v: Option<ObjExampleAsNnJsonbObjTt>) -> Self {
            Self(v)
        }
    }
    impl pg_crud::DfltSomeOneEl for OptObjExampleAsNlJsonbObjTt {
        fn dflt_some_one_el() -> Self {
            Self(Some(pg_crud::DfltSomeOneEl::dflt_some_one_el()))
        }
    }
    impl sqlx::Type<sqlx::Postgres> for OptObjExampleAsNlJsonbObjTt {
        fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> bool {
            <sqlx::types::Json<Self> as sqlx::Type<sqlx::Postgres>>::compatible(ty)
        }
        fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
            <sqlx::types::Json<Self> as sqlx::Type<sqlx::Postgres>>::type_info()
        }
    }
    impl sqlx::Encode<'_, sqlx::Postgres> for OptObjExampleAsNlJsonbObjTt {
        fn encode_by_ref(
            &self,
            buf: &mut sqlx::postgres::PgArgumentBuffer,
        ) -> Result<sqlx::encode::IsNull, Box<dyn std::error::Error + Send + Sync>> {
            sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&sqlx::types::Json(self), buf)
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        serde :: Serialize,
        serde :: Deserialize,
        utoipa :: ToSchema,
        schemars :: JsonSchema,
    )]
    pub struct OptObjExampleAsNlJsonbObjCr(Option<ObjExampleAsNnJsonbObjCr>);
    impl OptObjExampleAsNlJsonbObjCr {
        #[must_use]
        pub const fn new(v: Option<ObjExampleAsNnJsonbObjCr>) -> Self {
            Self(v)
        }
    }
    impl pg_crud::DfltSomeOneEl for OptObjExampleAsNlJsonbObjCr {
        fn dflt_some_one_el() -> Self {
            Self(Some(pg_crud::DfltSomeOneEl::dflt_some_one_el()))
        }
    }
    impl sqlx::Type<sqlx::Postgres> for OptObjExampleAsNlJsonbObjCr {
        fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> bool {
            <sqlx::types::Json<Self> as sqlx::Type<sqlx::Postgres>>::compatible(ty)
        }
        fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
            <sqlx::types::Json<Self> as sqlx::Type<sqlx::Postgres>>::type_info()
        }
    }
    impl sqlx::Encode<'_, sqlx::Postgres> for OptObjExampleAsNlJsonbObjCr {
        fn encode_by_ref(
            &self,
            buf: &mut sqlx::postgres::PgArgumentBuffer,
        ) -> Result<sqlx::encode::IsNull, Box<dyn std::error::Error + Send + Sync>> {
            sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&sqlx::types::Json(self), buf)
        }
    }
    impl std::fmt::Display for OptObjExampleAsNlJsonbObjCr {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{self:?}")
        }
    }
    impl loc_lib::ToErrString for OptObjExampleAsNlJsonbObjCr {
        fn to_err_string(&self) -> String {
            format!("{self:#?}")
        }
    }
    #[allow(clippy::arbitrary_source_item_ordering)]
    #[derive(Debug, Clone, PartialEq, serde :: Serialize)]
    pub struct OptObjExampleAsNlJsonbObjCrForQuery(
        Option<<ObjExampleAsNnJsonbObj as pg_crud::PgJson>::CrForQuery>,
    );
    impl From<OptObjExampleAsNlJsonbObjCr> for OptObjExampleAsNlJsonbObjCrForQuery {
        fn from(v: OptObjExampleAsNlJsonbObjCr) -> Self {
            Self(v.0.map(<ObjExampleAsNnJsonbObj as pg_crud::PgJson>::CrForQuery::from))
        }
    }
    impl sqlx::Type<sqlx::Postgres> for OptObjExampleAsNlJsonbObjCrForQuery {
        fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> bool {
            <sqlx::types::Json<Self> as sqlx::Type<sqlx::Postgres>>::compatible(ty)
        }
        fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
            <sqlx::types::Json<Self> as sqlx::Type<sqlx::Postgres>>::type_info()
        }
    }
    impl sqlx::Encode<'_, sqlx::Postgres> for OptObjExampleAsNlJsonbObjCrForQuery {
        fn encode_by_ref(
            &self,
            buf: &mut sqlx::postgres::PgArgumentBuffer,
        ) -> Result<sqlx::encode::IsNull, Box<dyn std::error::Error + Send + Sync>> {
            sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&sqlx::types::Json(self), buf)
        }
    }
    #[allow(clippy::arbitrary_source_item_ordering)]
    #[derive(
        Debug,
        Clone,
        PartialEq,
        serde :: Serialize,
        serde :: Deserialize,
        utoipa :: ToSchema,
        schemars :: JsonSchema,
    )]
    pub struct OptObjExampleAsNlJsonbObjSel(
        Option<<ObjExampleAsNnJsonbObj as pg_crud::PgJson>::Sel>,
    );
    impl OptObjExampleAsNlJsonbObjSel {
        #[must_use]
        pub fn new(v: Option<pg_crud::NotEmptyUnqVec<ObjExampleAsNnJsonbObjSelEl>>) -> Self {
            Self(v.map(<ObjExampleAsNnJsonbObj as pg_crud::PgJson>::Sel::new))
        }
        fn sel_qp_pg_type(&self, col: &str) -> Result<String, pg_crud::QpEr> {
            let v = self . 0 . as_ref () . map_or_else (< < ObjExampleAsNnJsonbObj as pg_crud :: PgJson > :: Sel as pg_crud :: DfltSomeOneEl > :: dflt_some_one_el , Clone :: clone) ;
            match v.sel_qp_pg_type(col) {
                Ok(v_c69f1ffe) => Ok(pg_crud::case_jsonb_typeof_null(&col, &v_c69f1ffe)),
                Err(er) => Err(er),
            }
        }
    }
    impl sqlx::Type<sqlx::Postgres> for OptObjExampleAsNlJsonbObjSel {
        fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> bool {
            <sqlx::types::Json<Self> as sqlx::Type<sqlx::Postgres>>::compatible(ty)
        }
        fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
            <sqlx::types::Json<Self> as sqlx::Type<sqlx::Postgres>>::type_info()
        }
    }
    impl sqlx::Decode<'_, sqlx::Postgres> for OptObjExampleAsNlJsonbObjSel {
        fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
            match <sqlx::types::Json<Self> as sqlx::Decode<sqlx::Postgres>>::decode(value) {
                Ok(v) => Ok(v.0),
                Err(er) => Err(er),
            }
        }
    }
    impl pg_crud::DfltSomeOneEl for OptObjExampleAsNlJsonbObjSel {
        fn dflt_some_one_el() -> Self {
            Self(Some(pg_crud::DfltSomeOneEl::dflt_some_one_el()))
        }
    }
    impl pg_crud::DfltSomeOneElMaxPageSize for OptObjExampleAsNlJsonbObjSel {
        fn dflt_some_one_el_max_page_size() -> Self {
            Self(Some(
                pg_crud::DfltSomeOneElMaxPageSize::dflt_some_one_el_max_page_size(),
            ))
        }
    }
    pub type OptObjExampleAsNlJsonbObjWh =
        pg_crud::NlJsonObjPgTypeWhFlt<<ObjExampleAsNnJsonbObj as pg_crud::PgJson>::Wh>;
    #[allow(clippy::arbitrary_source_item_ordering)]
    #[derive(
        Debug,
        Clone,
        PartialEq,
        serde :: Serialize,
        serde :: Deserialize,
        utoipa :: ToSchema,
        schemars :: JsonSchema,
    )]
    pub struct OptObjExampleAsNlJsonbObjRd(Option<<ObjExampleAsNnJsonbObj as pg_crud::PgJson>::Rd>);
    impl OptObjExampleAsNlJsonbObjRd {
        #[must_use]
        pub const fn new(v: Option<<ObjExampleAsNnJsonbObj as pg_crud::PgJson>::Rd>) -> Self {
            Self(v)
        }
    }
    impl pg_crud::DfltSomeOneEl for OptObjExampleAsNlJsonbObjRd {
        fn dflt_some_one_el() -> Self {
            Self(Some(pg_crud::DfltSomeOneEl::dflt_some_one_el()))
        }
    }
    impl sqlx::Type<sqlx::Postgres> for OptObjExampleAsNlJsonbObjRd {
        fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> bool {
            <sqlx::types::Json<Self> as sqlx::Type<sqlx::Postgres>>::compatible(ty)
        }
        fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
            <sqlx::types::Json<Self> as sqlx::Type<sqlx::Postgres>>::type_info()
        }
    }
    impl sqlx::Encode<'_, sqlx::Postgres> for OptObjExampleAsNlJsonbObjRd {
        fn encode_by_ref(
            &self,
            buf: &mut sqlx::postgres::PgArgumentBuffer,
        ) -> Result<sqlx::encode::IsNull, Box<dyn std::error::Error + Send + Sync>> {
            sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&sqlx::types::Json(self), buf)
        }
    }
    impl sqlx::Decode<'_, sqlx::Postgres> for OptObjExampleAsNlJsonbObjRd {
        fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
            match <sqlx::types::Json<Self> as sqlx::Decode<sqlx::Postgres>>::decode(value) {
                Ok(v) => Ok(v.0),
                Err(er) => Err(er),
            }
        }
    }
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    pub struct OptObjExampleAsNlJsonbObjRdIds(pg_crud::V<Option<ObjExampleAsNnJsonbObjRdIds>>);
    impl sqlx::Decode<'_, sqlx::Postgres> for OptObjExampleAsNlJsonbObjRdIds {
        fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
            match <sqlx::types::Json<Self> as sqlx::Decode<sqlx::Postgres>>::decode(value) {
                Ok(v) => Ok(v.0),
                Err(er) => Err(er),
            }
        }
    }
    impl sqlx::Type<sqlx::Postgres> for OptObjExampleAsNlJsonbObjRdIds {
        fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> bool {
            <sqlx::types::Json<Self> as sqlx::Type<sqlx::Postgres>>::compatible(ty)
        }
        fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
            <sqlx::types::Json<Self> as sqlx::Type<sqlx::Postgres>>::type_info()
        }
    }
    pub type OptObjExampleAsNlJsonbObjRdInn =
        Option<<ObjExampleAsNnJsonbObj as pg_crud::PgJson>::RdInn>;
    #[allow(clippy::arbitrary_source_item_ordering)]
    #[derive(
        Debug,
        Clone,
        PartialEq,
        serde :: Serialize,
        serde :: Deserialize,
        utoipa :: ToSchema,
        schemars :: JsonSchema,
    )]
    pub struct OptObjExampleAsNlJsonbObjUpd(
        Option<<ObjExampleAsNnJsonbObj as pg_crud::PgJson>::Upd>,
    );
    impl OptObjExampleAsNlJsonbObjUpd {
        #[must_use]
        pub const fn new(v: Option<<ObjExampleAsNnJsonbObj as pg_crud::PgJson>::Upd>) -> Self {
            Self(v)
        }
    }
    impl pg_crud::DfltSomeOneEl for OptObjExampleAsNlJsonbObjUpd {
        fn dflt_some_one_el() -> Self {
            Self(Some(pg_crud::DfltSomeOneEl::dflt_some_one_el()))
        }
    }
    #[allow(clippy::arbitrary_source_item_ordering)]
    #[derive(Debug, Clone, PartialEq, serde :: Serialize)]
    pub struct OptObjExampleAsNlJsonbObjUpdForQuery(
        Option<<ObjExampleAsNnJsonbObj as pg_crud::PgJson>::UpdForQuery>,
    );
    impl OptObjExampleAsNlJsonbObjUpdForQuery {
        #[allow(clippy::single_call_fn)]
        fn sel_only_updd_ids_qp(
            &self,
            col_field: &str,
            incr: &mut u64,
        ) -> Result<String, pg_crud::QpEr> {
            Ok(match &self.0 {
                Some(v_9570957e) => {
                    let mut acc_f7537df2 = String::default();
                    for el in v_9570957e.0.to_vec() {
                        match & el { ObjExampleAsNnJsonbObjUpdForQueryEl :: Field0 (v_92d002a5) => match < pg_crud :: I8AsNnJsonbNbr as pg_crud :: PgJson > :: sel_only_updd_ids_qp (& v_92d002a5 . v , "field_0" , col_field , incr) { Ok (mut v_a9da8905) => { let _ : Option < char > = v_a9da8905 . pop () ; if { use std :: fmt :: Write as _ ; write ! (acc_f7537df2 , "jsonb_build_object({v_a9da8905})||") } . is_err () { return Err (pg_crud :: QpEr :: WriteIntoBuffer { loc : loc_lib :: loc ! () }) ; } } , Err (er) => { return Err (er) ; } } , ObjExampleAsNnJsonbObjUpdForQueryEl :: Field1 (v_92d002a5) => match < pg_crud :: OptI8AsNlJsonbNbr as pg_crud :: PgJson > :: sel_only_updd_ids_qp (& v_92d002a5 . v , "field_1" , col_field , incr) { Ok (mut v_a9da8905) => { let _ : Option < char > = v_a9da8905 . pop () ; if { use std :: fmt :: Write as _ ; write ! (acc_f7537df2 , "jsonb_build_object({v_a9da8905})||") } . is_err () { return Err (pg_crud :: QpEr :: WriteIntoBuffer { loc : loc_lib :: loc ! () }) ; } } , Err (er) => { return Err (er) ; } } , ObjExampleAsNnJsonbObjUpdForQueryEl :: Field2 (v_92d002a5) => match < pg_crud :: VecOfI8AsNnArrOfNnJsonbNbr as pg_crud :: PgJson > :: sel_only_updd_ids_qp (& v_92d002a5 . v , "field_2" , col_field , incr) { Ok (mut v_a9da8905) => { let _ : Option < char > = v_a9da8905 . pop () ; if { use std :: fmt :: Write as _ ; write ! (acc_f7537df2 , "jsonb_build_object({v_a9da8905})||") } . is_err () { return Err (pg_crud :: QpEr :: WriteIntoBuffer { loc : loc_lib :: loc ! () }) ; } } , Err (er) => { return Err (er) ; } } }
                    }
                    let _: Option<char> = acc_f7537df2.pop();
                    let _: Option<char> = acc_f7537df2.pop();
                    format!("jsonb_build_object('v',{acc_f7537df2})")
                }
                None => pg_crud::NULL_JSONB.to_owned(),
            })
        }
    }
    impl From<<OptObjExampleAsNlJsonbObj as pg_crud::PgJson>::Upd>
        for <OptObjExampleAsNlJsonbObj as pg_crud::PgJson>::UpdForQuery
    {
        fn from(v: <OptObjExampleAsNlJsonbObj as pg_crud::PgJson>::Upd) -> Self {
            Self(v.0.map(<ObjExampleAsNnJsonbObj as pg_crud::PgJson>::UpdForQuery::from))
        }
    }
    #[allow(clippy::arbitrary_source_item_ordering)]
    impl pg_crud::PgJson for OptObjExampleAsNlJsonbObj {
        type Tt = OptObjExampleAsNlJsonbObjTt;
        type Cr = OptObjExampleAsNlJsonbObjCr;
        type CrForQuery = OptObjExampleAsNlJsonbObjCrForQuery;
        type Sel = OptObjExampleAsNlJsonbObjSel;
        fn sel_qp(
            v: &Self::Sel,
            fi: &str,
            col_field: &str,
            col_field_for_er_msg: &str,
            _: bool,
        ) -> Result<String, pg_crud::QpEr> {
            let col_field_fi = format!("{col_field}->'{fi}'");
            let v_46039f0e = v . 0 . as_ref () . map_or_else (< < ObjExampleAsNnJsonbObj as pg_crud :: PgJson > :: Sel as pg_crud :: DfltSomeOneEl > :: dflt_some_one_el , Clone :: clone) ;
            match <ObjExampleAsNnJsonbObj as pg_crud::PgJson>::sel_qp(
                &v_46039f0e,
                fi,
                &col_field_fi,
                col_field_for_er_msg,
                true,
            ) {
                Ok(v_1f8de96a) => Ok(format!(
                    "jsonb_build_object('{fi}',jsonb_build_object('v',case when jsonb_typeof({col_field_fi}) = 'null' then 'null'::jsonb else ({v_1f8de96a}) end))"
                )),
                Err(er) => Err(er),
            }
        }
        type Wh = OptObjExampleAsNlJsonbObjWh;
        type Rd = OptObjExampleAsNlJsonbObjRd;
        type RdIds = OptObjExampleAsNlJsonbObjRdIds;
        fn sel_only_ids_qp(col_field: &str) -> Result<String, pg_crud::QpEr> {
            match <ObjExampleAsNnJsonbObj as pg_crud::PgJson>::sel_only_ids_qp(col_field) {
                Ok(v_21000130) => Ok(format!(
                    "jsonb_build_object('v',case when jsonb_typeof({col_field}) = 'null' then 'null'::jsonb else ({v_21000130}) end)"
                )),
                Err(er) => Err(er),
            }
        }
        type RdInn = OptObjExampleAsNlJsonbObjRdInn;
        fn into_inn(v: Self::Rd) -> Self::RdInn {
            v.0.map(<ObjExampleAsNnJsonbObj as pg_crud::PgJson>::into_inn)
        }
        type Upd = OptObjExampleAsNlJsonbObjUpd;
        type UpdForQuery = OptObjExampleAsNlJsonbObjUpdForQuery;
        fn upd_qp(
            v: &Self::UpdForQuery,
            jsonb_set_accumulator: &str,
            jsonb_set_target: &str,
            jsonb_set_path: &str,
            incr: &mut u64,
        ) -> Result<String, pg_crud::QpEr> {
            let _: &str = jsonb_set_accumulator;
            let _: &str = jsonb_set_path;
            match &v.0 {
                Some(v_92f34435) => <ObjExampleAsNnJsonbObj as pg_crud::PgJson>::upd_qp(
                    v_92f34435,
                    jsonb_set_accumulator,
                    jsonb_set_target,
                    jsonb_set_path,
                    incr,
                ),
                None => match pg_crud::incr_checked_add_one_returning_incr(incr) {
                    Ok(v_27b8537f) => Ok(format!(
                        "jsonb_set({jsonb_set_accumulator},'{{{jsonb_set_path}}}',${v_27b8537f})"
                    )),
                    Err(er) => Err(er),
                },
            }
        }
        fn upd_qb(
            v: Self::UpdForQuery,
            mut query: sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>,
        ) -> Result<sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>, String>
        {
            match v.0 {
                Some(v_269a0d34) => {
                    <ObjExampleAsNnJsonbObj as pg_crud::PgJson>::upd_qb(v_269a0d34, query)
                }
                None => {
                    if let Err(er) =
                        query.try_bind(sqlx::types::Json(<Self as pg_crud::PgJson>::Upd::new(None)))
                    {
                        Err(er.to_string())
                    } else {
                        Ok(query)
                    }
                }
            }
        }
        fn sel_only_updd_ids_qp(
            v: &Self::UpdForQuery,
            fi: &str,
            col_field: &str,
            incr: &mut u64,
        ) -> Result<String, pg_crud::QpEr> {
            match v.sel_only_updd_ids_qp(&format!("{col_field}->'{fi}'"), incr) {
                Ok(v_e137951b) => Ok(format!("'{fi}',jsonb_build_object('v',{v_e137951b}),")),
                Err(er) => Err(er),
            }
        }
        fn sel_only_updd_ids_qb<'lt>(
            v: &'lt Self::UpdForQuery,
            mut query: sqlx::query::Query<'lt, sqlx::Postgres, sqlx::postgres::PgArguments>,
        ) -> Result<sqlx::query::Query<'lt, sqlx::Postgres, sqlx::postgres::PgArguments>, String>
        {
            if let Some(v_6334d77d) = &v.0 {
                match <ObjExampleAsNnJsonbObj as pg_crud::PgJson>::sel_only_updd_ids_qb(
                    v_6334d77d, query,
                ) {
                    Ok(v_0bd3ba6f) => {
                        query = v_0bd3ba6f;
                    }
                    Err(er) => {
                        return Err(er);
                    }
                }
            }
            Ok(query)
        }
        fn sel_only_crd_ids_qp(
            v: &Self::CrForQuery,
            fi: &str,
            col_field: &str,
            incr: &mut u64,
        ) -> Result<String, pg_crud::QpEr> {
            Ok(format!(
                "'{fi}'jsonb_build_object('v',{}),",
                match &v.0 {
                    Some(v_90219286) =>
                        format!("jsonb_build_object('v',{})", {
                            let mut acc_0e9170a3 = String::new();
                            match <pg_crud::I8AsNnJsonbNbr as pg_crud::PgJson>::sel_only_crd_ids_qp(
                                &v_90219286.field_0,
                                "field_0",
                                &format!("{col_field}->'field_0'"),
                                incr,
                            ) {
                                Ok(mut v_93015133) => {
                                    let _: Option<char> = v_93015133.pop();
                                    if {
                                        use std::fmt::Write as _;
                                        write!(acc_0e9170a3, "jsonb_build_object({v_93015133})||")
                                    }
                                    .is_err()
                                    {
                                        return Err(pg_crud::QpEr::WriteIntoBuffer {
                                            loc: loc_lib::loc!(),
                                        });
                                    }
                                }
                                Err(er) => {
                                    return Err(er);
                                }
                            }
                            match < pg_crud :: OptI8AsNlJsonbNbr as pg_crud :: PgJson > :: sel_only_crd_ids_qp (& v_90219286 . field_1 , "field_1" , & format ! ("{col_field}->'field_1'") , incr) { Ok (mut v_93015133) => { let _ : Option < char > = v_93015133 . pop () ; if { use std :: fmt :: Write as _ ; write ! (acc_0e9170a3 , "jsonb_build_object({v_93015133})||") } . is_err () { return Err (pg_crud :: QpEr :: WriteIntoBuffer { loc : loc_lib :: loc ! () }) ; } } , Err (er) => { return Err (er) ; } }
                            match < pg_crud :: VecOfI8AsNnArrOfNnJsonbNbr as pg_crud :: PgJson > :: sel_only_crd_ids_qp (& v_90219286 . field_2 , "field_2" , & format ! ("{col_field}->'field_2'") , incr) { Ok (mut v_93015133) => { let _ : Option < char > = v_93015133 . pop () ; if { use std :: fmt :: Write as _ ; write ! (acc_0e9170a3 , "jsonb_build_object({v_93015133})||") } . is_err () { return Err (pg_crud :: QpEr :: WriteIntoBuffer { loc : loc_lib :: loc ! () }) ; } } , Err (er) => { return Err (er) ; } }
                            let _: Option<char> = acc_0e9170a3.pop();
                            let _: Option<char> = acc_0e9170a3.pop();
                            acc_0e9170a3
                        }),
                    None => pg_crud::NULL_JSONB.to_owned(),
                }
            ))
        }
        fn sel_only_crd_ids_qb<'lt>(
            v: &'lt Self::CrForQuery,
            mut query: sqlx::query::Query<'lt, sqlx::Postgres, sqlx::postgres::PgArguments>,
        ) -> Result<sqlx::query::Query<'lt, sqlx::Postgres, sqlx::postgres::PgArguments>, String>
        {
            if let Some(v_a1ccd526) = &v.0 {
                match <ObjExampleAsNnJsonbObj as pg_crud::PgJson>::sel_only_crd_ids_qb(
                    v_a1ccd526, query,
                ) {
                    Ok(v_70ed6013) => {
                        query = v_70ed6013;
                    }
                    Err(er) => {
                        return Err(er);
                    }
                }
            }
            Ok(query)
        }
    }
    #[allow(clippy::arbitrary_source_item_ordering)]
    impl pg_crud::PgType for OptObjExampleAsNlJsonbObj {
        type Tt = OptObjExampleAsNlJsonbObjTt;
        fn cr_tbl_col_qp(col: &dyn std::fmt::Display, _: bool) -> impl std::fmt::Display {
            format!(
                "{col} jsonb not null check (jsonb_matches_schema('{}', {col}))",
                serde_json::to_string(&schemars::schema_for!(OptObjExampleAsNlJsonbObjTt))
                    .expect("59a1654b")
            )
        }
        type Cr = OptObjExampleAsNlJsonbObjCr;
        fn cr_qp(_: &Self::Cr, incr: &mut u64) -> Result<String, pg_crud::QpEr> {
            match pg_crud::incr_checked_add_one_returning_incr(incr) {
                Ok(v_7df9eb00) => Ok(format!("${v_7df9eb00}")),
                Err(er) => Err(er),
            }
        }
        fn cr_qb(
            v: Self::Cr,
            mut query: sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>,
        ) -> Result<sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>, String>
        {
            if let Err(er) = query.try_bind(<Self as pg_crud::PgJson>::CrForQuery::from(v)) {
                return Err(er.to_string());
            }
            Ok(query)
        }
        type Sel = OptObjExampleAsNlJsonbObjSel;
        fn sel_qp(v: &Self::Sel, col: &str) -> Result<String, pg_crud::QpEr> {
            match v.sel_qp_pg_type(col) {
                Ok(v_d91c19a6) => Ok(format!("{v_d91c19a6} as {col}")),
                Err(er) => Err(er),
            }
        }
        type Wh = OptObjExampleAsNlJsonbObjWh;
        type Rd = OptObjExampleAsNlJsonbObjRd;
        fn normalize(v: Self::Rd) -> Self::Rd {
            v
        }
        type RdIds = OptObjExampleAsNlJsonbObjRdIds;
        fn sel_only_ids_qp(col: &str) -> Result<String, pg_crud::QpEr> {
            match <Self as pg_crud::PgJson>::sel_only_ids_qp(col) {
                Ok(v_e776e9fa) => Ok(format!("{v_e776e9fa} as {col},")),
                Err(er) => Err(er),
            }
        }
        type RdInn = OptObjExampleAsNlJsonbObjRdInn;
        fn into_inn(v: Self::Rd) -> Self::RdInn {
            <Self as pg_crud::PgJson>::into_inn(v)
        }
        type Upd = OptObjExampleAsNlJsonbObjUpd;
        type UpdForQuery = OptObjExampleAsNlJsonbObjUpdForQuery;
        #[allow(unused_variables)]
        fn upd_qp(
            v: &Self::UpdForQuery,
            jsonb_set_accumulator: &str,
            jsonb_set_target: &str,
            jsonb_set_path: &str,
            incr: &mut u64,
        ) -> Result<String, pg_crud::QpEr> {
            match &v.0 {
                Some(v_92f34435) => <ObjExampleAsNnJsonbObj as pg_crud::PgJson>::upd_qp(
                    v_92f34435,
                    jsonb_set_accumulator,
                    jsonb_set_target,
                    jsonb_set_path,
                    incr,
                ),
                None => match pg_crud::incr_checked_add_one_returning_incr(incr) {
                    Ok(v_27b8537f) => Ok(format!("${v_27b8537f}")),
                    Err(er) => Err(er),
                },
            }
        }
        fn upd_qb(
            v: Self::UpdForQuery,
            query: sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>,
        ) -> Result<sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>, String>
        {
            <Self as pg_crud::PgJson>::upd_qb(v, query)
        }
        fn sel_only_updd_ids_qp(
            v: &Self::UpdForQuery,
            col: &str,
            incr: &mut u64,
        ) -> Result<String, pg_crud::QpEr> {
            match v.sel_only_updd_ids_qp(col, incr) {
                Ok(v_f0787243) => Ok(format!("jsonb_build_object('v',{v_f0787243}) as {col},")),
                Err(er) => Err(er),
            }
        }
        fn sel_only_updd_ids_qb<'lt>(
            v: &'lt Self::UpdForQuery,
            query: sqlx::query::Query<'lt, sqlx::Postgres, sqlx::postgres::PgArguments>,
        ) -> Result<sqlx::query::Query<'lt, sqlx::Postgres, sqlx::postgres::PgArguments>, String>
        {
            <Self as pg_crud::PgJson>::sel_only_updd_ids_qb(v, query)
        }
    }
    #[allow(unused_qualifications)]
    #[allow(clippy::absolute_paths)]
    #[allow(clippy::arbitrary_source_item_ordering)]
    #[cfg(feature = "test-utils")]
    #[allow(clippy::float_arithmetic)]
    impl pg_crud::PgJsonTestCases for OptObjExampleAsNlJsonbObj {
        type PgJson = Self;
        type Sel = OptObjExampleAsNlJsonbObjSel;
        fn opt_vec_cr() -> Option<Vec<<Self::PgJson as pg_crud::PgJson>::Cr>> {
            Some({
                let mut acc_ccd79a32 = Vec::new();
                if let Some(v_399e6a50) =
                    <ObjExampleAsNnJsonbObj as pg_crud::PgJsonTestCases>::opt_vec_cr()
                {
                    for el in v_399e6a50 {
                        let v = <Self as pg_crud::PgJson>::Cr::new(Some(el));
                        if !acc_ccd79a32.contains(&v) {
                            acc_ccd79a32.push(v);
                        }
                    }
                }
                {
                    let v = <Self as pg_crud::PgJson>::Cr::new(None);
                    if !acc_ccd79a32.contains(&v) {
                        acc_ccd79a32.push(v);
                    }
                }
                acc_ccd79a32
            })
        }
        fn rd_ids_to_2_dims_vec_rd_inn(
            rd_ids: &<Self::PgJson as pg_crud::PgJson>::RdIds,
        ) -> Vec<Vec<<Self::PgJson as pg_crud::PgJson>::RdInn>> {
            rd_ids . 0. v . as_ref () . into_iter () . flat_map (| v_5fa0668c | { < ObjExampleAsNnJsonbObj as pg_crud :: PgJsonTestCases > :: rd_ids_to_2_dims_vec_rd_inn (v_5fa0668c) . into_iter () . flat_map (| el0 | { el0 . into_iter () . map (| el1 | vec ! [Some (el1)]) }) }) . chain (std :: iter :: once (vec ! [None])) . collect ()
        }
        fn rd_inn_into_rd_with_new_or_try_new_unwraped(
            v: OptObjExampleAsNlJsonbObjRdInn,
        ) -> <Self::PgJson as pg_crud::PgJson>::Rd {
            < Self :: PgJson as pg_crud :: PgType > :: Rd :: new (v . map (< ObjExampleAsNnJsonbObj as pg_crud :: PgJsonTestCases > :: rd_inn_into_rd_with_new_or_try_new_unwraped))
        }
        fn rd_inn_into_upd_with_new_or_try_new_unwraped(
            v: OptObjExampleAsNlJsonbObjRdInn,
        ) -> <Self::PgJson as pg_crud::PgJson>::Upd {
            < Self :: PgJson as pg_crud :: PgType > :: Upd :: new (v . map (< ObjExampleAsNnJsonbObj as pg_crud :: PgTypeTestCases > :: rd_inn_into_upd_with_new_or_try_new_unwraped))
        }
        fn rd_ids_into_opt_v_rd_inn(
            v: <Self::PgJson as pg_crud::PgJson>::RdIds,
        ) -> Option<pg_crud::V<<Self::PgJson as pg_crud::PgJson>::RdInn>> {
            Some (pg_crud :: V { v : v . 0. v . and_then (| v_5d7e3961 | match < ObjExampleAsNnJsonbObj as pg_crud :: PgJsonTestCases > :: rd_ids_into_opt_v_rd_inn (v_5d7e3961) { Some (v_cfca0099) => Some (v_cfca0099 . v) , None => None , }) })
        }
        fn upd_to_rd_ids(
            v: &<Self::PgJson as pg_crud::PgJson>::Upd,
        ) -> <Self::PgJson as pg_crud::PgJson>::RdIds {
            OptObjExampleAsNlJsonbObjRdIds(pg_crud::V {
                v: v.0
                    .as_ref()
                    .map(<ObjExampleAsNnJsonbObj as pg_crud::PgJsonTestCases>::upd_to_rd_ids),
            })
        }
        fn rd_ids_to_opt_v_rd_dflt_some_one_el(
            v: &<Self::PgJson as pg_crud::PgJson>::RdIds,
        ) -> Option<pg_crud::V<<Self::PgJson as pg_crud::PgJson>::Rd>> {
            Some (pg_crud :: V { v : OptObjExampleAsNlJsonbObjRd :: new (v . 0. v . as_ref () . and_then (| v_dfa7815e | match < ObjExampleAsNnJsonbObj as pg_crud :: PgJsonTestCases > :: rd_ids_to_opt_v_rd_dflt_some_one_el (v_dfa7815e) { Some (v_02cef266) => Some (v_02cef266 . v) , None => None , })) })
        }
        fn previous_rd_and_opt_upd_into_rd(
            rd: <Self::PgJson as pg_crud::PgJson>::Rd,
            opt_upd: Option<<Self::PgJson as pg_crud::PgJson>::Upd>,
        ) -> <Self::PgJson as pg_crud::PgJson>::Rd {
            match opt_upd { Some (v_fca601b5) => OptObjExampleAsNlJsonbObjRd (match v_fca601b5 . 0 { Some (v_8d7747f1) => Some (< ObjExampleAsNnJsonbObj as pg_crud :: PgJsonTestCases > :: previous_rd_and_opt_upd_into_rd (rd . 0 . unwrap_or_else (pg_crud :: DfltSomeOneEl :: dflt_some_one_el) , Some (v_8d7747f1) ,)) , None => None , }) , None => rd , }
        }
        fn rd_ids_and_cr_into_rd(
            rd_ids: <Self::PgJson as pg_crud::PgJson>::RdIds,
            cr: <Self::PgJson as pg_crud::PgJson>::Cr,
        ) -> <Self::PgJson as pg_crud::PgJson>::Rd {
            OptObjExampleAsNlJsonbObjRd :: new (match (rd_ids . 0. v , cr . 0) { (Some (rd_ids_2b2ab8a1) , Some (cr_4a1adaa3)) => { Some (< ObjExampleAsNnJsonbObj as pg_crud :: PgJsonTestCases > :: rd_ids_and_cr_into_opt_v_rd (rd_ids_2b2ab8a1 , cr_4a1adaa3) . expect ("56ac4450") . v) } , (Some (_) , None) => panic ! ("75be9ae0") , (None , Some (_)) => panic ! ("6a95d7ae") , (None , None) => None , })
        }
        fn rd_ids_and_cr_into_opt_v_rd(
            rd_ids: <Self::PgJson as pg_crud::PgJson>::RdIds,
            cr: <Self::PgJson as pg_crud::PgJson>::Cr,
        ) -> Option<pg_crud::V<<Self::PgJson as pg_crud::PgJson>::Rd>> {
            Some(pg_crud::V {
                v: <Self as pg_crud::PgJsonTestCases>::rd_ids_and_cr_into_rd(rd_ids, cr),
            })
        }
        fn rd_ids_and_cr_into_tt(
            rd_ids: <Self::PgJson as pg_crud::PgJson>::RdIds,
            cr: <Self::PgJson as pg_crud::PgJson>::Cr,
        ) -> <Self::PgJson as pg_crud::PgJson>::Tt {
            OptObjExampleAsNlJsonbObjTt::new(match (rd_ids.0.v, cr.0) {
                (Some(rd_ids_fb2ec2e4), Some(cr_2f615d4f)) => Some(
                    <ObjExampleAsNnJsonbObj as pg_crud::PgJsonTestCases>::rd_ids_and_cr_into_tt(
                        rd_ids_fb2ec2e4,
                        cr_2f615d4f,
                    ),
                ),
                (Some(_), None) => panic!("9349dcd5"),
                (None, Some(_)) => panic!("45f8e70a"),
                (None, None) => None,
            })
        }
        fn rd_ids_and_cr_into_wh_eq(
            rd_ids: <Self::PgJson as pg_crud::PgJson>::RdIds,
            cr: <Self::PgJson as pg_crud::PgJson>::Cr,
        ) -> <Self::PgJson as pg_crud::PgJson>::Wh {
            pg_crud :: NlJsonObjPgTypeWhFlt (match (rd_ids . 0. v , cr . 0) { (Some (rd_ids_ce30c0fe) , Some (cr_8fd81ed8)) => match pg_crud :: NotEmptyUnqVec :: try_new (vec ! [< ObjExampleAsNnJsonbObj as pg_crud :: PgJsonTestCases > :: rd_ids_and_cr_into_wh_eq (rd_ids_ce30c0fe , cr_8fd81ed8)]) { Ok (v_7a9cd49b) => Some (v_7a9cd49b) , Err (er) => match er { pg_crud :: NotEmptyUnqVecTryNewEr :: IsEmpty { .. } => None , pg_crud :: NotEmptyUnqVecTryNewEr :: NotUnq { .. } => panic ! ("463769fc") } } , (Some (_) , None) => panic ! ("1a2b314c") , (None , Some (_)) => panic ! ("9faea0f9") , (None , None) => None , })
        }
        fn rd_ids_and_cr_into_vec_wh_eq_using_fields(
            rd_ids: <Self::PgJson as pg_crud::PgJson>::RdIds,
            cr: <Self::PgJson as pg_crud::PgJson>::Cr,
        ) -> pg_crud::NotEmptyUnqVec<<Self::PgJson as pg_crud::PgJson>::Wh> {
            pg_crud :: NotEmptyUnqVec :: try_new (vec ! [pg_crud :: NlJsonObjPgTypeWhFlt (match (rd_ids . 0. v , cr . 0) { (Some (rd_ids_2898c440) , Some (cr_f1c4667c)) => Some (< ObjExampleAsNnJsonbObj as pg_crud :: PgJsonTestCases > :: rd_ids_and_cr_into_vec_wh_eq_using_fields (rd_ids_2898c440 , cr_f1c4667c)) , (Some (_) , None) => panic ! ("49e4c289") , (None , Some (_)) => panic ! ("ad71caa2") , (None , None) => None , })]) . expect ("ba9c52c1")
        }
        fn rd_ids_and_cr_into_vec_wh_eq_to_json_field(
            rd_ids: <Self::PgJson as pg_crud::PgJson>::RdIds,
            cr: <Self::PgJson as pg_crud::PgJson>::Cr,
        ) -> pg_crud::NotEmptyUnqVec<<Self::PgJson as pg_crud::PgJson>::Wh> {
            pg_crud :: NotEmptyUnqVec :: try_new ({ let mut acc_12b6f16d = Vec :: new () ; match (rd_ids . 0. v , cr . 0) { (Some (rd_ids_2f024927) , Some (cr_120c1dad)) => { for el_a8b181a0 in < ObjExampleAsNnJsonbObj as pg_crud :: PgJsonTestCases > :: rd_ids_and_cr_into_vec_wh_eq_to_json_field (rd_ids_2f024927 , cr_120c1dad) . into_vec () { match pg_crud :: NotEmptyUnqVec :: try_new (vec ! [el_a8b181a0]) { Ok (v_8e72cfd7) => { acc_12b6f16d . push (pg_crud :: NlJsonObjPgTypeWhFlt (Some (v_8e72cfd7))) ; } , Err (er) => match er { pg_crud :: NotEmptyUnqVecTryNewEr :: IsEmpty { .. } => () , pg_crud :: NotEmptyUnqVecTryNewEr :: NotUnq { .. } => panic ! ("2a88b17f") } } } } , (Some (_) , None) => panic ! ("b4507b4c") , (None , Some (_)) => panic ! ("8f458c1d") , (None , None) => { acc_12b6f16d . push (pg_crud :: NlJsonObjPgTypeWhFlt (None)) ; } , } acc_12b6f16d }) . expect ("7efc9aae")
        }
        fn rd_ids_and_cr_into_pg_json_opt_vec_wh_dim_one_eq(
            rd_ids: <Self::PgJson as pg_crud::PgJson>::RdIds,
            cr: <Self::PgJson as pg_crud::PgJson>::Cr,
        ) -> Option<pg_crud::NotEmptyUnqVec<<Self::PgJson as pg_crud::PgJson>::Wh>> {
            match pg_crud :: NotEmptyUnqVec :: try_new (match (rd_ids . 0. v , cr . 0) { (Some (rd_ids_cdcb6239) , Some (cr_fdd53941)) => match < ObjExampleAsNnJsonbObj as pg_crud :: PgJsonTestCases > :: rd_ids_and_cr_into_pg_json_opt_vec_wh_dim_one_eq (rd_ids_cdcb6239 , cr_fdd53941) { Some (v_d6124e21) => { let mut acc_bd78dc08 = Vec :: new () ; for el in v_d6124e21 . clone () . into_vec () { match pg_crud :: NotEmptyUnqVec :: try_new (vec ! [el]) { Ok (v_7ed84f3b) => { acc_bd78dc08 . push (pg_crud :: NlJsonObjPgTypeWhFlt (Some (v_7ed84f3b))) ; } , Err (er) => match er { pg_crud :: NotEmptyUnqVecTryNewEr :: IsEmpty { .. } => () , pg_crud :: NotEmptyUnqVecTryNewEr :: NotUnq { .. } => panic ! ("23dca12f") } } } let v_e48110ec = pg_crud :: NlJsonObjPgTypeWhFlt (Some (v_d6124e21)) ; if ! acc_bd78dc08 . contains (& v_e48110ec) { acc_bd78dc08 . push (v_e48110ec) ; } acc_bd78dc08 } , None => { return None ; } } , (Some (_) , None) => panic ! ("6abeac7b") , (None , Some (_)) => panic ! ("a2761cd2") , (None , None) => vec ! [pg_crud :: NlJsonObjPgTypeWhFlt (None)] }) { Ok (v_55f2dc3d) => Some (v_55f2dc3d) , Err (er) => match er { pg_crud :: NotEmptyUnqVecTryNewEr :: IsEmpty { .. } => None , pg_crud :: NotEmptyUnqVecTryNewEr :: NotUnq { .. } => panic ! ("88912e24") } }
        }
        fn rd_ids_and_cr_into_pg_json_opt_vec_wh_dim_two_eq(
            rd_ids: <Self::PgJson as pg_crud::PgJson>::RdIds,
            cr: <Self::PgJson as pg_crud::PgJson>::Cr,
        ) -> Option<pg_crud::NotEmptyUnqVec<<Self::PgJson as pg_crud::PgJson>::Wh>> {
            match pg_crud :: NotEmptyUnqVec :: try_new (match (rd_ids . 0. v , cr . 0) { (Some (rd_ids_cdcb6239) , Some (cr_fdd53941)) => match < ObjExampleAsNnJsonbObj as pg_crud :: PgJsonTestCases > :: rd_ids_and_cr_into_pg_json_opt_vec_wh_dim_two_eq (rd_ids_cdcb6239 , cr_fdd53941) { Some (v_d6124e21) => { let mut acc_bd78dc08 = Vec :: new () ; for el in v_d6124e21 . clone () . into_vec () { match pg_crud :: NotEmptyUnqVec :: try_new (vec ! [el]) { Ok (v_7ed84f3b) => { acc_bd78dc08 . push (pg_crud :: NlJsonObjPgTypeWhFlt (Some (v_7ed84f3b))) ; } , Err (er) => match er { pg_crud :: NotEmptyUnqVecTryNewEr :: IsEmpty { .. } => () , pg_crud :: NotEmptyUnqVecTryNewEr :: NotUnq { .. } => panic ! ("23dca12f") } } } let v_e48110ec = pg_crud :: NlJsonObjPgTypeWhFlt (Some (v_d6124e21)) ; if ! acc_bd78dc08 . contains (& v_e48110ec) { acc_bd78dc08 . push (v_e48110ec) ; } acc_bd78dc08 } , None => { return None ; } } , (Some (_) , None) => panic ! ("6abeac7b") , (None , Some (_)) => panic ! ("a2761cd2") , (None , None) => vec ! [pg_crud :: NlJsonObjPgTypeWhFlt (None)] }) { Ok (v_55f2dc3d) => Some (v_55f2dc3d) , Err (er) => match er { pg_crud :: NotEmptyUnqVecTryNewEr :: IsEmpty { .. } => None , pg_crud :: NotEmptyUnqVecTryNewEr :: NotUnq { .. } => panic ! ("88912e24") } }
        }
        fn rd_ids_and_cr_into_pg_json_opt_vec_wh_dim_three_eq(
            rd_ids: <Self::PgJson as pg_crud::PgJson>::RdIds,
            cr: <Self::PgJson as pg_crud::PgJson>::Cr,
        ) -> Option<pg_crud::NotEmptyUnqVec<<Self::PgJson as pg_crud::PgJson>::Wh>> {
            match pg_crud :: NotEmptyUnqVec :: try_new (match (rd_ids . 0. v , cr . 0) { (Some (rd_ids_cdcb6239) , Some (cr_fdd53941)) => match < ObjExampleAsNnJsonbObj as pg_crud :: PgJsonTestCases > :: rd_ids_and_cr_into_pg_json_opt_vec_wh_dim_three_eq (rd_ids_cdcb6239 , cr_fdd53941) { Some (v_d6124e21) => { let mut acc_bd78dc08 = Vec :: new () ; for el in v_d6124e21 . clone () . into_vec () { match pg_crud :: NotEmptyUnqVec :: try_new (vec ! [el]) { Ok (v_7ed84f3b) => { acc_bd78dc08 . push (pg_crud :: NlJsonObjPgTypeWhFlt (Some (v_7ed84f3b))) ; } , Err (er) => match er { pg_crud :: NotEmptyUnqVecTryNewEr :: IsEmpty { .. } => () , pg_crud :: NotEmptyUnqVecTryNewEr :: NotUnq { .. } => panic ! ("23dca12f") } } } let v_e48110ec = pg_crud :: NlJsonObjPgTypeWhFlt (Some (v_d6124e21)) ; if ! acc_bd78dc08 . contains (& v_e48110ec) { acc_bd78dc08 . push (v_e48110ec) ; } acc_bd78dc08 } , None => { return None ; } } , (Some (_) , None) => panic ! ("6abeac7b") , (None , Some (_)) => panic ! ("a2761cd2") , (None , None) => vec ! [pg_crud :: NlJsonObjPgTypeWhFlt (None)] }) { Ok (v_55f2dc3d) => Some (v_55f2dc3d) , Err (er) => match er { pg_crud :: NotEmptyUnqVecTryNewEr :: IsEmpty { .. } => None , pg_crud :: NotEmptyUnqVecTryNewEr :: NotUnq { .. } => panic ! ("88912e24") } }
        }
        fn rd_ids_and_cr_into_pg_json_opt_vec_wh_dim_four_eq(
            rd_ids: <Self::PgJson as pg_crud::PgJson>::RdIds,
            cr: <Self::PgJson as pg_crud::PgJson>::Cr,
        ) -> Option<pg_crud::NotEmptyUnqVec<<Self::PgJson as pg_crud::PgJson>::Wh>> {
            match pg_crud :: NotEmptyUnqVec :: try_new (match (rd_ids . 0. v , cr . 0) { (Some (rd_ids_cdcb6239) , Some (cr_fdd53941)) => match < ObjExampleAsNnJsonbObj as pg_crud :: PgJsonTestCases > :: rd_ids_and_cr_into_pg_json_opt_vec_wh_dim_four_eq (rd_ids_cdcb6239 , cr_fdd53941) { Some (v_d6124e21) => { let mut acc_bd78dc08 = Vec :: new () ; for el in v_d6124e21 . clone () . into_vec () { match pg_crud :: NotEmptyUnqVec :: try_new (vec ! [el]) { Ok (v_7ed84f3b) => { acc_bd78dc08 . push (pg_crud :: NlJsonObjPgTypeWhFlt (Some (v_7ed84f3b))) ; } , Err (er) => match er { pg_crud :: NotEmptyUnqVecTryNewEr :: IsEmpty { .. } => () , pg_crud :: NotEmptyUnqVecTryNewEr :: NotUnq { .. } => panic ! ("23dca12f") } } } let v_e48110ec = pg_crud :: NlJsonObjPgTypeWhFlt (Some (v_d6124e21)) ; if ! acc_bd78dc08 . contains (& v_e48110ec) { acc_bd78dc08 . push (v_e48110ec) ; } acc_bd78dc08 } , None => { return None ; } } , (Some (_) , None) => panic ! ("6abeac7b") , (None , Some (_)) => panic ! ("a2761cd2") , (None , None) => vec ! [pg_crud :: NlJsonObjPgTypeWhFlt (None)] }) { Ok (v_55f2dc3d) => Some (v_55f2dc3d) , Err (er) => match er { pg_crud :: NotEmptyUnqVecTryNewEr :: IsEmpty { .. } => None , pg_crud :: NotEmptyUnqVecTryNewEr :: NotUnq { .. } => panic ! ("88912e24") } }
        }
        fn cr_into_pg_json_opt_vec_wh_len_eq(
            cr: <Self::PgJson as pg_crud::PgJson>::Cr,
        ) -> Option<pg_crud::NotEmptyUnqVec<<Self::PgJson as pg_crud::PgJson>::Wh>> {
            match pg_crud :: NotEmptyUnqVec :: try_new (match cr . 0 { Some (cr_09a81dae) => match < ObjExampleAsNnJsonbObj as pg_crud :: PgJsonTestCases > :: cr_into_pg_json_opt_vec_wh_len_eq (cr_09a81dae) { Some (v_3680a4c9) => { let mut acc_5c441d3a = Vec :: new () ; for el_a8b181a0 in v_3680a4c9 . clone () . into_vec () { match pg_crud :: NotEmptyUnqVec :: try_new (vec ! [el_a8b181a0]) { Ok (v_15097b27) => { acc_5c441d3a . push (pg_crud :: NlJsonObjPgTypeWhFlt (Some (v_15097b27))) ; } , Err (er) => match er { pg_crud :: NotEmptyUnqVecTryNewEr :: IsEmpty { .. } => () , pg_crud :: NotEmptyUnqVecTryNewEr :: NotUnq { .. } => panic ! ("6c4da72e") } } } let v_84ea8e4c = pg_crud :: NlJsonObjPgTypeWhFlt (Some (v_3680a4c9)) ; if ! acc_5c441d3a . contains (& v_84ea8e4c) { acc_5c441d3a . push (v_84ea8e4c) ; } acc_5c441d3a } , None => { return None ; } } , None => vec ! [pg_crud :: NlJsonObjPgTypeWhFlt (None)] , }) { Ok (v_72dbefbc) => Some (v_72dbefbc) , Err (er) => match er { pg_crud :: NotEmptyUnqVecTryNewEr :: IsEmpty { .. } => None , pg_crud :: NotEmptyUnqVecTryNewEr :: NotUnq { .. } => panic ! ("d41bcbca") } }
        }
        fn cr_into_pg_json_opt_vec_wh_len_greater_than(
            cr: <Self::PgJson as pg_crud::PgJson>::Cr,
        ) -> Option<pg_crud::NotEmptyUnqVec<<Self::PgJson as pg_crud::PgJson>::Wh>> {
            cr . 0 . map_or_else (|| None , | cr_612f2a61 | < ObjExampleAsNnJsonbObj as pg_crud :: PgJsonTestCases > :: cr_into_pg_json_opt_vec_wh_len_greater_than (cr_612f2a61) . map_or_else (|| None , | v_1ea95b5d | match pg_crud :: NotEmptyUnqVec :: try_new ({ let mut acc_87f84b5c = Vec :: new () ; for el_9bbf8527 in v_1ea95b5d . clone () . into_vec () { match pg_crud :: NotEmptyUnqVec :: try_new (vec ! [el_9bbf8527]) { Ok (v_1d0202fc) => { acc_87f84b5c . push (pg_crud :: NlJsonObjPgTypeWhFlt (Some (v_1d0202fc))) ; } Err (er) => match er { pg_crud :: NotEmptyUnqVecTryNewEr :: IsEmpty { .. } => () , pg_crud :: NotEmptyUnqVecTryNewEr :: NotUnq { .. } => panic ! ("bdb0a112") , } , } } let v_4e4cfda3 = pg_crud :: NlJsonObjPgTypeWhFlt (Some (v_1ea95b5d)) ; if ! acc_87f84b5c . contains (& v_4e4cfda3) { acc_87f84b5c . push (v_4e4cfda3) ; } acc_87f84b5c }) { Ok (v_ea4ca151) => Some (v_ea4ca151) , Err (er) => match er { pg_crud :: NotEmptyUnqVecTryNewEr :: IsEmpty { .. } => None , pg_crud :: NotEmptyUnqVecTryNewEr :: NotUnq { .. } => panic ! ("c7ecc36f") , } , } ,))
        }
        fn rd_ids_and_cr_into_pg_json_opt_vec_wh_greater_than(
            rd_ids: <Self::PgJson as pg_crud::PgJson>::RdIds,
            cr: <Self::PgJson as pg_crud::PgJson>::Cr,
        ) -> Option<
            pg_crud::NotEmptyUnqVec<
                pg_crud::SingleOrMultiple<<Self::PgJson as pg_crud::PgJson>::Wh>,
            >,
        > {
            match (rd_ids . 0. v , cr . 0) { (Some (rd_ids_3e2e30c8) , Some (cr_79039a2f)) => < ObjExampleAsNnJsonbObj as pg_crud :: PgJsonTestCases > :: rd_ids_and_cr_into_pg_json_opt_vec_wh_greater_than (rd_ids_3e2e30c8 , cr_79039a2f) . map_or_else (|| None , | v_35662b3a | match pg_crud :: NotEmptyUnqVec :: try_new ({ let mut acc_e0d72451 = vec ! [] ; for el in v_35662b3a . into_vec () { match el { pg_crud :: SingleOrMultiple :: Multiple (multiple) => { acc_e0d72451 . push (pg_crud :: SingleOrMultiple :: Single (pg_crud :: NlJsonObjPgTypeWhFlt (Some (multiple)))) ; } , pg_crud :: SingleOrMultiple :: Single (single) => match pg_crud :: NotEmptyUnqVec :: try_new (vec ! [single]) { Ok (v_4ce6ecd3) => { acc_e0d72451 . push (pg_crud :: SingleOrMultiple :: Single (pg_crud :: NlJsonObjPgTypeWhFlt (Some (v_4ce6ecd3)))) ; } Err (er) => match er { pg_crud :: NotEmptyUnqVecTryNewEr :: IsEmpty { .. } => () , pg_crud :: NotEmptyUnqVecTryNewEr :: NotUnq { .. } => panic ! ("626ffa77") , } , } , } } acc_e0d72451 }) { Ok (v_5d381053) => Some (v_5d381053) , Err (er) => match er { pg_crud :: NotEmptyUnqVecTryNewEr :: IsEmpty { .. } => None , pg_crud :: NotEmptyUnqVecTryNewEr :: NotUnq { .. } => panic ! ("23a17416") , } , }) , (Some (_) , None) => panic ! ("994082bf") , (None , Some (_)) => panic ! ("04f4d016") , (None , None) => None , }
        }
        fn rd_ids_and_cr_into_pg_json_opt_vec_wh_btwn(
            rd_ids: <Self::PgJson as pg_crud::PgJson>::RdIds,
            cr: <Self::PgJson as pg_crud::PgJson>::Cr,
        ) -> Option<
            pg_crud::NotEmptyUnqVec<
                pg_crud::SingleOrMultiple<<Self::PgJson as pg_crud::PgJson>::Wh>,
            >,
        > {
            match (rd_ids . 0. v , cr . 0) { (Some (rd_ids_3e2e30c8) , Some (cr_79039a2f)) => < ObjExampleAsNnJsonbObj as pg_crud :: PgJsonTestCases > :: rd_ids_and_cr_into_pg_json_opt_vec_wh_btwn (rd_ids_3e2e30c8 , cr_79039a2f) . map_or_else (|| None , | v_35662b3a | match pg_crud :: NotEmptyUnqVec :: try_new ({ let mut acc_e0d72451 = vec ! [] ; for el in v_35662b3a . into_vec () { match el { pg_crud :: SingleOrMultiple :: Multiple (multiple) => { acc_e0d72451 . push (pg_crud :: SingleOrMultiple :: Single (pg_crud :: NlJsonObjPgTypeWhFlt (Some (multiple)))) ; } , pg_crud :: SingleOrMultiple :: Single (single) => match pg_crud :: NotEmptyUnqVec :: try_new (vec ! [single]) { Ok (v_4ce6ecd3) => { acc_e0d72451 . push (pg_crud :: SingleOrMultiple :: Single (pg_crud :: NlJsonObjPgTypeWhFlt (Some (v_4ce6ecd3)))) ; } Err (er) => match er { pg_crud :: NotEmptyUnqVecTryNewEr :: IsEmpty { .. } => () , pg_crud :: NotEmptyUnqVecTryNewEr :: NotUnq { .. } => panic ! ("626ffa77") , } , } , } } acc_e0d72451 }) { Ok (v_5d381053) => Some (v_5d381053) , Err (er) => match er { pg_crud :: NotEmptyUnqVecTryNewEr :: IsEmpty { .. } => None , pg_crud :: NotEmptyUnqVecTryNewEr :: NotUnq { .. } => panic ! ("23a17416") , } , }) , (Some (_) , None) => panic ! ("994082bf") , (None , Some (_)) => panic ! ("04f4d016") , (None , None) => None , }
        }
        fn rd_ids_and_cr_into_pg_json_opt_vec_wh_in(
            rd_ids: <Self::PgJson as pg_crud::PgJson>::RdIds,
            cr: <Self::PgJson as pg_crud::PgJson>::Cr,
        ) -> Option<
            pg_crud::NotEmptyUnqVec<
                pg_crud::SingleOrMultiple<<Self::PgJson as pg_crud::PgJson>::Wh>,
            >,
        > {
            match (rd_ids . 0. v , cr . 0) { (Some (rd_ids_3e2e30c8) , Some (cr_79039a2f)) => < ObjExampleAsNnJsonbObj as pg_crud :: PgJsonTestCases > :: rd_ids_and_cr_into_pg_json_opt_vec_wh_in (rd_ids_3e2e30c8 , cr_79039a2f) . map_or_else (|| None , | v_35662b3a | match pg_crud :: NotEmptyUnqVec :: try_new ({ let mut acc_e0d72451 = vec ! [] ; for el in v_35662b3a . into_vec () { match el { pg_crud :: SingleOrMultiple :: Multiple (multiple) => { acc_e0d72451 . push (pg_crud :: SingleOrMultiple :: Single (pg_crud :: NlJsonObjPgTypeWhFlt (Some (multiple)))) ; } , pg_crud :: SingleOrMultiple :: Single (single) => match pg_crud :: NotEmptyUnqVec :: try_new (vec ! [single]) { Ok (v_4ce6ecd3) => { acc_e0d72451 . push (pg_crud :: SingleOrMultiple :: Single (pg_crud :: NlJsonObjPgTypeWhFlt (Some (v_4ce6ecd3)))) ; } Err (er) => match er { pg_crud :: NotEmptyUnqVecTryNewEr :: IsEmpty { .. } => () , pg_crud :: NotEmptyUnqVecTryNewEr :: NotUnq { .. } => panic ! ("626ffa77") , } , } , } } acc_e0d72451 }) { Ok (v_5d381053) => Some (v_5d381053) , Err (er) => match er { pg_crud :: NotEmptyUnqVecTryNewEr :: IsEmpty { .. } => None , pg_crud :: NotEmptyUnqVecTryNewEr :: NotUnq { .. } => panic ! ("23a17416") , } , }) , (Some (_) , None) => panic ! ("994082bf") , (None , Some (_)) => panic ! ("04f4d016") , (None , None) => None , }
        }
        fn rd_ids_and_cr_into_pg_json_opt_vec_wh_rgx(
            rd_ids: <Self::PgJson as pg_crud::PgJson>::RdIds,
            cr: <Self::PgJson as pg_crud::PgJson>::Cr,
        ) -> Option<
            pg_crud::NotEmptyUnqVec<
                pg_crud::SingleOrMultiple<<Self::PgJson as pg_crud::PgJson>::Wh>,
            >,
        > {
            match (rd_ids . 0. v , cr . 0) { (Some (rd_ids_3e2e30c8) , Some (cr_79039a2f)) => < ObjExampleAsNnJsonbObj as pg_crud :: PgJsonTestCases > :: rd_ids_and_cr_into_pg_json_opt_vec_wh_rgx (rd_ids_3e2e30c8 , cr_79039a2f) . map_or_else (|| None , | v_35662b3a | match pg_crud :: NotEmptyUnqVec :: try_new ({ let mut acc_e0d72451 = vec ! [] ; for el in v_35662b3a . into_vec () { match el { pg_crud :: SingleOrMultiple :: Multiple (multiple) => { acc_e0d72451 . push (pg_crud :: SingleOrMultiple :: Single (pg_crud :: NlJsonObjPgTypeWhFlt (Some (multiple)))) ; } , pg_crud :: SingleOrMultiple :: Single (single) => match pg_crud :: NotEmptyUnqVec :: try_new (vec ! [single]) { Ok (v_4ce6ecd3) => { acc_e0d72451 . push (pg_crud :: SingleOrMultiple :: Single (pg_crud :: NlJsonObjPgTypeWhFlt (Some (v_4ce6ecd3)))) ; } Err (er) => match er { pg_crud :: NotEmptyUnqVecTryNewEr :: IsEmpty { .. } => () , pg_crud :: NotEmptyUnqVecTryNewEr :: NotUnq { .. } => panic ! ("626ffa77") , } , } , } } acc_e0d72451 }) { Ok (v_5d381053) => Some (v_5d381053) , Err (er) => match er { pg_crud :: NotEmptyUnqVecTryNewEr :: IsEmpty { .. } => None , pg_crud :: NotEmptyUnqVecTryNewEr :: NotUnq { .. } => panic ! ("23a17416") , } , }) , (Some (_) , None) => panic ! ("994082bf") , (None , Some (_)) => panic ! ("04f4d016") , (None , None) => None , }
        }
        fn rd_ids_and_cr_into_pg_json_opt_vec_wh_contains_el_greater_than(
            rd_ids: <Self::PgJson as pg_crud::PgJson>::RdIds,
            cr: <Self::PgJson as pg_crud::PgJson>::Cr,
        ) -> Option<
            pg_crud::NotEmptyUnqVec<
                pg_crud::SingleOrMultiple<<Self::PgJson as pg_crud::PgJson>::Wh>,
            >,
        > {
            match (rd_ids . 0. v , cr . 0) { (Some (rd_ids_3e2e30c8) , Some (cr_79039a2f)) => < ObjExampleAsNnJsonbObj as pg_crud :: PgJsonTestCases > :: rd_ids_and_cr_into_pg_json_opt_vec_wh_contains_el_greater_than (rd_ids_3e2e30c8 , cr_79039a2f) . map_or_else (|| None , | v_35662b3a | match pg_crud :: NotEmptyUnqVec :: try_new ({ let mut acc_e0d72451 = vec ! [] ; for el in v_35662b3a . into_vec () { match el { pg_crud :: SingleOrMultiple :: Multiple (multiple) => { acc_e0d72451 . push (pg_crud :: SingleOrMultiple :: Single (pg_crud :: NlJsonObjPgTypeWhFlt (Some (multiple)))) ; } , pg_crud :: SingleOrMultiple :: Single (single) => match pg_crud :: NotEmptyUnqVec :: try_new (vec ! [single]) { Ok (v_4ce6ecd3) => { acc_e0d72451 . push (pg_crud :: SingleOrMultiple :: Single (pg_crud :: NlJsonObjPgTypeWhFlt (Some (v_4ce6ecd3)))) ; } Err (er) => match er { pg_crud :: NotEmptyUnqVecTryNewEr :: IsEmpty { .. } => () , pg_crud :: NotEmptyUnqVecTryNewEr :: NotUnq { .. } => panic ! ("626ffa77") , } , } , } } acc_e0d72451 }) { Ok (v_5d381053) => Some (v_5d381053) , Err (er) => match er { pg_crud :: NotEmptyUnqVecTryNewEr :: IsEmpty { .. } => None , pg_crud :: NotEmptyUnqVecTryNewEr :: NotUnq { .. } => panic ! ("23a17416") , } , }) , (Some (_) , None) => panic ! ("994082bf") , (None , Some (_)) => panic ! ("04f4d016") , (None , None) => None , }
        }
        fn rd_ids_and_cr_into_pg_json_opt_vec_wh_contains_el_rgx(
            rd_ids: <Self::PgJson as pg_crud::PgJson>::RdIds,
            cr: <Self::PgJson as pg_crud::PgJson>::Cr,
        ) -> Option<
            pg_crud::NotEmptyUnqVec<
                pg_crud::SingleOrMultiple<<Self::PgJson as pg_crud::PgJson>::Wh>,
            >,
        > {
            match (rd_ids . 0. v , cr . 0) { (Some (rd_ids_3e2e30c8) , Some (cr_79039a2f)) => < ObjExampleAsNnJsonbObj as pg_crud :: PgJsonTestCases > :: rd_ids_and_cr_into_pg_json_opt_vec_wh_contains_el_rgx (rd_ids_3e2e30c8 , cr_79039a2f) . map_or_else (|| None , | v_35662b3a | match pg_crud :: NotEmptyUnqVec :: try_new ({ let mut acc_e0d72451 = vec ! [] ; for el in v_35662b3a . into_vec () { match el { pg_crud :: SingleOrMultiple :: Multiple (multiple) => { acc_e0d72451 . push (pg_crud :: SingleOrMultiple :: Single (pg_crud :: NlJsonObjPgTypeWhFlt (Some (multiple)))) ; } , pg_crud :: SingleOrMultiple :: Single (single) => match pg_crud :: NotEmptyUnqVec :: try_new (vec ! [single]) { Ok (v_4ce6ecd3) => { acc_e0d72451 . push (pg_crud :: SingleOrMultiple :: Single (pg_crud :: NlJsonObjPgTypeWhFlt (Some (v_4ce6ecd3)))) ; } Err (er) => match er { pg_crud :: NotEmptyUnqVecTryNewEr :: IsEmpty { .. } => () , pg_crud :: NotEmptyUnqVecTryNewEr :: NotUnq { .. } => panic ! ("626ffa77") , } , } , } } acc_e0d72451 }) { Ok (v_5d381053) => Some (v_5d381053) , Err (er) => match er { pg_crud :: NotEmptyUnqVecTryNewEr :: IsEmpty { .. } => None , pg_crud :: NotEmptyUnqVecTryNewEr :: NotUnq { .. } => panic ! ("23a17416") , } , }) , (Some (_) , None) => panic ! ("994082bf") , (None , Some (_)) => panic ! ("04f4d016") , (None , None) => None , }
        }
    }
    #[allow(unused_qualifications)]
    #[allow(clippy::absolute_paths)]
    #[allow(clippy::arbitrary_source_item_ordering)]
    #[cfg(feature = "test-utils")]
    #[allow(clippy::float_arithmetic)]
    impl pg_crud::PgTypeTestCases for OptObjExampleAsNlJsonbObj {
        type PgType = Self;
        type Sel = OptObjExampleAsNlJsonbObjSel;
        fn opt_vec_cr() -> Option<Vec<<Self::PgType as pg_crud::PgType>::Cr>> {
            <Self as pg_crud::PgJsonTestCases>::opt_vec_cr()
        }
        fn rd_ids_to_2_dims_vec_rd_inn(
            rd_ids: &<Self::PgType as pg_crud::PgType>::RdIds,
        ) -> Vec<Vec<<Self::PgType as pg_crud::PgType>::RdInn>> {
            <Self as pg_crud::PgJsonTestCases>::rd_ids_to_2_dims_vec_rd_inn(rd_ids)
        }
        fn rd_inn_into_rd_with_new_or_try_new_unwraped(
            v: OptObjExampleAsNlJsonbObjRdInn,
        ) -> <Self::PgType as pg_crud::PgType>::Rd {
            <Self as pg_crud::PgJsonTestCases>::rd_inn_into_rd_with_new_or_try_new_unwraped(v)
        }
        fn rd_inn_into_upd_with_new_or_try_new_unwraped(
            v: OptObjExampleAsNlJsonbObjRdInn,
        ) -> <Self::PgType as pg_crud::PgType>::Upd {
            <Self as pg_crud::PgJsonTestCases>::rd_inn_into_upd_with_new_or_try_new_unwraped(v)
        }
        fn upd_to_rd_ids(
            v: &<Self::PgType as pg_crud::PgType>::Upd,
        ) -> <Self::PgType as pg_crud::PgType>::RdIds {
            <Self as pg_crud::PgJsonTestCases>::upd_to_rd_ids(v)
        }
        fn rd_ids_to_opt_v_rd_dflt_some_one_el(
            v: &<Self::PgType as pg_crud::PgType>::RdIds,
        ) -> Option<pg_crud::V<<Self::PgType as pg_crud::PgType>::Rd>> {
            <Self as pg_crud::PgJsonTestCases>::rd_ids_to_opt_v_rd_dflt_some_one_el(v)
        }
        fn previous_rd_and_opt_upd_into_rd(
            rd: <Self::PgType as pg_crud::PgType>::Rd,
            opt_upd: Option<<Self::PgType as pg_crud::PgType>::Upd>,
        ) -> <Self::PgType as pg_crud::PgType>::Rd {
            <Self as pg_crud::PgJsonTestCases>::previous_rd_and_opt_upd_into_rd(rd, opt_upd)
        }
        fn rd_ids_and_cr_into_rd(
            rd_ids: <Self::PgType as pg_crud::PgType>::RdIds,
            cr: <Self::PgType as pg_crud::PgType>::Cr,
        ) -> <Self::PgType as pg_crud::PgType>::Rd {
            <Self as pg_crud::PgJsonTestCases>::rd_ids_and_cr_into_rd(rd_ids, cr)
        }
        fn rd_ids_and_cr_into_opt_v_rd(
            rd_ids: <Self::PgType as pg_crud::PgType>::RdIds,
            cr: <Self::PgType as pg_crud::PgType>::Cr,
        ) -> Option<pg_crud::V<<Self::PgType as pg_crud::PgType>::Rd>> {
            <Self as pg_crud::PgJsonTestCases>::rd_ids_and_cr_into_opt_v_rd(rd_ids, cr)
        }
        fn rd_ids_and_cr_into_tt(
            rd_ids: <Self::PgType as pg_crud::PgType>::RdIds,
            cr: <Self::PgType as pg_crud::PgType>::Cr,
        ) -> <Self::PgType as pg_crud::PgType>::Tt {
            <Self as pg_crud::PgJsonTestCases>::rd_ids_and_cr_into_tt(rd_ids, cr)
        }
        fn rd_ids_and_cr_into_wh_eq(
            rd_ids: <Self::PgType as pg_crud::PgType>::RdIds,
            cr: <Self::PgType as pg_crud::PgType>::Cr,
        ) -> <Self::PgType as pg_crud::PgType>::Wh {
            <Self as pg_crud::PgJsonTestCases>::rd_ids_and_cr_into_wh_eq(rd_ids, cr)
        }
        fn rd_ids_and_cr_into_vec_wh_eq_using_fields(
            rd_ids: <Self::PgType as pg_crud::PgType>::RdIds,
            cr: <Self::PgType as pg_crud::PgType>::Cr,
        ) -> pg_crud::NotEmptyUnqVec<<Self::PgType as pg_crud::PgType>::Wh> {
            <Self as pg_crud::PgJsonTestCases>::rd_ids_and_cr_into_vec_wh_eq_using_fields(
                rd_ids, cr,
            )
        }
        fn rd_ids_and_cr_into_opt_vec_wh_eq_to_json_field(
            rd_ids: <Self::PgType as pg_crud::PgType>::RdIds,
            cr: <Self::PgType as pg_crud::PgType>::Cr,
        ) -> Option<pg_crud::NotEmptyUnqVec<<Self::PgType as pg_crud::PgType>::Wh>> {
            Some(
                <Self as pg_crud::PgJsonTestCases>::rd_ids_and_cr_into_vec_wh_eq_to_json_field(
                    rd_ids, cr,
                ),
            )
        }
        fn rd_ids_and_cr_into_pg_json_opt_vec_wh_dim_one_eq(
            rd_ids: <Self::PgType as pg_crud::PgType>::RdIds,
            cr: <Self::PgType as pg_crud::PgType>::Cr,
        ) -> Option<pg_crud::NotEmptyUnqVec<<Self::PgType as pg_crud::PgType>::Wh>> {
            <Self as pg_crud::PgJsonTestCases>::rd_ids_and_cr_into_pg_json_opt_vec_wh_dim_one_eq(
                rd_ids, cr,
            )
        }
        fn rd_ids_and_cr_into_pg_json_opt_vec_wh_dim_two_eq(
            rd_ids: <Self::PgType as pg_crud::PgType>::RdIds,
            cr: <Self::PgType as pg_crud::PgType>::Cr,
        ) -> Option<pg_crud::NotEmptyUnqVec<<Self::PgType as pg_crud::PgType>::Wh>> {
            <Self as pg_crud::PgJsonTestCases>::rd_ids_and_cr_into_pg_json_opt_vec_wh_dim_two_eq(
                rd_ids, cr,
            )
        }
        fn rd_ids_and_cr_into_pg_json_opt_vec_wh_dim_three_eq(
            rd_ids: <Self::PgType as pg_crud::PgType>::RdIds,
            cr: <Self::PgType as pg_crud::PgType>::Cr,
        ) -> Option<pg_crud::NotEmptyUnqVec<<Self::PgType as pg_crud::PgType>::Wh>> {
            <Self as pg_crud::PgJsonTestCases>::rd_ids_and_cr_into_pg_json_opt_vec_wh_dim_three_eq(
                rd_ids, cr,
            )
        }
        fn rd_ids_and_cr_into_pg_json_opt_vec_wh_dim_four_eq(
            rd_ids: <Self::PgType as pg_crud::PgType>::RdIds,
            cr: <Self::PgType as pg_crud::PgType>::Cr,
        ) -> Option<pg_crud::NotEmptyUnqVec<<Self::PgType as pg_crud::PgType>::Wh>> {
            <Self as pg_crud::PgJsonTestCases>::rd_ids_and_cr_into_pg_json_opt_vec_wh_dim_four_eq(
                rd_ids, cr,
            )
        }
        fn cr_into_pg_json_opt_vec_wh_len_eq(
            cr: <Self::PgType as pg_crud::PgType>::Cr,
        ) -> Option<pg_crud::NotEmptyUnqVec<<Self::PgType as pg_crud::PgType>::Wh>> {
            <Self as pg_crud::PgJsonTestCases>::cr_into_pg_json_opt_vec_wh_len_eq(cr)
        }
        fn cr_into_pg_json_opt_vec_wh_len_greater_than(
            cr: <Self::PgType as pg_crud::PgType>::Cr,
        ) -> Option<pg_crud::NotEmptyUnqVec<<Self::PgType as pg_crud::PgType>::Wh>> {
            <Self as pg_crud::PgJsonTestCases>::cr_into_pg_json_opt_vec_wh_len_greater_than(cr)
        }
        fn rd_ids_and_cr_into_pg_json_opt_vec_wh_greater_than(
            rd_ids: <Self::PgType as pg_crud::PgType>::RdIds,
            cr: <Self::PgType as pg_crud::PgType>::Cr,
        ) -> Option<
            pg_crud::NotEmptyUnqVec<
                pg_crud::SingleOrMultiple<<Self::PgType as pg_crud::PgType>::Wh>,
            >,
        > {
            <Self as pg_crud::PgJsonTestCases>::rd_ids_and_cr_into_pg_json_opt_vec_wh_greater_than(
                rd_ids, cr,
            )
        }
        fn rd_ids_and_cr_into_pg_json_opt_vec_wh_btwn(
            rd_ids: <Self::PgType as pg_crud::PgType>::RdIds,
            cr: <Self::PgType as pg_crud::PgType>::Cr,
        ) -> Option<
            pg_crud::NotEmptyUnqVec<
                pg_crud::SingleOrMultiple<<Self::PgType as pg_crud::PgType>::Wh>,
            >,
        > {
            <Self as pg_crud::PgJsonTestCases>::rd_ids_and_cr_into_pg_json_opt_vec_wh_btwn(
                rd_ids, cr,
            )
        }
        fn rd_ids_and_cr_into_pg_json_opt_vec_wh_in(
            rd_ids: <Self::PgType as pg_crud::PgType>::RdIds,
            cr: <Self::PgType as pg_crud::PgType>::Cr,
        ) -> Option<
            pg_crud::NotEmptyUnqVec<
                pg_crud::SingleOrMultiple<<Self::PgType as pg_crud::PgType>::Wh>,
            >,
        > {
            <Self as pg_crud::PgJsonTestCases>::rd_ids_and_cr_into_pg_json_opt_vec_wh_in(rd_ids, cr)
        }
        fn rd_ids_and_cr_into_pg_json_opt_vec_wh_rgx(
            rd_ids: <Self::PgType as pg_crud::PgType>::RdIds,
            cr: <Self::PgType as pg_crud::PgType>::Cr,
        ) -> Option<
            pg_crud::NotEmptyUnqVec<
                pg_crud::SingleOrMultiple<<Self::PgType as pg_crud::PgType>::Wh>,
            >,
        > {
            <Self as pg_crud::PgJsonTestCases>::rd_ids_and_cr_into_pg_json_opt_vec_wh_rgx(
                rd_ids, cr,
            )
        }
        fn rd_ids_and_cr_into_pg_json_opt_vec_wh_contains_el_greater_than(
            rd_ids: <Self::PgType as pg_crud::PgType>::RdIds,
            cr: <Self::PgType as pg_crud::PgType>::Cr,
        ) -> Option<
            pg_crud::NotEmptyUnqVec<
                pg_crud::SingleOrMultiple<<Self::PgType as pg_crud::PgType>::Wh>,
            >,
        > {
            < Self as pg_crud :: PgJsonTestCases > :: rd_ids_and_cr_into_pg_json_opt_vec_wh_contains_el_greater_than (rd_ids , cr)
        }
        fn rd_ids_and_cr_into_pg_json_opt_vec_wh_contains_el_rgx(
            rd_ids: <Self::PgType as pg_crud::PgType>::RdIds,
            cr: <Self::PgType as pg_crud::PgType>::Cr,
        ) -> Option<
            pg_crud::NotEmptyUnqVec<
                pg_crud::SingleOrMultiple<<Self::PgType as pg_crud::PgType>::Wh>,
            >,
        > {
            < Self as pg_crud :: PgJsonTestCases > :: rd_ids_and_cr_into_pg_json_opt_vec_wh_contains_el_rgx (rd_ids , cr)
        }
    }
    #[allow(clippy::arbitrary_source_item_ordering)]
    impl pg_crud::PgTypeNotPk for OptObjExampleAsNlJsonbObj {
        type PgType = Self;
        type Cr = OptObjExampleAsNlJsonbObjCr;
    }
    #[derive(Debug, Clone, Copy)]
    pub struct VecOfObjExampleWithIdAsNnArrOfNnJsonbObjWithId;
    #[allow(clippy::arbitrary_source_item_ordering)]
    #[derive(
        Debug,
        Clone,
        PartialEq,
        serde :: Serialize,
        serde :: Deserialize,
        utoipa :: ToSchema,
        schemars :: JsonSchema,
    )]
    pub struct VecOfObjExampleWithIdAsNnArrOfNnJsonbObjWithIdTt(
        Vec<ObjExampleWithIdAsNnJsonbObjWithIdTt>,
    );
    impl VecOfObjExampleWithIdAsNnArrOfNnJsonbObjWithIdTt {
        #[must_use]
        pub const fn new(v: Vec<ObjExampleWithIdAsNnJsonbObjWithIdTt>) -> Self {
            Self(v)
        }
    }
    impl pg_crud::DfltSomeOneEl for VecOfObjExampleWithIdAsNnArrOfNnJsonbObjWithIdTt {
        fn dflt_some_one_el() -> Self {
            Self(vec![pg_crud::DfltSomeOneEl::dflt_some_one_el()])
        }
    }
    impl sqlx::Type<sqlx::Postgres> for VecOfObjExampleWithIdAsNnArrOfNnJsonbObjWithIdTt {
        fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> bool {
            <sqlx::types::Json<Self> as sqlx::Type<sqlx::Postgres>>::compatible(ty)
        }
        fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
            <sqlx::types::Json<Self> as sqlx::Type<sqlx::Postgres>>::type_info()
        }
    }
    impl sqlx::Encode<'_, sqlx::Postgres> for VecOfObjExampleWithIdAsNnArrOfNnJsonbObjWithIdTt {
        fn encode_by_ref(
            &self,
            buf: &mut sqlx::postgres::PgArgumentBuffer,
        ) -> Result<sqlx::encode::IsNull, Box<dyn std::error::Error + Send + Sync>> {
            sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&sqlx::types::Json(self), buf)
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        serde :: Serialize,
        serde :: Deserialize,
        utoipa :: ToSchema,
        schemars :: JsonSchema,
    )]
    pub struct VecOfObjExampleWithIdAsNnArrOfNnJsonbObjWithIdCr(
        Vec<ObjExampleWithIdAsNnJsonbObjWithIdCr>,
    );
    impl VecOfObjExampleWithIdAsNnArrOfNnJsonbObjWithIdCr {
        #[must_use]
        pub const fn new(v: Vec<ObjExampleWithIdAsNnJsonbObjWithIdCr>) -> Self {
            Self(v)
        }
    }
    impl pg_crud::DfltSomeOneEl for VecOfObjExampleWithIdAsNnArrOfNnJsonbObjWithIdCr {
        fn dflt_some_one_el() -> Self {
            Self(vec![pg_crud::DfltSomeOneEl::dflt_some_one_el()])
        }
    }
    impl sqlx::Type<sqlx::Postgres> for VecOfObjExampleWithIdAsNnArrOfNnJsonbObjWithIdCr {
        fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> bool {
            <sqlx::types::Json<Self> as sqlx::Type<sqlx::Postgres>>::compatible(ty)
        }
        fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
            <sqlx::types::Json<Self> as sqlx::Type<sqlx::Postgres>>::type_info()
        }
    }
    impl sqlx::Encode<'_, sqlx::Postgres> for VecOfObjExampleWithIdAsNnArrOfNnJsonbObjWithIdCr {
        fn encode_by_ref(
            &self,
            buf: &mut sqlx::postgres::PgArgumentBuffer,
        ) -> Result<sqlx::encode::IsNull, Box<dyn std::error::Error + Send + Sync>> {
            sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&sqlx::types::Json(self), buf)
        }
    }
    impl std::fmt::Display for VecOfObjExampleWithIdAsNnArrOfNnJsonbObjWithIdCr {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{self:?}")
        }
    }
    impl loc_lib::ToErrString for VecOfObjExampleWithIdAsNnArrOfNnJsonbObjWithIdCr {
        fn to_err_string(&self) -> String {
            format!("{self:#?}")
        }
    }
    #[allow(clippy::arbitrary_source_item_ordering)]
    #[derive(Debug, Clone, PartialEq, serde :: Serialize)]
    pub struct VecOfObjExampleWithIdAsNnArrOfNnJsonbObjWithIdCrForQuery(
        Vec<ObjExampleWithIdAsNnJsonbObjWithIdCrForQuery>,
    );
    impl From<VecOfObjExampleWithIdAsNnArrOfNnJsonbObjWithIdCr>
        for VecOfObjExampleWithIdAsNnArrOfNnJsonbObjWithIdCrForQuery
    {
        fn from(v: VecOfObjExampleWithIdAsNnArrOfNnJsonbObjWithIdCr) -> Self {
            Self(
                v.0.into_iter()
                    .map(ObjExampleWithIdAsNnJsonbObjWithIdCrForQuery::from)
                    .collect(),
            )
        }
    }
    impl sqlx::Type<sqlx::Postgres> for VecOfObjExampleWithIdAsNnArrOfNnJsonbObjWithIdCrForQuery {
        fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> bool {
            <sqlx::types::Json<Self> as sqlx::Type<sqlx::Postgres>>::compatible(ty)
        }
        fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
            <sqlx::types::Json<Self> as sqlx::Type<sqlx::Postgres>>::type_info()
        }
    }
    impl sqlx::Encode<'_, sqlx::Postgres> for VecOfObjExampleWithIdAsNnArrOfNnJsonbObjWithIdCrForQuery {
        fn encode_by_ref(
            &self,
            buf: &mut sqlx::postgres::PgArgumentBuffer,
        ) -> Result<sqlx::encode::IsNull, Box<dyn std::error::Error + Send + Sync>> {
            sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&sqlx::types::Json(self), buf)
        }
    }
    #[allow(clippy::arbitrary_source_item_ordering)]
    #[derive(
        Debug,
        Clone,
        PartialEq,
        serde :: Serialize,
        serde :: Deserialize,
        utoipa :: ToSchema,
        schemars :: JsonSchema,
    )]
    pub struct VecOfObjExampleWithIdAsNnArrOfNnJsonbObjWithIdSel {
        obj_example_with_id_as_nn_jsonb_obj_with_id_sel: ObjExampleWithIdAsNnJsonbObjWithIdSel,
        dim1_pgn: pg_crud::PgnStartsWithZero,
    }
    impl VecOfObjExampleWithIdAsNnArrOfNnJsonbObjWithIdSel {
        #[must_use]
        pub const fn new(
            obj_example_with_id_as_nn_jsonb_obj_with_id_sel: ObjExampleWithIdAsNnJsonbObjWithIdSel,
            dim1_pgn: pg_crud::PgnStartsWithZero,
        ) -> Self {
            Self {
                obj_example_with_id_as_nn_jsonb_obj_with_id_sel,
                dim1_pgn,
            }
        }
        fn sel_qp_pg_type(&self, col: &str) -> Result<String, pg_crud::QpEr> {
            let obj_example_with_id_as_nn_jsonb_obj_with_id_sel = {
                let mut acc_sel_qp_with_id = String::default();
                for el in self
                    .obj_example_with_id_as_nn_jsonb_obj_with_id_sel
                    .0
                    .to_vec()
                {
                    if { use std :: fmt :: Write as _ ; write ! (acc_sel_qp_with_id , "{}||" , match el { ObjExampleWithIdAsNnJsonbObjWithIdSelEl :: Id (v_3c8acf6a) => match < pg_crud :: UuidUuidAsNnJsonbString as pg_crud :: PgJson > :: sel_qp (v_3c8acf6a , "id" , "value" , col , false ,) { Ok (v_d54cf786) => v_d54cf786 , Err (er) => { return Err (er) ; } } , ObjExampleWithIdAsNnJsonbObjWithIdSelEl :: Field0 (v_3c8acf6a) => match < pg_crud :: I8AsNnJsonbNbr as pg_crud :: PgJson > :: sel_qp (v_3c8acf6a , "field_0" , "value" , col , false ,) { Ok (v_d54cf786) => v_d54cf786 , Err (er) => { return Err (er) ; } } , ObjExampleWithIdAsNnJsonbObjWithIdSelEl :: Field1 (v_3c8acf6a) => match < pg_crud :: OptI8AsNlJsonbNbr as pg_crud :: PgJson > :: sel_qp (v_3c8acf6a , "field_1" , "value" , col , false ,) { Ok (v_d54cf786) => v_d54cf786 , Err (er) => { return Err (er) ; } } , ObjExampleWithIdAsNnJsonbObjWithIdSelEl :: Field2 (v_3c8acf6a) => match < pg_crud :: VecOfI8AsNnArrOfNnJsonbNbr as pg_crud :: PgJson > :: sel_qp (v_3c8acf6a , "field_2" , "value" , col , false ,) { Ok (v_d54cf786) => v_d54cf786 , Err (er) => { return Err (er) ; } } }) } . is_err () { return Err (pg_crud :: QpEr :: WriteIntoBuffer { loc : loc_lib :: loc ! () }) ; }
                }
                let _: Option<char> = acc_sel_qp_with_id.pop();
                let _: Option<char> = acc_sel_qp_with_id.pop();
                acc_sel_qp_with_id
            };
            let dim1_start = self.dim1_pgn.start();
            let dim1_end = self.dim1_pgn.end();
            Ok(format!(
                "(case when (jsonb_array_length({col}) = 0) then '[]'::jsonb else (select jsonb_agg(({obj_example_with_id_as_nn_jsonb_obj_with_id_sel})) from jsonb_array_elements((select {col})) with ordinality where ordinality between {dim1_start} and {dim1_end}) end)"
            ))
        }
    }
    impl sqlx::Type<sqlx::Postgres> for VecOfObjExampleWithIdAsNnArrOfNnJsonbObjWithIdSel {
        fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> bool {
            <sqlx::types::Json<Self> as sqlx::Type<sqlx::Postgres>>::compatible(ty)
        }
        fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
            <sqlx::types::Json<Self> as sqlx::Type<sqlx::Postgres>>::type_info()
        }
    }
    impl sqlx::Decode<'_, sqlx::Postgres> for VecOfObjExampleWithIdAsNnArrOfNnJsonbObjWithIdSel {
        fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
            match <sqlx::types::Json<Self> as sqlx::Decode<sqlx::Postgres>>::decode(value) {
                Ok(v) => Ok(v.0),
                Err(er) => Err(er),
            }
        }
    }
    impl pg_crud::DfltSomeOneEl for VecOfObjExampleWithIdAsNnArrOfNnJsonbObjWithIdSel {
        fn dflt_some_one_el() -> Self {
            Self {
                obj_example_with_id_as_nn_jsonb_obj_with_id_sel:
                    pg_crud::DfltSomeOneEl::dflt_some_one_el(),
                dim1_pgn: pg_crud::DfltSomeOneEl::dflt_some_one_el(),
            }
        }
    }
    impl pg_crud::DfltSomeOneElMaxPageSize for VecOfObjExampleWithIdAsNnArrOfNnJsonbObjWithIdSel {
        fn dflt_some_one_el_max_page_size() -> Self {
            Self {
                obj_example_with_id_as_nn_jsonb_obj_with_id_sel:
                    pg_crud::DfltSomeOneElMaxPageSize::dflt_some_one_el_max_page_size(),
                dim1_pgn: pg_crud::DfltSomeOneElMaxPageSize::dflt_some_one_el_max_page_size(),
            }
        }
    }
    #[allow(clippy::arbitrary_source_item_ordering)]
    #[derive(
        Debug,
        Clone,
        PartialEq,
        serde :: Serialize,
        serde :: Deserialize,
        utoipa :: ToSchema,
        schemars :: JsonSchema,
    )]
    pub enum VecOfObjExampleWithIdAsNnArrOfNnJsonbObjWithIdWh {
        Eq(
            pg_crud::PgJsonWhEq<
                <VecOfObjExampleWithIdAsNnArrOfNnJsonbObjWithId as pg_crud::PgJson>::Tt,
            >,
        ),
        DimOneEq(pg_crud::PgJsonWhDimOneEq<ObjExampleWithIdAsNnJsonbObjWithIdTt>),
        LenEq(pg_crud::PgJsonWhLenEq),
        LenGreaterThan(pg_crud::PgJsonWhLenGreaterThan),
        In(
            pg_crud::PgJsonWhIn<
                <VecOfObjExampleWithIdAsNnArrOfNnJsonbObjWithId as pg_crud::PgJson>::Tt,
            >,
        ),
        DimOneIn(pg_crud::PgJsonWhDimOneIn<ObjExampleWithIdAsNnJsonbObjWithIdTt>),
        ContainsAllElsOfArr(
            pg_crud::PgJsonWhContainsAllElsOfArr<ObjExampleWithIdAsNnJsonbObjWithIdTt>,
        ),
        OverlapsWithArr(pg_crud::PgJsonWhOverlapsWithArr<ObjExampleWithIdAsNnJsonbObjWithIdTt>),
        ElId(pg_crud::PgTypeWh<<pg_crud::UuidUuidAsNnJsonbString as pg_crud::PgJson>::Wh>),
        ElField0(pg_crud::PgTypeWh<<pg_crud::I8AsNnJsonbNbr as pg_crud::PgJson>::Wh>),
        ElField1(pg_crud::PgTypeWh<<pg_crud::OptI8AsNlJsonbNbr as pg_crud::PgJson>::Wh>),
        ElField2(pg_crud::PgTypeWh<<pg_crud::VecOfI8AsNnArrOfNnJsonbNbr as pg_crud::PgJson>::Wh>),
    }
    #[allow(clippy::arbitrary_source_item_ordering)]
    impl<'lt> pg_crud::PgTypeWhFlt<'lt> for VecOfObjExampleWithIdAsNnArrOfNnJsonbObjWithIdWh {
        fn qp(
            &self,
            incr: &mut u64,
            col: &dyn std::fmt::Display,
            add_oprtr: bool,
        ) -> Result<String, pg_crud::QpEr> {
            let mut gen_el_query = |oprtr: &pg_crud::Oprtr,
                                    v_637adcbd: &dyn pg_crud::PgTypeWhFlt<'_>,
                                    field: &str|
             -> Result<String, pg_crud::QpEr> {
                let oprtr_qp = oprtr.to_qp(add_oprtr);
                let elem = "elem";
                let v_9696ee60 = match pg_crud::PgTypeWhFlt::qp(
                    v_637adcbd,
                    incr,
                    &format!("{elem}->'{field}'"),
                    false,
                ) {
                    Ok(v_c7ec4e53) => v_c7ec4e53,
                    Err(er) => {
                        return Err(er);
                    }
                };
                Ok(format!(
                    "{oprtr_qp}(exists (select 1 from jsonb_array_elements({col}) as {elem} where {v_9696ee60}))"
                ))
            };
            match &self {
                Self::Eq(v_df049001) => pg_crud::PgTypeWhFlt::qp(v_df049001, incr, col, add_oprtr),
                Self::DimOneEq(v_df049001) => {
                    pg_crud::PgTypeWhFlt::qp(v_df049001, incr, col, add_oprtr)
                }
                Self::LenEq(v_df049001) => {
                    pg_crud::PgTypeWhFlt::qp(v_df049001, incr, col, add_oprtr)
                }
                Self::LenGreaterThan(v_df049001) => {
                    pg_crud::PgTypeWhFlt::qp(v_df049001, incr, col, add_oprtr)
                }
                Self::In(v_df049001) => pg_crud::PgTypeWhFlt::qp(v_df049001, incr, col, add_oprtr),
                Self::DimOneIn(v_df049001) => {
                    pg_crud::PgTypeWhFlt::qp(v_df049001, incr, col, add_oprtr)
                }
                Self::ContainsAllElsOfArr(v_df049001) => {
                    pg_crud::PgTypeWhFlt::qp(v_df049001, incr, col, add_oprtr)
                }
                Self::OverlapsWithArr(v_df049001) => {
                    pg_crud::PgTypeWhFlt::qp(v_df049001, incr, col, add_oprtr)
                }
                Self::ElId(v_5ff7ccdf) => gen_el_query(v_5ff7ccdf.get_oprtr(), v_5ff7ccdf, "id"),
                Self::ElField0(v_5ff7ccdf) => {
                    gen_el_query(v_5ff7ccdf.get_oprtr(), v_5ff7ccdf, "field_0")
                }
                Self::ElField1(v_5ff7ccdf) => {
                    gen_el_query(v_5ff7ccdf.get_oprtr(), v_5ff7ccdf, "field_1")
                }
                Self::ElField2(v_5ff7ccdf) => {
                    gen_el_query(v_5ff7ccdf.get_oprtr(), v_5ff7ccdf, "field_2")
                }
            }
        }
        fn qb(
            self,
            query: sqlx::query::Query<'lt, sqlx::Postgres, sqlx::postgres::PgArguments>,
        ) -> Result<sqlx::query::Query<'lt, sqlx::Postgres, sqlx::postgres::PgArguments>, String>
        {
            match self {
                Self::Eq(v) => pg_crud::PgTypeWhFlt::qb(v, query),
                Self::DimOneEq(v) => pg_crud::PgTypeWhFlt::qb(v, query),
                Self::LenEq(v) => pg_crud::PgTypeWhFlt::qb(v, query),
                Self::LenGreaterThan(v) => pg_crud::PgTypeWhFlt::qb(v, query),
                Self::In(v) => pg_crud::PgTypeWhFlt::qb(v, query),
                Self::DimOneIn(v) => pg_crud::PgTypeWhFlt::qb(v, query),
                Self::ContainsAllElsOfArr(v) => pg_crud::PgTypeWhFlt::qb(v, query),
                Self::OverlapsWithArr(v) => pg_crud::PgTypeWhFlt::qb(v, query),
                Self::ElId(v) => pg_crud::PgTypeWhFlt::qb(v, query),
                Self::ElField0(v) => pg_crud::PgTypeWhFlt::qb(v, query),
                Self::ElField1(v) => pg_crud::PgTypeWhFlt::qb(v, query),
                Self::ElField2(v) => pg_crud::PgTypeWhFlt::qb(v, query),
            }
        }
    }
    impl loc_lib::ToErrString for VecOfObjExampleWithIdAsNnArrOfNnJsonbObjWithIdWh {
        fn to_err_string(&self) -> String {
            format!("{self:?}")
        }
    }
    impl pg_crud::AllEnumVrtsArrDfltSomeOneEl for VecOfObjExampleWithIdAsNnArrOfNnJsonbObjWithIdWh {
        fn all_vrts_dflt_some_one_el() -> Vec<Self> {
            vec![
                Self::Eq(pg_crud::DfltSomeOneEl::dflt_some_one_el()),
                Self::DimOneEq(pg_crud::DfltSomeOneEl::dflt_some_one_el()),
                Self::LenEq(pg_crud::DfltSomeOneEl::dflt_some_one_el()),
                Self::LenGreaterThan(pg_crud::DfltSomeOneEl::dflt_some_one_el()),
                Self::In(pg_crud::DfltSomeOneEl::dflt_some_one_el()),
                Self::DimOneIn(pg_crud::DfltSomeOneEl::dflt_some_one_el()),
                Self::ContainsAllElsOfArr(pg_crud::DfltSomeOneEl::dflt_some_one_el()),
                Self::OverlapsWithArr(pg_crud::DfltSomeOneEl::dflt_some_one_el()),
                Self::ElId(pg_crud::DfltSomeOneEl::dflt_some_one_el()),
                Self::ElField0(pg_crud::DfltSomeOneEl::dflt_some_one_el()),
                Self::ElField1(pg_crud::DfltSomeOneEl::dflt_some_one_el()),
                Self::ElField2(pg_crud::DfltSomeOneEl::dflt_some_one_el()),
            ]
        }
    }
    #[allow(clippy::arbitrary_source_item_ordering)]
    #[derive(
        Debug,
        Clone,
        PartialEq,
        serde :: Serialize,
        serde :: Deserialize,
        utoipa :: ToSchema,
        schemars :: JsonSchema,
    )]
    pub struct VecOfObjExampleWithIdAsNnArrOfNnJsonbObjWithIdRd(
        Vec<ObjExampleWithIdAsNnJsonbObjWithIdRd>,
    );
    impl VecOfObjExampleWithIdAsNnArrOfNnJsonbObjWithIdRd {
        #[must_use]
        pub const fn new(v: Vec<ObjExampleWithIdAsNnJsonbObjWithIdRd>) -> Self {
            Self(v)
        }
    }
    impl pg_crud::DfltSomeOneEl for VecOfObjExampleWithIdAsNnArrOfNnJsonbObjWithIdRd {
        fn dflt_some_one_el() -> Self {
            Self(vec![pg_crud::DfltSomeOneEl::dflt_some_one_el()])
        }
    }
    impl sqlx::Type<sqlx::Postgres> for VecOfObjExampleWithIdAsNnArrOfNnJsonbObjWithIdRd {
        fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> bool {
            <sqlx::types::Json<Self> as sqlx::Type<sqlx::Postgres>>::compatible(ty)
        }
        fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
            <sqlx::types::Json<Self> as sqlx::Type<sqlx::Postgres>>::type_info()
        }
    }
    impl sqlx::Encode<'_, sqlx::Postgres> for VecOfObjExampleWithIdAsNnArrOfNnJsonbObjWithIdRd {
        fn encode_by_ref(
            &self,
            buf: &mut sqlx::postgres::PgArgumentBuffer,
        ) -> Result<sqlx::encode::IsNull, Box<dyn std::error::Error + Send + Sync>> {
            sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&sqlx::types::Json(self), buf)
        }
    }
    impl sqlx::Decode<'_, sqlx::Postgres> for VecOfObjExampleWithIdAsNnArrOfNnJsonbObjWithIdRd {
        fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
            match <sqlx::types::Json<Self> as sqlx::Decode<sqlx::Postgres>>::decode(value) {
                Ok(v) => Ok(v.0),
                Err(er) => Err(er),
            }
        }
    }
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    pub struct VecOfObjExampleWithIdAsNnArrOfNnJsonbObjWithIdRdIds(
        pg_crud::V<Vec<ObjExampleWithIdAsNnJsonbObjWithIdRdIds>>,
    );
    impl sqlx::Decode<'_, sqlx::Postgres> for VecOfObjExampleWithIdAsNnArrOfNnJsonbObjWithIdRdIds {
        fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
            match <sqlx::types::Json<Self> as sqlx::Decode<sqlx::Postgres>>::decode(value) {
                Ok(v) => Ok(v.0),
                Err(er) => Err(er),
            }
        }
    }
    impl sqlx::Type<sqlx::Postgres> for VecOfObjExampleWithIdAsNnArrOfNnJsonbObjWithIdRdIds {
        fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> bool {
            <sqlx::types::Json<Self> as sqlx::Type<sqlx::Postgres>>::compatible(ty)
        }
        fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
            <sqlx::types::Json<Self> as sqlx::Type<sqlx::Postgres>>::type_info()
        }
    }
    pub type VecOfObjExampleWithIdAsNnArrOfNnJsonbObjWithIdRdInn =
        Vec<ObjExampleWithIdAsNnJsonbObjWithIdRdInn>;
    #[allow(clippy::arbitrary_source_item_ordering)]
    #[derive(
        Debug, Clone, PartialEq, serde :: Serialize, utoipa :: ToSchema, schemars :: JsonSchema,
    )]
    pub struct VecOfObjExampleWithIdAsNnArrOfNnJsonbObjWithIdUpd {
        #[serde(skip_serializing_if = "Vec::is_empty")]
        cr: Vec<ObjExampleWithIdAsNnJsonbObjWithIdCr>,
        upd: pg_crud::UnqVec<ObjExampleWithIdAsNnJsonbObjWithIdUpdEl>,
        #[serde(skip_serializing_if = "Vec::is_empty")]
        del: Vec<<pg_crud::UuidUuidAsNnJsonbString as pg_crud::PgJson>::Upd>,
    }
    #[derive(
        Debug, serde :: Serialize, serde :: Deserialize, thiserror :: Error, loc_lib :: Location,
    )]
    pub enum VecOfObjExampleWithIdAsNnArrOfNnJsonbObjWithIdUpdTryNewEr {
        CrUpdDelAreEmpty {
            loc: loc_lib::loc::Loc,
        },
        IdsAreNotUnq {
            #[eo_to_err_string_serde]
            duplicate: String,
            loc: loc_lib::loc::Loc,
        },
        NotUnqIdInJsonDelArr {
            #[eo_to_err_string_serde]
            er: String,
            loc: loc_lib::loc::Loc,
        },
        NotUnqIdInJsonUpdAndDelArrs {
            #[eo_to_err_string_serde]
            er: String,
            loc: loc_lib::loc::Loc,
        },
    }
    impl VecOfObjExampleWithIdAsNnArrOfNnJsonbObjWithIdUpd {
        pub fn try_new(
            cr: Vec<ObjExampleWithIdAsNnJsonbObjWithIdCr>,
            upd: pg_crud::UnqVec<ObjExampleWithIdAsNnJsonbObjWithIdUpdEl>,
            del: Vec<<pg_crud::UuidUuidAsNnJsonbString as pg_crud::PgJson>::Upd>,
        ) -> Result<Self, VecOfObjExampleWithIdAsNnArrOfNnJsonbObjWithIdUpdTryNewEr> {
            if cr.is_empty() && upd.is_empty() && del.is_empty() {
                return Err(
                    VecOfObjExampleWithIdAsNnArrOfNnJsonbObjWithIdUpdTryNewEr::CrUpdDelAreEmpty {
                        loc: loc_lib::loc!(),
                    },
                );
            }
            {
                let mut acc_2bf4e098 = Vec::new();
                for el in upd.to_vec() {
                    if acc_2bf4e098.contains(&&el.id) {
                        return Err (VecOfObjExampleWithIdAsNnArrOfNnJsonbObjWithIdUpdTryNewEr :: IdsAreNotUnq { duplicate : < < pg_crud :: UuidUuidAsNnJsonbString as pg_crud :: PgJson > :: Upd as loc_lib :: ToErrString > :: to_err_string (& el . id) , loc : loc_lib :: loc ! () }) ;
                    }
                    acc_2bf4e098.push(&el.id);
                }
                for el in &del {
                    if acc_2bf4e098.contains(&el) {
                        return Err (VecOfObjExampleWithIdAsNnArrOfNnJsonbObjWithIdUpdTryNewEr :: IdsAreNotUnq { duplicate : < < pg_crud :: UuidUuidAsNnJsonbString as pg_crud :: PgJson > :: Upd as loc_lib :: ToErrString > :: to_err_string (el) , loc : loc_lib :: loc ! () }) ;
                    }
                    acc_2bf4e098.push(el);
                }
            }
            {
                let upd_acc = upd
                    .to_vec()
                    .iter()
                    .map(|el| &el.id)
                    .collect::<Vec<&<pg_crud::UuidUuidAsNnJsonbString as pg_crud::PgJson>::Upd>>();
                let del_acc = {
                    let mut del_acc = Vec::new();
                    for el in &del {
                        if del_acc.contains(&el) {
                            return Err (VecOfObjExampleWithIdAsNnArrOfNnJsonbObjWithIdUpdTryNewEr :: NotUnqIdInJsonDelArr { er : format ! ("custom serde er deserializing VecOfObjExampleWithIdAsNnArrOfNnJsonbObjWithIdUpd: not unique id in json del arr: {}" , < pg_crud :: UuidUuidAsNnJsonbString as pg_crud :: PgJsonObjVecElId > :: get_inn (& el . clone () . into ())) , loc : loc_lib :: loc ! () }) ;
                        }
                        del_acc.push(el);
                    }
                    del_acc
                };
                for el in upd_acc {
                    if del_acc.contains(&el) {
                        return Err (VecOfObjExampleWithIdAsNnArrOfNnJsonbObjWithIdUpdTryNewEr :: NotUnqIdInJsonUpdAndDelArrs { er : format ! ("custom serde er deserializing VecOfObjExampleWithIdAsNnArrOfNnJsonbObjWithIdUpd: not unique id in json upd and del arrs: {}" , < pg_crud :: UuidUuidAsNnJsonbString as pg_crud :: PgJsonObjVecElId > :: get_inn (& el . clone () . into ())) , loc : loc_lib :: loc ! () }) ;
                    }
                }
            }
            Ok(Self { cr, upd, del })
        }
    }
    #[derive(serde :: Deserialize, Default)]
    #[allow(clippy::arbitrary_source_item_ordering)]
    struct VecOfObjExampleWithIdAsNnArrOfNnJsonbObjWithIdUpdRaw {
        #[serde(default)]
        cr: Vec<ObjExampleWithIdAsNnJsonbObjWithIdCr>,
        #[serde(default)]
        upd: pg_crud::UnqVec<ObjExampleWithIdAsNnJsonbObjWithIdUpdEl>,
        #[serde(default)]
        del: Vec<<pg_crud::UuidUuidAsNnJsonbString as pg_crud::PgJson>::Upd>,
    }
    #[allow(unused_qualifications)]
    #[allow(clippy::absolute_paths)]
    #[allow(clippy::arbitrary_source_item_ordering)]
    const _: () = {
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for VecOfObjExampleWithIdAsNnArrOfNnJsonbObjWithIdUpd {
            fn deserialize<__D>(__deserializer: __D) -> Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                let raw = < VecOfObjExampleWithIdAsNnArrOfNnJsonbObjWithIdUpdRaw as _serde :: Deserialize > :: deserialize (__deserializer) ? ;
                Self::try_new(raw.cr, raw.upd, raw.del)
                    .map_err(|er| _serde::de::Error::custom(format!("{er:?}")))
            }
        }
    };
    impl pg_crud::DfltSomeOneEl for VecOfObjExampleWithIdAsNnArrOfNnJsonbObjWithIdUpd {
        fn dflt_some_one_el() -> Self {
            Self {
                cr: vec![pg_crud::DfltSomeOneEl::dflt_some_one_el()],
                upd: pg_crud::DfltSomeOneEl::dflt_some_one_el(),
                del: vec![pg_crud::DfltSomeOneEl::dflt_some_one_el()],
            }
        }
    }
    #[allow(clippy::arbitrary_source_item_ordering)]
    #[derive(Debug, Clone, PartialEq, serde :: Serialize)]
    pub struct VecOfObjExampleWithIdAsNnArrOfNnJsonbObjWithIdUpdForQuery {
        #[serde(skip_serializing_if = "Vec::is_empty")]
        cr: Vec<ObjExampleWithIdAsNnJsonbObjWithIdCrForQuery>,
        upd: pg_crud::UnqVec<ObjExampleWithIdAsNnJsonbObjWithIdUpdForQueryEl>,
        #[serde(skip_serializing_if = "Vec::is_empty")]
        del: Vec<<pg_crud::UuidUuidAsNnJsonbString as pg_crud::PgJson>::UpdForQuery>,
    }
    impl VecOfObjExampleWithIdAsNnArrOfNnJsonbObjWithIdUpdForQuery {
        #[allow(clippy::single_call_fn)]
        fn sel_only_updd_ids_qp(
            &self,
            col_field: &str,
            incr: &mut u64,
        ) -> Result<String, pg_crud::QpEr> {
            Ok(format!(
                "(select jsonb_agg({}) from jsonb_array_elements({}) as elem where elem->>'id' in ({}))",
                {
                    let mut acc_57cd0744 = String::new();
                    for el_d7561f40 in self.upd.to_vec() {
                        let mut acc_892857b1 = String::new();
                        match < pg_crud :: UuidUuidAsNnJsonbString as pg_crud :: PgJson > :: sel_only_updd_ids_qp (& el_d7561f40 . id , "id" , "elem" , incr) { Ok (mut v_6bac798d) => { let _ : Option < char > = v_6bac798d . pop () ; if { use std :: fmt :: Write as _ ; write ! (acc_892857b1 , "jsonb_build_object({v_6bac798d})||") } . is_err () { return Err (pg_crud :: QpEr :: WriteIntoBuffer { loc : loc_lib :: loc ! () }) ; } } Err (er) => { return Err (er) ; } }
                        for el_738b2a83 in el_d7561f40.fields.0.to_vec() {
                            match & el_738b2a83 { ObjExampleAsNnJsonbObjUpdForQueryEl :: Field0 (v_40a8d7a1) => match < pg_crud :: I8AsNnJsonbNbr as pg_crud :: PgJson > :: sel_only_updd_ids_qp (& v_40a8d7a1 . v , "field_0" , "elem" , incr) { Ok (mut v_33d3b52e) => { let _ : Option < char > = v_33d3b52e . pop () ; if { use std :: fmt :: Write as _ ; write ! (acc_892857b1 , "jsonb_build_object({v_33d3b52e})||") } . is_err () { return Err (pg_crud :: QpEr :: WriteIntoBuffer { loc : loc_lib :: loc ! () }) ; } } , Err (er) => { return Err (er) ; } } , ObjExampleAsNnJsonbObjUpdForQueryEl :: Field1 (v_40a8d7a1) => match < pg_crud :: OptI8AsNlJsonbNbr as pg_crud :: PgJson > :: sel_only_updd_ids_qp (& v_40a8d7a1 . v , "field_1" , "elem" , incr) { Ok (mut v_33d3b52e) => { let _ : Option < char > = v_33d3b52e . pop () ; if { use std :: fmt :: Write as _ ; write ! (acc_892857b1 , "jsonb_build_object({v_33d3b52e})||") } . is_err () { return Err (pg_crud :: QpEr :: WriteIntoBuffer { loc : loc_lib :: loc ! () }) ; } } , Err (er) => { return Err (er) ; } } , ObjExampleAsNnJsonbObjUpdForQueryEl :: Field2 (v_40a8d7a1) => match < pg_crud :: VecOfI8AsNnArrOfNnJsonbNbr as pg_crud :: PgJson > :: sel_only_updd_ids_qp (& v_40a8d7a1 . v , "field_2" , "elem" , incr) { Ok (mut v_33d3b52e) => { let _ : Option < char > = v_33d3b52e . pop () ; if { use std :: fmt :: Write as _ ; write ! (acc_892857b1 , "jsonb_build_object({v_33d3b52e})||") } . is_err () { return Err (pg_crud :: QpEr :: WriteIntoBuffer { loc : loc_lib :: loc ! () }) ; } } , Err (er) => { return Err (er) ; } } }
                        }
                        let _: Option<char> = acc_892857b1.pop();
                        let _: Option<char> = acc_892857b1.pop();
                        if {
                            use std::fmt::Write as _;
                            write!(acc_57cd0744, "{acc_892857b1}||")
                        }
                        .is_err()
                        {
                            return Err(pg_crud::QpEr::WriteIntoBuffer {
                                loc: loc_lib::loc!(),
                            });
                        }
                    }
                    for el_b1359d90 in &self.cr {
                        match < pg_crud :: UuidUuidAsNnJsonbString as pg_crud :: PgJson > :: sel_only_crd_ids_qp (& el_b1359d90 . id , "id" , "elem" , incr) { Ok (mut v_549a93c8) => { let _ : Option < char > = v_549a93c8 . pop () ; if { use std :: fmt :: Write as _ ; write ! (acc_57cd0744 , "jsonb_build_object({v_549a93c8})||") } . is_err () { return Err (pg_crud :: QpEr :: WriteIntoBuffer { loc : loc_lib :: loc ! () }) ; } } , Err (er) => { return Err (er) ; } }
                        match <pg_crud::I8AsNnJsonbNbr as pg_crud::PgJson>::sel_only_crd_ids_qp(
                            &el_b1359d90.field_0,
                            "field_0",
                            "elem",
                            incr,
                        ) {
                            Ok(mut v_549a93c8) => {
                                let _: Option<char> = v_549a93c8.pop();
                                if {
                                    use std::fmt::Write as _;
                                    write!(acc_57cd0744, "jsonb_build_object({v_549a93c8})||")
                                }
                                .is_err()
                                {
                                    return Err(pg_crud::QpEr::WriteIntoBuffer {
                                        loc: loc_lib::loc!(),
                                    });
                                }
                            }
                            Err(er) => {
                                return Err(er);
                            }
                        }
                        match <pg_crud::OptI8AsNlJsonbNbr as pg_crud::PgJson>::sel_only_crd_ids_qp(
                            &el_b1359d90.field_1,
                            "field_1",
                            "elem",
                            incr,
                        ) {
                            Ok(mut v_549a93c8) => {
                                let _: Option<char> = v_549a93c8.pop();
                                if {
                                    use std::fmt::Write as _;
                                    write!(acc_57cd0744, "jsonb_build_object({v_549a93c8})||")
                                }
                                .is_err()
                                {
                                    return Err(pg_crud::QpEr::WriteIntoBuffer {
                                        loc: loc_lib::loc!(),
                                    });
                                }
                            }
                            Err(er) => {
                                return Err(er);
                            }
                        }
                        match < pg_crud :: VecOfI8AsNnArrOfNnJsonbNbr as pg_crud :: PgJson > :: sel_only_crd_ids_qp (& el_b1359d90 . field_2 , "field_2" , "elem" , incr) { Ok (mut v_549a93c8) => { let _ : Option < char > = v_549a93c8 . pop () ; if { use std :: fmt :: Write as _ ; write ! (acc_57cd0744 , "jsonb_build_object({v_549a93c8})||") } . is_err () { return Err (pg_crud :: QpEr :: WriteIntoBuffer { loc : loc_lib :: loc ! () }) ; } } , Err (er) => { return Err (er) ; } }
                    }
                    let _: Option<char> = acc_57cd0744.pop();
                    let _: Option<char> = acc_57cd0744.pop();
                    format!("jsonb_build_object('v',{acc_57cd0744})")
                },
                col_field,
                {
                    let mut acc_d497e8a5 = String::new();
                    for _ in self.upd.to_vec() {
                        match pg_crud::incr_checked_add_one_returning_incr(incr) {
                            Ok(v_c31cb081) => {
                                if {
                                    use std::fmt::Write as _;
                                    write!(acc_d497e8a5, "${v_c31cb081},")
                                }
                                .is_err()
                                {
                                    return Err(pg_crud::QpEr::WriteIntoBuffer {
                                        loc: loc_lib::loc!(),
                                    });
                                }
                            }
                            Err(er) => {
                                return Err(er);
                            }
                        }
                    }
                    for _ in &self.cr {
                        match pg_crud::incr_checked_add_one_returning_incr(incr) {
                            Ok(v_b52c3fe1) => {
                                if {
                                    use std::fmt::Write as _;
                                    write!(acc_d497e8a5, "${v_b52c3fe1},")
                                }
                                .is_err()
                                {
                                    return Err(pg_crud::QpEr::WriteIntoBuffer {
                                        loc: loc_lib::loc!(),
                                    });
                                }
                            }
                            Err(er) => {
                                return Err(er);
                            }
                        }
                    }
                    let _: Option<char> = acc_d497e8a5.pop();
                    acc_d497e8a5
                }
            ))
        }
    }
    impl From<<VecOfObjExampleWithIdAsNnArrOfNnJsonbObjWithId as pg_crud::PgJson>::Upd>
        for <VecOfObjExampleWithIdAsNnArrOfNnJsonbObjWithId as pg_crud::PgJson>::UpdForQuery
    {
        fn from(
            v: <VecOfObjExampleWithIdAsNnArrOfNnJsonbObjWithId as pg_crud::PgJson>::Upd,
        ) -> Self {
            Self {
                cr: v
                    .cr
                    .into_iter()
                    .map(ObjExampleWithIdAsNnJsonbObjWithIdCrForQuery::from)
                    .collect(),
                upd: pg_crud::UnqVec::from_t1_impl_from_t2(v.upd),
                del: v.del.into_iter().map(Into::into).collect(),
            }
        }
    }
    #[allow(clippy::arbitrary_source_item_ordering)]
    impl pg_crud::PgJson for VecOfObjExampleWithIdAsNnArrOfNnJsonbObjWithId {
        type Tt = VecOfObjExampleWithIdAsNnArrOfNnJsonbObjWithIdTt;
        type Cr = VecOfObjExampleWithIdAsNnArrOfNnJsonbObjWithIdCr;
        type CrForQuery = VecOfObjExampleWithIdAsNnArrOfNnJsonbObjWithIdCrForQuery;
        type Sel = VecOfObjExampleWithIdAsNnArrOfNnJsonbObjWithIdSel;
        fn sel_qp(
            v: &Self::Sel,
            fi: &str,
            col_field: &str,
            _: &str,
            _: bool,
        ) -> Result<String, pg_crud::QpEr> {
            let obj_example_with_id_as_nn_jsonb_obj_with_id_sel = {
                let mut acc_sel_qp_arr_with_id = String::default();
                for el in v.obj_example_with_id_as_nn_jsonb_obj_with_id_sel.0.to_vec() {
                    if { use std :: fmt :: Write as _ ; write ! (acc_sel_qp_arr_with_id , "{}||" , match el { ObjExampleWithIdAsNnJsonbObjWithIdSelEl :: Id (v_3c8acf6a) => match < pg_crud :: UuidUuidAsNnJsonbString as pg_crud :: PgJson > :: sel_qp (v_3c8acf6a , "id" , "value" , "value" , false ,) { Ok (v_d54cf786) => v_d54cf786 , Err (er) => { return Err (er) ; } } , ObjExampleWithIdAsNnJsonbObjWithIdSelEl :: Field0 (v_3c8acf6a) => match < pg_crud :: I8AsNnJsonbNbr as pg_crud :: PgJson > :: sel_qp (v_3c8acf6a , "field_0" , "value" , "value" , false ,) { Ok (v_d54cf786) => v_d54cf786 , Err (er) => { return Err (er) ; } } , ObjExampleWithIdAsNnJsonbObjWithIdSelEl :: Field1 (v_3c8acf6a) => match < pg_crud :: OptI8AsNlJsonbNbr as pg_crud :: PgJson > :: sel_qp (v_3c8acf6a , "field_1" , "value" , "value" , false ,) { Ok (v_d54cf786) => v_d54cf786 , Err (er) => { return Err (er) ; } } , ObjExampleWithIdAsNnJsonbObjWithIdSelEl :: Field2 (v_3c8acf6a) => match < pg_crud :: VecOfI8AsNnArrOfNnJsonbNbr as pg_crud :: PgJson > :: sel_qp (v_3c8acf6a , "field_2" , "value" , "value" , false ,) { Ok (v_d54cf786) => v_d54cf786 , Err (er) => { return Err (er) ; } } }) } . is_err () { return Err (pg_crud :: QpEr :: WriteIntoBuffer { loc : loc_lib :: loc ! () }) ; }
                }
                let _: Option<char> = acc_sel_qp_arr_with_id.pop();
                let _: Option<char> = acc_sel_qp_arr_with_id.pop();
                acc_sel_qp_arr_with_id
            };
            let dim1_start = v.dim1_pgn.start();
            let dim1_end = v.dim1_pgn.end();
            Ok(format!(
                "jsonb_build_object('{fi}',jsonb_build_object('v',(case when (jsonb_array_length({col_field}->'{fi}') = 0) then '[]'::jsonb else (select jsonb_agg(({obj_example_with_id_as_nn_jsonb_obj_with_id_sel})) from jsonb_array_elements((select {col_field}->'{fi}')) with ordinality where ordinality between {dim1_start} and {dim1_end}) end)))"
            ))
        }
        type Wh = VecOfObjExampleWithIdAsNnArrOfNnJsonbObjWithIdWh;
        type Rd = VecOfObjExampleWithIdAsNnArrOfNnJsonbObjWithIdRd;
        type RdIds = VecOfObjExampleWithIdAsNnArrOfNnJsonbObjWithIdRdIds;
        fn sel_only_ids_qp(col_field: &str) -> Result<String, pg_crud::QpEr> {
            Ok(format!(
                "jsonb_build_object('v',(select jsonb_agg({}) from jsonb_array_elements({col_field}) as elem))",
                {
                    let mut acc = String::default();
                    if { use std :: fmt :: Write as _ ; write ! (acc , "jsonb_build_object('id',{})||" , match < pg_crud :: UuidUuidAsNnJsonbString as pg_crud :: PgJson > :: sel_only_ids_qp ("elem->'id'") { Ok (v_2317e0af) => v_2317e0af , Err (er) => { return Err (er) ; } }) } . is_err () { return Err (pg_crud :: QpEr :: WriteIntoBuffer { loc : loc_lib :: loc ! () }) ; }
                    if {
                        use std::fmt::Write as _;
                        write!(
                            acc,
                            "jsonb_build_object('field_0',{})||",
                            match <pg_crud::I8AsNnJsonbNbr as pg_crud::PgJson>::sel_only_ids_qp(
                                "elem->'field_0'"
                            ) {
                                Ok(v_2317e0af) => v_2317e0af,
                                Err(er) => {
                                    return Err(er);
                                }
                            }
                        )
                    }
                    .is_err()
                    {
                        return Err(pg_crud::QpEr::WriteIntoBuffer {
                            loc: loc_lib::loc!(),
                        });
                    }
                    if {
                        use std::fmt::Write as _;
                        write!(
                            acc,
                            "jsonb_build_object('field_1',{})||",
                            match <pg_crud::OptI8AsNlJsonbNbr as pg_crud::PgJson>::sel_only_ids_qp(
                                "elem->'field_1'"
                            ) {
                                Ok(v_2317e0af) => v_2317e0af,
                                Err(er) => {
                                    return Err(er);
                                }
                            }
                        )
                    }
                    .is_err()
                    {
                        return Err(pg_crud::QpEr::WriteIntoBuffer {
                            loc: loc_lib::loc!(),
                        });
                    }
                    if { use std :: fmt :: Write as _ ; write ! (acc , "jsonb_build_object('field_2',{})||" , match < pg_crud :: VecOfI8AsNnArrOfNnJsonbNbr as pg_crud :: PgJson > :: sel_only_ids_qp ("elem->'field_2'") { Ok (v_2317e0af) => v_2317e0af , Err (er) => { return Err (er) ; } }) } . is_err () { return Err (pg_crud :: QpEr :: WriteIntoBuffer { loc : loc_lib :: loc ! () }) ; }
                    let _: Option<char> = acc.pop();
                    let _: Option<char> = acc.pop();
                    format!("jsonb_build_object('v',{acc})")
                }
            ))
        }
        type RdInn = VecOfObjExampleWithIdAsNnArrOfNnJsonbObjWithIdRdInn;
        fn into_inn(v: Self::Rd) -> Self::RdInn {
            v.0.into_iter()
                .map(|el_34d57236| ObjExampleWithIdAsNnJsonbObjWithIdRdInn {
                    id: el_34d57236.id.map(|v_6e5af985| pg_crud::V {
                        v: <pg_crud::UuidUuidAsNnJsonbString as pg_crud::PgJson>::into_inn(
                            v_6e5af985.v,
                        ),
                    }),
                    field_0: el_34d57236.field_0.map(|v_6e5af985| pg_crud::V {
                        v: <pg_crud::I8AsNnJsonbNbr as pg_crud::PgJson>::into_inn(v_6e5af985.v),
                    }),
                    field_1: el_34d57236.field_1.map(|v_6e5af985| pg_crud::V {
                        v: <pg_crud::OptI8AsNlJsonbNbr as pg_crud::PgJson>::into_inn(v_6e5af985.v),
                    }),
                    field_2: el_34d57236.field_2.map(|v_6e5af985| pg_crud::V {
                        v: <pg_crud::VecOfI8AsNnArrOfNnJsonbNbr as pg_crud::PgJson>::into_inn(
                            v_6e5af985.v,
                        ),
                    }),
                })
                .collect()
        }
        type Upd = VecOfObjExampleWithIdAsNnArrOfNnJsonbObjWithIdUpd;
        type UpdForQuery = VecOfObjExampleWithIdAsNnArrOfNnJsonbObjWithIdUpdForQuery;
        fn upd_qp(
            v: &Self::UpdForQuery,
            jsonb_set_accumulator: &str,
            jsonb_set_target: &str,
            jsonb_set_path: &str,
            incr: &mut u64,
        ) -> Result<String, pg_crud::QpEr> {
            let _: &str = jsonb_set_accumulator;
            let _: &str = jsonb_set_path;
            let v_58d685d3 = v;
            let upd_qp_acc = {
                if v_58d685d3.upd.is_empty() {
                    String::from("elem")
                } else {
                    let mut acc_2e2ad041 = String::default();
                    for el in v_58d685d3.upd.to_vec() {
                        let ident_with_id_h = {
                            let id_incr = match < pg_crud :: UuidUuidAsNnJsonbString as pg_crud :: PgJsonObjVecElId > :: incr_checked_add_one (incr) { Ok (v_15b44b54) => v_15b44b54 , Err (er) => { return Err (er) ; } } ;
                            match <ObjExampleAsNnJsonbObj as pg_crud::PgJson>::upd_qp(
                                &el.fields, "", "elem", "", incr,
                            ) {
                                Ok(v_56c44461) => {
                                    Ok(format!("when elem->>'id' = ${id_incr} then {v_56c44461} "))
                                }
                                Err(er) => Err(er),
                            }
                        };
                        match ident_with_id_h {
                            Ok(v_8333f8f4) => {
                                if {
                                    use std::fmt::Write as _;
                                    write!(acc_2e2ad041, "{v_8333f8f4}")
                                }
                                .is_err()
                                {
                                    return Err(pg_crud::QpEr::WriteIntoBuffer {
                                        loc: loc_lib::loc!(),
                                    });
                                }
                            }
                            Err(er) => {
                                return Err(er);
                            }
                        }
                    }
                    let _: Option<char> = acc_2e2ad041.pop();
                    format!("case {acc_2e2ad041} else elem end")
                }
            };
            let del_qp_acc = {
                let mut acc_5b4cd920 = String::default();
                for _ in &v_58d685d3.del {
                    let incr_cb6ba4a7 = match < pg_crud :: UuidUuidAsNnJsonbString as pg_crud :: PgJsonObjVecElId > :: incr_checked_add_one (incr) { Ok (v_110650cc) => v_110650cc , Err (er) => { return Err (er) ; } } ;
                    let mb_space_and_space = if acc_5b4cd920.is_empty() { "" } else { " and " };
                    if {
                        use std::fmt::Write as _;
                        write!(
                            acc_5b4cd920,
                            "{mb_space_and_space}elem->>'id' <> ${incr_cb6ba4a7}"
                        )
                    }
                    .is_err()
                    {
                        return Err(pg_crud::QpEr::WriteIntoBuffer {
                            loc: loc_lib::loc!(),
                        });
                    }
                }
                acc_5b4cd920
            };
            let cr_qp_acc = {
                let mut acc_8554f572 = String::default();
                for _ in &v_58d685d3.cr {
                    let incr_5bbc4961 = match < pg_crud :: UuidUuidAsNnJsonbString as pg_crud :: PgJsonObjVecElId > :: incr_checked_add_one (incr) { Ok (v_27487842) => v_27487842 , Err (er) => { return Err (er) ; } } ;
                    if {
                        use std::fmt::Write as _;
                        write!(acc_8554f572, "${incr_5bbc4961},")
                    }
                    .is_err()
                    {
                        return Err(pg_crud::QpEr::WriteIntoBuffer {
                            loc: loc_lib::loc!(),
                        });
                    }
                }
                let _: Option<char> = acc_8554f572.pop();
                acc_8554f572
            };
            let mb_wh = if v_58d685d3.del.is_empty() {
                String::default()
            } else {
                format!(" where {del_qp_acc}")
            };
            let mb_jsonb_build_arr = if v_58d685d3.cr.is_empty() {
                String::default()
            } else {
                format!(" || jsonb_build_arr({cr_qp_acc})")
            };
            Ok(format!(
                "jsonb_set({jsonb_set_accumulator},'{{{jsonb_set_path}}}',case when jsonb_typeof({jsonb_set_target}) = 'null' then '[]'::jsonb else (select coalesce((select jsonb_agg({upd_qp_acc}) from jsonb_array_elements({jsonb_set_target}) as elem {mb_wh}),'[]'::jsonb)) end {mb_jsonb_build_arr})"
            ))
        }
        fn upd_qb(
            v: Self::UpdForQuery,
            mut query: sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>,
        ) -> Result<sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>, String>
        {
            for el in v.upd.into_vec() {
                match < pg_crud :: UuidUuidAsNnJsonbString as pg_crud :: PgJsonObjVecElId > :: qb_string_as_pg_text_upd_for_query (el . id , query) { Ok (v_7633dc9b) => { query = v_7633dc9b ; } Err (er) => { return Err (er) ; } }
                match <ObjExampleAsNnJsonbObj as pg_crud::PgJson>::upd_qb(el.fields, query) {
                    Ok(v_2073f07a) => {
                        query = v_2073f07a;
                    }
                    Err(er) => {
                        return Err(er);
                    }
                }
            }
            for el in v.del {
                match < pg_crud :: UuidUuidAsNnJsonbString as pg_crud :: PgJsonObjVecElId > :: qb_string_as_pg_text_upd_for_query (el , query) { Ok (v_31262d92) => { query = v_31262d92 ; } Err (er) => { return Err (er) ; } }
            }
            for el in v.cr {
                if let Err(er) = query.try_bind(sqlx::types::Json(el)) {
                    return Err(er.to_string());
                }
            }
            Ok(query)
        }
        fn sel_only_updd_ids_qp(
            v: &Self::UpdForQuery,
            fi: &str,
            col_field: &str,
            incr: &mut u64,
        ) -> Result<String, pg_crud::QpEr> {
            match v.sel_only_updd_ids_qp(&format!("{col_field}->'{fi}'"), incr) {
                Ok(v_e137951b) => Ok(format!("'{fi}',jsonb_build_object('v',{v_e137951b}),")),
                Err(er) => Err(er),
            }
        }
        fn sel_only_updd_ids_qb<'lt>(
            v: &'lt Self::UpdForQuery,
            mut query: sqlx::query::Query<'lt, sqlx::Postgres, sqlx::postgres::PgArguments>,
        ) -> Result<sqlx::query::Query<'lt, sqlx::Postgres, sqlx::postgres::PgArguments>, String>
        {
            for el in v.upd.to_vec() {
                match <pg_crud::UuidUuidAsNnJsonbString as pg_crud::PgJson>::sel_only_updd_ids_qb(
                    &el.id, query,
                ) {
                    Ok(v_0fd735de) => {
                        query = v_0fd735de;
                    }
                    Err(er) => {
                        return Err(er);
                    }
                }
                match <ObjExampleAsNnJsonbObj as pg_crud::PgJson>::sel_only_updd_ids_qb(
                    &el.fields, query,
                ) {
                    Ok(v_4b52fa4f) => {
                        query = v_4b52fa4f;
                    }
                    Err(er) => {
                        return Err(er);
                    }
                }
            }
            for el_5fba4c1f in &v.cr {
                match <pg_crud::UuidUuidAsNnJsonbString as pg_crud::PgJson>::sel_only_crd_ids_qb(
                    &el_5fba4c1f.id,
                    query,
                ) {
                    Ok(v_cb81ec2c) => {
                        query = v_cb81ec2c;
                    }
                    Err(er) => {
                        return Err(er);
                    }
                }
                match <pg_crud::I8AsNnJsonbNbr as pg_crud::PgJson>::sel_only_crd_ids_qb(
                    &el_5fba4c1f.field_0,
                    query,
                ) {
                    Ok(v_cb81ec2c) => {
                        query = v_cb81ec2c;
                    }
                    Err(er) => {
                        return Err(er);
                    }
                }
                match <pg_crud::OptI8AsNlJsonbNbr as pg_crud::PgJson>::sel_only_crd_ids_qb(
                    &el_5fba4c1f.field_1,
                    query,
                ) {
                    Ok(v_cb81ec2c) => {
                        query = v_cb81ec2c;
                    }
                    Err(er) => {
                        return Err(er);
                    }
                }
                match <pg_crud::VecOfI8AsNnArrOfNnJsonbNbr as pg_crud::PgJson>::sel_only_crd_ids_qb(
                    &el_5fba4c1f.field_2,
                    query,
                ) {
                    Ok(v_cb81ec2c) => {
                        query = v_cb81ec2c;
                    }
                    Err(er) => {
                        return Err(er);
                    }
                }
            }
            for el in v.upd.to_vec() {
                match < pg_crud :: UuidUuidAsNnJsonbString as pg_crud :: PgJsonObjVecElId > :: qb_string_as_pg_text_upd_for_query (el . id . clone () , query) { Ok (v_b0da764b) => { query = v_b0da764b ; } Err (er) => { return Err (er) ; } }
            }
            for el in &v.cr {
                match < pg_crud :: UuidUuidAsNnJsonbString as pg_crud :: PgJsonObjVecElId > :: qb_string_as_pg_text_cr_for_query (el . id . clone () , query) { Ok (v_dd8932e8) => { query = v_dd8932e8 ; } Err (er) => { return Err (er) ; } }
            }
            Ok(query)
        }
        fn sel_only_crd_ids_qp(
            v: &Self::CrForQuery,
            fi: &str,
            col_field: &str,
            incr: &mut u64,
        ) -> Result<String, pg_crud::QpEr> {
            Ok(format!(
                "'{fi}',jsonb_build_object('v',(select jsonb_agg({}) from jsonb_array_elements({}) as elem where elem->>'id' in ({}))),",
                {
                    let mut acc_0f2b92d0 = String::new();
                    for el in &v.0 {
                        match < pg_crud :: UuidUuidAsNnJsonbString as pg_crud :: PgJson > :: sel_only_crd_ids_qp (& el . id , "id" , "elem" , incr) { Ok (mut v_6d76c065) => { let _ : Option < char > = v_6d76c065 . pop () ; if { use std :: fmt :: Write as _ ; write ! (acc_0f2b92d0 , "jsonb_build_object({v_6d76c065})||") } . is_err () { return Err (pg_crud :: QpEr :: WriteIntoBuffer { loc : loc_lib :: loc ! () }) ; } } , Err (er) => { return Err (er) ; } }
                        match <pg_crud::I8AsNnJsonbNbr as pg_crud::PgJson>::sel_only_crd_ids_qp(
                            &el.field_0,
                            "field_0",
                            "elem",
                            incr,
                        ) {
                            Ok(mut v_6d76c065) => {
                                let _: Option<char> = v_6d76c065.pop();
                                if {
                                    use std::fmt::Write as _;
                                    write!(acc_0f2b92d0, "jsonb_build_object({v_6d76c065})||")
                                }
                                .is_err()
                                {
                                    return Err(pg_crud::QpEr::WriteIntoBuffer {
                                        loc: loc_lib::loc!(),
                                    });
                                }
                            }
                            Err(er) => {
                                return Err(er);
                            }
                        }
                        match <pg_crud::OptI8AsNlJsonbNbr as pg_crud::PgJson>::sel_only_crd_ids_qp(
                            &el.field_1,
                            "field_1",
                            "elem",
                            incr,
                        ) {
                            Ok(mut v_6d76c065) => {
                                let _: Option<char> = v_6d76c065.pop();
                                if {
                                    use std::fmt::Write as _;
                                    write!(acc_0f2b92d0, "jsonb_build_object({v_6d76c065})||")
                                }
                                .is_err()
                                {
                                    return Err(pg_crud::QpEr::WriteIntoBuffer {
                                        loc: loc_lib::loc!(),
                                    });
                                }
                            }
                            Err(er) => {
                                return Err(er);
                            }
                        }
                        match < pg_crud :: VecOfI8AsNnArrOfNnJsonbNbr as pg_crud :: PgJson > :: sel_only_crd_ids_qp (& el . field_2 , "field_2" , "elem" , incr) { Ok (mut v_6d76c065) => { let _ : Option < char > = v_6d76c065 . pop () ; if { use std :: fmt :: Write as _ ; write ! (acc_0f2b92d0 , "jsonb_build_object({v_6d76c065})||") } . is_err () { return Err (pg_crud :: QpEr :: WriteIntoBuffer { loc : loc_lib :: loc ! () }) ; } } , Err (er) => { return Err (er) ; } }
                    }
                    let _: Option<char> = acc_0f2b92d0.pop();
                    let _: Option<char> = acc_0f2b92d0.pop();
                    format!("jsonb_build_object('v',{acc_0f2b92d0})")
                },
                &format!("{col_field}->'{fi}'"),
                {
                    let mut acc_44b1f772 = String::new();
                    for _ in &v.0 {
                        match pg_crud::incr_checked_add_one_returning_incr(incr) {
                            Ok(v_73b58d3a) => {
                                if {
                                    use std::fmt::Write as _;
                                    write!(acc_44b1f772, "${v_73b58d3a},")
                                }
                                .is_err()
                                {
                                    return Err(pg_crud::QpEr::WriteIntoBuffer {
                                        loc: loc_lib::loc!(),
                                    });
                                }
                            }
                            Err(er) => {
                                return Err(er);
                            }
                        }
                    }
                    let _: Option<char> = acc_44b1f772.pop();
                    acc_44b1f772
                }
            ))
        }
        fn sel_only_crd_ids_qb<'lt>(
            v: &'lt Self::CrForQuery,
            mut query: sqlx::query::Query<'lt, sqlx::Postgres, sqlx::postgres::PgArguments>,
        ) -> Result<sqlx::query::Query<'lt, sqlx::Postgres, sqlx::postgres::PgArguments>, String>
        {
            for el_9bdcd847 in &v.0 {
                match <pg_crud::UuidUuidAsNnJsonbString as pg_crud::PgJson>::sel_only_crd_ids_qb(
                    &el_9bdcd847.id,
                    query,
                ) {
                    Ok(v_ade27463) => {
                        query = v_ade27463;
                    }
                    Err(er) => {
                        return Err(er);
                    }
                }
                match <pg_crud::I8AsNnJsonbNbr as pg_crud::PgJson>::sel_only_crd_ids_qb(
                    &el_9bdcd847.field_0,
                    query,
                ) {
                    Ok(v_ade27463) => {
                        query = v_ade27463;
                    }
                    Err(er) => {
                        return Err(er);
                    }
                }
                match <pg_crud::OptI8AsNlJsonbNbr as pg_crud::PgJson>::sel_only_crd_ids_qb(
                    &el_9bdcd847.field_1,
                    query,
                ) {
                    Ok(v_ade27463) => {
                        query = v_ade27463;
                    }
                    Err(er) => {
                        return Err(er);
                    }
                }
                match <pg_crud::VecOfI8AsNnArrOfNnJsonbNbr as pg_crud::PgJson>::sel_only_crd_ids_qb(
                    &el_9bdcd847.field_2,
                    query,
                ) {
                    Ok(v_ade27463) => {
                        query = v_ade27463;
                    }
                    Err(er) => {
                        return Err(er);
                    }
                }
            }
            for el in &v.0 {
                match < pg_crud :: UuidUuidAsNnJsonbString as pg_crud :: PgJsonObjVecElId > :: qb_string_as_pg_text_cr_for_query (el . id . clone () , query) { Ok (v_a3749ea8) => { query = v_a3749ea8 ; } Err (er) => { return Err (er) ; } }
            }
            Ok(query)
        }
    }
    #[allow(clippy::arbitrary_source_item_ordering)]
    impl pg_crud::PgType for VecOfObjExampleWithIdAsNnArrOfNnJsonbObjWithId {
        type Tt = VecOfObjExampleWithIdAsNnArrOfNnJsonbObjWithIdTt;
        fn cr_tbl_col_qp(col: &dyn std::fmt::Display, _: bool) -> impl std::fmt::Display {
            format!(
                "{col} jsonb not null check (jsonb_matches_schema('{}', {col}))",
                serde_json::to_string(&schemars::schema_for!(
                    VecOfObjExampleWithIdAsNnArrOfNnJsonbObjWithIdTt
                ))
                .expect("59a1654b")
            )
        }
        type Cr = VecOfObjExampleWithIdAsNnArrOfNnJsonbObjWithIdCr;
        fn cr_qp(_: &Self::Cr, incr: &mut u64) -> Result<String, pg_crud::QpEr> {
            match pg_crud::incr_checked_add_one_returning_incr(incr) {
                Ok(v_7df9eb00) => Ok(format!("${v_7df9eb00}")),
                Err(er) => Err(er),
            }
        }
        fn cr_qb(
            v: Self::Cr,
            mut query: sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>,
        ) -> Result<sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>, String>
        {
            if let Err(er) = query.try_bind(<Self as pg_crud::PgJson>::CrForQuery::from(v)) {
                return Err(er.to_string());
            }
            Ok(query)
        }
        type Sel = VecOfObjExampleWithIdAsNnArrOfNnJsonbObjWithIdSel;
        fn sel_qp(v: &Self::Sel, col: &str) -> Result<String, pg_crud::QpEr> {
            match v.sel_qp_pg_type(col) {
                Ok(v_d91c19a6) => Ok(format!("{v_d91c19a6} as {col}")),
                Err(er) => Err(er),
            }
        }
        type Wh = VecOfObjExampleWithIdAsNnArrOfNnJsonbObjWithIdWh;
        type Rd = VecOfObjExampleWithIdAsNnArrOfNnJsonbObjWithIdRd;
        fn normalize(v: Self::Rd) -> Self::Rd {
            v
        }
        type RdIds = VecOfObjExampleWithIdAsNnArrOfNnJsonbObjWithIdRdIds;
        fn sel_only_ids_qp(col: &str) -> Result<String, pg_crud::QpEr> {
            match <Self as pg_crud::PgJson>::sel_only_ids_qp(col) {
                Ok(v_e776e9fa) => Ok(format!("{v_e776e9fa} as {col},")),
                Err(er) => Err(er),
            }
        }
        type RdInn = VecOfObjExampleWithIdAsNnArrOfNnJsonbObjWithIdRdInn;
        fn into_inn(v: Self::Rd) -> Self::RdInn {
            <Self as pg_crud::PgJson>::into_inn(v)
        }
        type Upd = VecOfObjExampleWithIdAsNnArrOfNnJsonbObjWithIdUpd;
        type UpdForQuery = VecOfObjExampleWithIdAsNnArrOfNnJsonbObjWithIdUpdForQuery;
        #[allow(unused_variables)]
        fn upd_qp(
            v: &Self::UpdForQuery,
            jsonb_set_accumulator: &str,
            jsonb_set_target: &str,
            jsonb_set_path: &str,
            incr: &mut u64,
        ) -> Result<String, pg_crud::QpEr> {
            let v_58d685d3 = v;
            let upd_qp_acc = {
                if v_58d685d3.upd.is_empty() {
                    String::from("elem")
                } else {
                    let mut acc_2e2ad041 = String::default();
                    for el in v_58d685d3.upd.to_vec() {
                        let ident_with_id_h = {
                            let id_incr = match < pg_crud :: UuidUuidAsNnJsonbString as pg_crud :: PgJsonObjVecElId > :: incr_checked_add_one (incr) { Ok (v_15b44b54) => v_15b44b54 , Err (er) => { return Err (er) ; } } ;
                            match <ObjExampleAsNnJsonbObj as pg_crud::PgJson>::upd_qp(
                                &el.fields, "", "elem", "", incr,
                            ) {
                                Ok(v_56c44461) => {
                                    Ok(format!("when elem->>'id' = ${id_incr} then {v_56c44461} "))
                                }
                                Err(er) => Err(er),
                            }
                        };
                        match ident_with_id_h {
                            Ok(v_8333f8f4) => {
                                if {
                                    use std::fmt::Write as _;
                                    write!(acc_2e2ad041, "{v_8333f8f4}")
                                }
                                .is_err()
                                {
                                    return Err(pg_crud::QpEr::WriteIntoBuffer {
                                        loc: loc_lib::loc!(),
                                    });
                                }
                            }
                            Err(er) => {
                                return Err(er);
                            }
                        }
                    }
                    let _: Option<char> = acc_2e2ad041.pop();
                    format!("case {acc_2e2ad041} else elem end")
                }
            };
            let del_qp_acc = {
                let mut acc_5b4cd920 = String::default();
                for _ in &v_58d685d3.del {
                    let incr_cb6ba4a7 = match < pg_crud :: UuidUuidAsNnJsonbString as pg_crud :: PgJsonObjVecElId > :: incr_checked_add_one (incr) { Ok (v_110650cc) => v_110650cc , Err (er) => { return Err (er) ; } } ;
                    let mb_space_and_space = if acc_5b4cd920.is_empty() { "" } else { " and " };
                    if {
                        use std::fmt::Write as _;
                        write!(
                            acc_5b4cd920,
                            "{mb_space_and_space}elem->>'id' <> ${incr_cb6ba4a7}"
                        )
                    }
                    .is_err()
                    {
                        return Err(pg_crud::QpEr::WriteIntoBuffer {
                            loc: loc_lib::loc!(),
                        });
                    }
                }
                acc_5b4cd920
            };
            let cr_qp_acc = {
                let mut acc_8554f572 = String::default();
                for _ in &v_58d685d3.cr {
                    let incr_5bbc4961 = match < pg_crud :: UuidUuidAsNnJsonbString as pg_crud :: PgJsonObjVecElId > :: incr_checked_add_one (incr) { Ok (v_27487842) => v_27487842 , Err (er) => { return Err (er) ; } } ;
                    if {
                        use std::fmt::Write as _;
                        write!(acc_8554f572, "${incr_5bbc4961},")
                    }
                    .is_err()
                    {
                        return Err(pg_crud::QpEr::WriteIntoBuffer {
                            loc: loc_lib::loc!(),
                        });
                    }
                }
                let _: Option<char> = acc_8554f572.pop();
                acc_8554f572
            };
            let mb_wh = if v_58d685d3.del.is_empty() {
                String::default()
            } else {
                format!(" where {del_qp_acc}")
            };
            let mb_jsonb_build_arr = if v_58d685d3.cr.is_empty() {
                String::default()
            } else {
                format!(" || jsonb_build_arr({cr_qp_acc})")
            };
            Ok(format!(
                "((select coalesce((select jsonb_agg({upd_qp_acc}) from jsonb_array_elements({jsonb_set_target}) as elem {mb_wh}),'[]'::jsonb)) {mb_jsonb_build_arr})"
            ))
        }
        fn upd_qb(
            v: Self::UpdForQuery,
            query: sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>,
        ) -> Result<sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>, String>
        {
            <Self as pg_crud::PgJson>::upd_qb(v, query)
        }
        fn sel_only_updd_ids_qp(
            v: &Self::UpdForQuery,
            col: &str,
            incr: &mut u64,
        ) -> Result<String, pg_crud::QpEr> {
            match v.sel_only_updd_ids_qp(col, incr) {
                Ok(v_f0787243) => Ok(format!("jsonb_build_object('v',{v_f0787243}) as {col},")),
                Err(er) => Err(er),
            }
        }
        fn sel_only_updd_ids_qb<'lt>(
            v: &'lt Self::UpdForQuery,
            query: sqlx::query::Query<'lt, sqlx::Postgres, sqlx::postgres::PgArguments>,
        ) -> Result<sqlx::query::Query<'lt, sqlx::Postgres, sqlx::postgres::PgArguments>, String>
        {
            <Self as pg_crud::PgJson>::sel_only_updd_ids_qb(v, query)
        }
    }
    #[allow(unused_qualifications)]
    #[allow(clippy::absolute_paths)]
    #[allow(clippy::arbitrary_source_item_ordering)]
    #[cfg(feature = "test-utils")]
    #[allow(clippy::float_arithmetic)]
    impl pg_crud::PgJsonTestCases for VecOfObjExampleWithIdAsNnArrOfNnJsonbObjWithId {
        type PgJson = Self;
        type Sel = VecOfObjExampleWithIdAsNnArrOfNnJsonbObjWithIdSel;
        fn opt_vec_cr() -> Option<Vec<<Self::PgJson as pg_crud::PgJson>::Cr>> {
            Some({
                let mut acc_ccd79a32 = Vec::new();
                if let Some(vec_cr) =
                    <pg_crud::I8AsNnJsonbNbr as pg_crud::PgJsonTestCases>::opt_vec_cr()
                {
                    let mut acc_6a886d56 = Vec::new();
                    let opt_extra = {
                        let mut opt_extra = None;
                        for el_37154498 in &vec_cr {
                            if opt_extra.is_none() {
                                let v = ObjExampleWithIdAsNnJsonbObjWithIdCr::new(
                                    el_37154498.clone(),
                                    pg_crud::DfltSomeOneEl::dflt_some_one_el(),
                                    pg_crud::DfltSomeOneEl::dflt_some_one_el(),
                                );
                                opt_extra = Some((
                                    VecOfObjExampleWithIdAsNnArrOfNnJsonbObjWithIdCr::new(vec![
                                        v.clone(),
                                    ]),
                                    VecOfObjExampleWithIdAsNnArrOfNnJsonbObjWithIdCr::new(vec![
                                        v.clone(),
                                        v,
                                    ]),
                                ));
                            } else {
                                break;
                            }
                        }
                        opt_extra
                    };
                    let has_len_greater_than_one = vec_cr.len() > 1;
                    for el_37154498 in vec_cr {
                        acc_6a886d56.push(ObjExampleWithIdAsNnJsonbObjWithIdCr::new(
                            el_37154498,
                            pg_crud::DfltSomeOneEl::dflt_some_one_el(),
                            pg_crud::DfltSomeOneEl::dflt_some_one_el(),
                        ));
                    }
                    {
                        let v_07c0c08c =
                            VecOfObjExampleWithIdAsNnArrOfNnJsonbObjWithIdCr::new(acc_6a886d56);
                        if !acc_ccd79a32.contains(&v_07c0c08c) {
                            acc_ccd79a32.push(v_07c0c08c);
                        }
                    }
                    if let Some(v_f6686d5d) = opt_extra {
                        if has_len_greater_than_one {
                            let v_60116463 = v_f6686d5d.0;
                            if !acc_ccd79a32.contains(&v_60116463) {
                                acc_ccd79a32.push(v_60116463);
                            }
                        } else {
                            let v_7a843059 = v_f6686d5d.1;
                            if !acc_ccd79a32.contains(&v_7a843059) {
                                acc_ccd79a32.push(v_7a843059);
                            }
                        }
                    }
                }
                if let Some(vec_cr) =
                    <pg_crud::OptI8AsNlJsonbNbr as pg_crud::PgJsonTestCases>::opt_vec_cr()
                {
                    let mut acc_6a886d56 = Vec::new();
                    let opt_extra = {
                        let mut opt_extra = None;
                        for el_37154498 in &vec_cr {
                            if opt_extra.is_none() {
                                let v = ObjExampleWithIdAsNnJsonbObjWithIdCr::new(
                                    pg_crud::DfltSomeOneEl::dflt_some_one_el(),
                                    el_37154498.clone(),
                                    pg_crud::DfltSomeOneEl::dflt_some_one_el(),
                                );
                                opt_extra = Some((
                                    VecOfObjExampleWithIdAsNnArrOfNnJsonbObjWithIdCr::new(vec![
                                        v.clone(),
                                    ]),
                                    VecOfObjExampleWithIdAsNnArrOfNnJsonbObjWithIdCr::new(vec![
                                        v.clone(),
                                        v,
                                    ]),
                                ));
                            } else {
                                break;
                            }
                        }
                        opt_extra
                    };
                    let has_len_greater_than_one = vec_cr.len() > 1;
                    for el_37154498 in vec_cr {
                        acc_6a886d56.push(ObjExampleWithIdAsNnJsonbObjWithIdCr::new(
                            pg_crud::DfltSomeOneEl::dflt_some_one_el(),
                            el_37154498,
                            pg_crud::DfltSomeOneEl::dflt_some_one_el(),
                        ));
                    }
                    {
                        let v_07c0c08c =
                            VecOfObjExampleWithIdAsNnArrOfNnJsonbObjWithIdCr::new(acc_6a886d56);
                        if !acc_ccd79a32.contains(&v_07c0c08c) {
                            acc_ccd79a32.push(v_07c0c08c);
                        }
                    }
                    if let Some(v_f6686d5d) = opt_extra {
                        if has_len_greater_than_one {
                            let v_60116463 = v_f6686d5d.0;
                            if !acc_ccd79a32.contains(&v_60116463) {
                                acc_ccd79a32.push(v_60116463);
                            }
                        } else {
                            let v_7a843059 = v_f6686d5d.1;
                            if !acc_ccd79a32.contains(&v_7a843059) {
                                acc_ccd79a32.push(v_7a843059);
                            }
                        }
                    }
                }
                if let Some(vec_cr) =
                    <pg_crud::VecOfI8AsNnArrOfNnJsonbNbr as pg_crud::PgJsonTestCases>::opt_vec_cr()
                {
                    let mut acc_6a886d56 = Vec::new();
                    let opt_extra = {
                        let mut opt_extra = None;
                        for el_37154498 in &vec_cr {
                            if opt_extra.is_none() {
                                let v = ObjExampleWithIdAsNnJsonbObjWithIdCr::new(
                                    pg_crud::DfltSomeOneEl::dflt_some_one_el(),
                                    pg_crud::DfltSomeOneEl::dflt_some_one_el(),
                                    el_37154498.clone(),
                                );
                                opt_extra = Some((
                                    VecOfObjExampleWithIdAsNnArrOfNnJsonbObjWithIdCr::new(vec![
                                        v.clone(),
                                    ]),
                                    VecOfObjExampleWithIdAsNnArrOfNnJsonbObjWithIdCr::new(vec![
                                        v.clone(),
                                        v,
                                    ]),
                                ));
                            } else {
                                break;
                            }
                        }
                        opt_extra
                    };
                    let has_len_greater_than_one = vec_cr.len() > 1;
                    for el_37154498 in vec_cr {
                        acc_6a886d56.push(ObjExampleWithIdAsNnJsonbObjWithIdCr::new(
                            pg_crud::DfltSomeOneEl::dflt_some_one_el(),
                            pg_crud::DfltSomeOneEl::dflt_some_one_el(),
                            el_37154498,
                        ));
                    }
                    {
                        let v_07c0c08c =
                            VecOfObjExampleWithIdAsNnArrOfNnJsonbObjWithIdCr::new(acc_6a886d56);
                        if !acc_ccd79a32.contains(&v_07c0c08c) {
                            acc_ccd79a32.push(v_07c0c08c);
                        }
                    }
                    if let Some(v_f6686d5d) = opt_extra {
                        if has_len_greater_than_one {
                            let v_60116463 = v_f6686d5d.0;
                            if !acc_ccd79a32.contains(&v_60116463) {
                                acc_ccd79a32.push(v_60116463);
                            }
                        } else {
                            let v_7a843059 = v_f6686d5d.1;
                            if !acc_ccd79a32.contains(&v_7a843059) {
                                acc_ccd79a32.push(v_7a843059);
                            }
                        }
                    }
                }
                acc_ccd79a32
            })
        }
        fn rd_ids_to_2_dims_vec_rd_inn(
            rd_ids: &<Self::PgJson as pg_crud::PgJson>::RdIds,
        ) -> Vec<Vec<<Self::PgJson as pg_crud::PgJson>::RdInn>> {
            rd_ids . 0. v . iter () . map (| el | { let mut acc_00b3df88 = Vec :: new () ; for el_4b4da5aa in < pg_crud :: I8AsNnJsonbNbr as pg_crud :: PgJsonTestCases > :: rd_ids_to_2_dims_vec_rd_inn (& el . 0. v . field_0 . clone ()) { for el_18d1f553 in el_4b4da5aa { acc_00b3df88 . push (vec ! [ObjExampleWithIdAsNnJsonbObjWithIdRdInn { id : Some (pg_crud :: V { v : el . 0. v . id . 0. v }) , field_0 : Some (pg_crud :: V { v : el_18d1f553 }) , field_1 : < pg_crud :: OptI8AsNlJsonbNbr as pg_crud :: PgJsonTestCases > :: rd_ids_into_opt_v_rd_inn (el . 0. v . field_1 . clone ()) , field_2 : < pg_crud :: VecOfI8AsNnArrOfNnJsonbNbr as pg_crud :: PgJsonTestCases > :: rd_ids_into_opt_v_rd_inn (el . 0. v . field_2 . clone ()) }]) ; } } for el_4b4da5aa in < pg_crud :: OptI8AsNlJsonbNbr as pg_crud :: PgJsonTestCases > :: rd_ids_to_2_dims_vec_rd_inn (& el . 0. v . field_1 . clone ()) { for el_18d1f553 in el_4b4da5aa { acc_00b3df88 . push (vec ! [ObjExampleWithIdAsNnJsonbObjWithIdRdInn { id : Some (pg_crud :: V { v : el . 0. v . id . 0. v }) , field_0 : < pg_crud :: I8AsNnJsonbNbr as pg_crud :: PgJsonTestCases > :: rd_ids_into_opt_v_rd_inn (el . 0. v . field_0 . clone ()) , field_1 : Some (pg_crud :: V { v : el_18d1f553 }) , field_2 : < pg_crud :: VecOfI8AsNnArrOfNnJsonbNbr as pg_crud :: PgJsonTestCases > :: rd_ids_into_opt_v_rd_inn (el . 0. v . field_2 . clone ()) }]) ; } } for el_4b4da5aa in < pg_crud :: VecOfI8AsNnArrOfNnJsonbNbr as pg_crud :: PgJsonTestCases > :: rd_ids_to_2_dims_vec_rd_inn (& el . 0. v . field_2 . clone ()) { for el_18d1f553 in el_4b4da5aa { acc_00b3df88 . push (vec ! [ObjExampleWithIdAsNnJsonbObjWithIdRdInn { id : Some (pg_crud :: V { v : el . 0. v . id . 0. v }) , field_0 : < pg_crud :: I8AsNnJsonbNbr as pg_crud :: PgJsonTestCases > :: rd_ids_into_opt_v_rd_inn (el . 0. v . field_0 . clone ()) , field_1 : < pg_crud :: OptI8AsNlJsonbNbr as pg_crud :: PgJsonTestCases > :: rd_ids_into_opt_v_rd_inn (el . 0. v . field_1 . clone ()) , field_2 : Some (pg_crud :: V { v : el_18d1f553 }) }]) ; } } acc_00b3df88 }) . collect ()
        }
        fn rd_inn_into_rd_with_new_or_try_new_unwraped(
            v: VecOfObjExampleWithIdAsNnArrOfNnJsonbObjWithIdRdInn,
        ) -> <Self::PgJson as pg_crud::PgJson>::Rd {
            VecOfObjExampleWithIdAsNnArrOfNnJsonbObjWithIdRd :: new (v . into_iter () . map (| el_ffed1bfc | ObjExampleWithIdAsNnJsonbObjWithIdRd { id : el_ffed1bfc . id . map (| v_3ac52220 | pg_crud :: V { v : < pg_crud :: UuidUuidAsNnJsonbString as pg_crud :: PgJsonTestCases > :: rd_inn_into_rd_with_new_or_try_new_unwraped (v_3ac52220 . v) }) , field_0 : el_ffed1bfc . field_0 . map (| v_3ac52220 | pg_crud :: V { v : < pg_crud :: I8AsNnJsonbNbr as pg_crud :: PgJsonTestCases > :: rd_inn_into_rd_with_new_or_try_new_unwraped (v_3ac52220 . v) }) , field_1 : el_ffed1bfc . field_1 . map (| v_3ac52220 | pg_crud :: V { v : < pg_crud :: OptI8AsNlJsonbNbr as pg_crud :: PgJsonTestCases > :: rd_inn_into_rd_with_new_or_try_new_unwraped (v_3ac52220 . v) }) , field_2 : el_ffed1bfc . field_2 . map (| v_3ac52220 | pg_crud :: V { v : < pg_crud :: VecOfI8AsNnArrOfNnJsonbNbr as pg_crud :: PgJsonTestCases > :: rd_inn_into_rd_with_new_or_try_new_unwraped (v_3ac52220 . v) }) }) . collect ())
        }
        fn rd_inn_into_upd_with_new_or_try_new_unwraped(
            v: VecOfObjExampleWithIdAsNnArrOfNnJsonbObjWithIdRdInn,
        ) -> <Self::PgJson as pg_crud::PgJson>::Upd {
            VecOfObjExampleWithIdAsNnArrOfNnJsonbObjWithIdUpd :: try_new (Vec :: new () , pg_crud :: UnqVec :: < ObjExampleWithIdAsNnJsonbObjWithIdUpdEl > :: try_new (v . into_iter () . map (| el_ffed1bfc | ObjExampleWithIdAsNnJsonbObjWithIdUpdEl { id : pg_crud :: UuidUuidAsNnJsonbStringUpd :: new (el_ffed1bfc . id . clone () . expect ("f04a2c6d") . v) , fields : < ObjExampleAsNnJsonbObj as pg_crud :: PgJsonTestCases > :: rd_inn_into_upd_with_new_or_try_new_unwraped (ObjExampleAsNnJsonbObjRdInn { field_0 : el_ffed1bfc . field_0 , field_1 : el_ffed1bfc . field_1 , field_2 : el_ffed1bfc . field_2 }) , }) . collect () ,) . expect ("ca51d559") , Vec :: new () ,) . expect ("0449fe82")
        }
        fn rd_ids_into_opt_v_rd_inn(
            v: <Self::PgJson as pg_crud::PgJson>::RdIds,
        ) -> Option<pg_crud::V<<Self::PgJson as pg_crud::PgJson>::RdInn>> {
            Some (pg_crud :: V { v : v . 0. v . into_iter () . fold (Vec :: new () , | mut acc_cf4743b1 , el_6603f209 | { acc_cf4743b1 . push (ObjExampleWithIdAsNnJsonbObjWithIdRdInn { id : < pg_crud :: UuidUuidAsNnJsonbString as pg_crud :: PgJsonTestCases > :: rd_ids_into_opt_v_rd_inn (el_6603f209 . 0. v . id) . map_or_else (|| Some (pg_crud :: V { v : < pg_crud :: UuidUuidAsNnJsonbString as pg_crud :: PgJson > :: into_inn (< < pg_crud :: UuidUuidAsNnJsonbString as pg_crud :: PgJson > :: Rd as pg_crud :: DfltSomeOneEl > :: dflt_some_one_el ()) }) , Some) , field_0 : < pg_crud :: I8AsNnJsonbNbr as pg_crud :: PgJsonTestCases > :: rd_ids_into_opt_v_rd_inn (el_6603f209 . 0. v . field_0) . map_or_else (|| Some (pg_crud :: V { v : < pg_crud :: I8AsNnJsonbNbr as pg_crud :: PgJson > :: into_inn (< < pg_crud :: I8AsNnJsonbNbr as pg_crud :: PgJson > :: Rd as pg_crud :: DfltSomeOneEl > :: dflt_some_one_el ()) }) , Some) , field_1 : < pg_crud :: OptI8AsNlJsonbNbr as pg_crud :: PgJsonTestCases > :: rd_ids_into_opt_v_rd_inn (el_6603f209 . 0. v . field_1) . map_or_else (|| Some (pg_crud :: V { v : < pg_crud :: OptI8AsNlJsonbNbr as pg_crud :: PgJson > :: into_inn (< < pg_crud :: OptI8AsNlJsonbNbr as pg_crud :: PgJson > :: Rd as pg_crud :: DfltSomeOneEl > :: dflt_some_one_el ()) }) , Some) , field_2 : < pg_crud :: VecOfI8AsNnArrOfNnJsonbNbr as pg_crud :: PgJsonTestCases > :: rd_ids_into_opt_v_rd_inn (el_6603f209 . 0. v . field_2) . map_or_else (|| Some (pg_crud :: V { v : < pg_crud :: VecOfI8AsNnArrOfNnJsonbNbr as pg_crud :: PgJson > :: into_inn (< < pg_crud :: VecOfI8AsNnArrOfNnJsonbNbr as pg_crud :: PgJson > :: Rd as pg_crud :: DfltSomeOneEl > :: dflt_some_one_el ()) }) , Some) }) ; acc_cf4743b1 }) })
        }
        fn upd_to_rd_ids(
            v: &<Self::PgJson as pg_crud::PgJson>::Upd,
        ) -> <Self::PgJson as pg_crud::PgJson>::RdIds {
            VecOfObjExampleWithIdAsNnArrOfNnJsonbObjWithIdRdIds (pg_crud :: V { v : v . upd . to_vec () . iter () . map (| el_4634bb8a | { let mut field_0 = None ; let mut field_1 = None ; let mut field_2 = None ; for el_da177c5a in el_4634bb8a . fields . 0 . to_vec () { match & el_da177c5a { ObjExampleAsNnJsonbObjUpdEl :: Field0 (v_d2a6daf8) => { field_0 = Some (v_d2a6daf8 . v . clone ()) ; } , ObjExampleAsNnJsonbObjUpdEl :: Field1 (_) | ObjExampleAsNnJsonbObjUpdEl :: Field2 (_) => () , } } for el_da177c5a in el_4634bb8a . fields . 0 . to_vec () { match & el_da177c5a { ObjExampleAsNnJsonbObjUpdEl :: Field1 (v_d2a6daf8) => { field_1 = Some (v_d2a6daf8 . v . clone ()) ; } , ObjExampleAsNnJsonbObjUpdEl :: Field0 (_) | ObjExampleAsNnJsonbObjUpdEl :: Field2 (_) => () , } } for el_da177c5a in el_4634bb8a . fields . 0 . to_vec () { match & el_da177c5a { ObjExampleAsNnJsonbObjUpdEl :: Field2 (v_d2a6daf8) => { field_2 = Some (v_d2a6daf8 . v . clone ()) ; } , ObjExampleAsNnJsonbObjUpdEl :: Field0 (_) | ObjExampleAsNnJsonbObjUpdEl :: Field1 (_) => () , } } ObjExampleWithIdAsNnJsonbObjWithIdRdIds (pg_crud :: V { v : ObjExampleWithIdAsNnJsonbObjWithIdRdIdsH { id : < pg_crud :: UuidUuidAsNnJsonbString as pg_crud :: PgJsonTestCases > :: upd_to_rd_ids (& el_4634bb8a . id) , field_0 : < pg_crud :: I8AsNnJsonbNbr as pg_crud :: PgJsonTestCases > :: upd_to_rd_ids (& field_0 . expect ("a3ec7cae")) , field_1 : < pg_crud :: OptI8AsNlJsonbNbr as pg_crud :: PgJsonTestCases > :: upd_to_rd_ids (& field_1 . expect ("a3ec7cae")) , field_2 : < pg_crud :: VecOfI8AsNnArrOfNnJsonbNbr as pg_crud :: PgJsonTestCases > :: upd_to_rd_ids (& field_2 . expect ("a3ec7cae")) } }) }) . collect () })
        }
        fn rd_ids_to_opt_v_rd_dflt_some_one_el(
            v: &<Self::PgJson as pg_crud::PgJson>::RdIds,
        ) -> Option<pg_crud::V<<Self::PgJson as pg_crud::PgJson>::Rd>> {
            Some(pg_crud::V {
                v: VecOfObjExampleWithIdAsNnArrOfNnJsonbObjWithIdRd::new({
                    let mut acc_5f587d35 = v . 0. v . clone () . into_iter () . map (| el_629b1544 | { ObjExampleWithIdAsNnJsonbObjWithIdRd :: try_new (< pg_crud :: UuidUuidAsNnJsonbString as pg_crud :: PgJsonTestCases > :: rd_ids_to_opt_v_rd_dflt_some_one_el (& el_629b1544 . 0. v . id) , < pg_crud :: I8AsNnJsonbNbr as pg_crud :: PgJsonTestCases > :: rd_ids_to_opt_v_rd_dflt_some_one_el (& el_629b1544 . 0. v . field_0) , < pg_crud :: OptI8AsNlJsonbNbr as pg_crud :: PgJsonTestCases > :: rd_ids_to_opt_v_rd_dflt_some_one_el (& el_629b1544 . 0. v . field_1) , < pg_crud :: VecOfI8AsNnArrOfNnJsonbNbr as pg_crud :: PgJsonTestCases > :: rd_ids_to_opt_v_rd_dflt_some_one_el (& el_629b1544 . 0. v . field_2)) . expect ("8f6fb6b6") }) . collect :: < Vec < ObjExampleWithIdAsNnJsonbObjWithIdRd >> () ;
                    acc_5f587d35.sort_by(|first, second| {
                        if let (Some(v_first), Some(v_second)) = (&first.id, &second.id) {
                            <pg_crud::UuidUuidAsNnJsonbString as pg_crud::PgJson>::into_inn(
                                v_first.v.clone(),
                            )
                            .cmp(
                                &<pg_crud::UuidUuidAsNnJsonbString as pg_crud::PgJson>::into_inn(
                                    v_second.v.clone(),
                                ),
                            )
                        } else {
                            panic!("0bdf0f44");
                        }
                    });
                    acc_5f587d35
                }),
            })
        }
        fn previous_rd_and_opt_upd_into_rd(
            rd: <Self::PgJson as pg_crud::PgJson>::Rd,
            opt_upd: Option<<Self::PgJson as pg_crud::PgJson>::Upd>,
        ) -> <Self::PgJson as pg_crud::PgJson>::Rd {
            match opt_upd {
                Some(v_47f5a20b) => VecOfObjExampleWithIdAsNnArrOfNnJsonbObjWithIdRd({
                    let mut acc_04a67ef2 = Vec::new();
                    for el_377d1bb4 in v_47f5a20b.upd.into_vec() {
                        let mut opt_rd_el = None;
                        for el in &rd.0 {
                            if * < pg_crud :: UuidUuidAsNnJsonbString as pg_crud :: PgJsonObjVecElId > :: get_inn (& el_377d1bb4 . id . clone () . into ()) == < pg_crud :: UuidUuidAsNnJsonbString as pg_crud :: PgJson > :: into_inn (el . id . clone () . expect ("df2413fe") . v) { opt_rd_el = Some (el . clone ()) ; break ; }
                        }
                        let found_rd_el = opt_rd_el.expect("139882b9");
                        let mut field_0 = None;
                        let mut field_1 = None;
                        let mut field_2 = None;
                        for el_629b1544 in el_377d1bb4.fields.0.into_vec() {
                            match el_629b1544 {
                                ObjExampleAsNnJsonbObjUpdEl::Field0(v_730a4dde) => {
                                    field_0 = Some(v_730a4dde.v);
                                }
                                ObjExampleAsNnJsonbObjUpdEl::Field1(v_730a4dde) => {
                                    field_1 = Some(v_730a4dde.v);
                                }
                                ObjExampleAsNnJsonbObjUpdEl::Field2(v_730a4dde) => {
                                    field_2 = Some(v_730a4dde.v);
                                }
                            }
                        }
                        acc_04a67ef2 . push (ObjExampleWithIdAsNnJsonbObjWithIdRd { id : found_rd_el . id , field_0 : Some (pg_crud :: V { v : < pg_crud :: I8AsNnJsonbNbr as pg_crud :: PgJsonTestCases > :: previous_rd_and_opt_upd_into_rd (found_rd_el . field_0 . expect ("2e8229f7") . v , field_0) }) , field_1 : Some (pg_crud :: V { v : < pg_crud :: OptI8AsNlJsonbNbr as pg_crud :: PgJsonTestCases > :: previous_rd_and_opt_upd_into_rd (found_rd_el . field_1 . expect ("2e8229f7") . v , field_1) }) , field_2 : Some (pg_crud :: V { v : < pg_crud :: VecOfI8AsNnArrOfNnJsonbNbr as pg_crud :: PgJsonTestCases > :: previous_rd_and_opt_upd_into_rd (found_rd_el . field_2 . expect ("2e8229f7") . v , field_2) }) }) ;
                    }
                    acc_04a67ef2
                }),
                None => rd,
            }
        }
        fn rd_ids_and_cr_into_rd(
            rd_ids: <Self::PgJson as pg_crud::PgJson>::RdIds,
            cr: <Self::PgJson as pg_crud::PgJson>::Cr,
        ) -> <Self::PgJson as pg_crud::PgJson>::Rd {
            VecOfObjExampleWithIdAsNnArrOfNnJsonbObjWithIdRd::new({
                assert_eq!(rd_ids.0.v.len(), cr.0.len(), "90d33ddd");
                let mut acc_37909420 = Vec::new();
                for (rd_ids_225e2b76, cr_3c660445) in rd_ids.0.v.into_iter().zip(cr.0) {
                    acc_37909420 . push (ObjExampleWithIdAsNnJsonbObjWithIdRd :: try_new (< pg_crud :: UuidUuidAsNnJsonbString as pg_crud :: PgJsonTestCases > :: rd_ids_and_cr_into_opt_v_rd (rd_ids_225e2b76 . 0. v . id , pg_crud :: DfltSomeOneEl :: dflt_some_one_el () ,) , < pg_crud :: I8AsNnJsonbNbr as pg_crud :: PgJsonTestCases > :: rd_ids_and_cr_into_opt_v_rd (rd_ids_225e2b76 . 0. v . field_0 , cr_3c660445 . field_0 ,) , < pg_crud :: OptI8AsNlJsonbNbr as pg_crud :: PgJsonTestCases > :: rd_ids_and_cr_into_opt_v_rd (rd_ids_225e2b76 . 0. v . field_1 , cr_3c660445 . field_1 ,) , < pg_crud :: VecOfI8AsNnArrOfNnJsonbNbr as pg_crud :: PgJsonTestCases > :: rd_ids_and_cr_into_opt_v_rd (rd_ids_225e2b76 . 0. v . field_2 , cr_3c660445 . field_2 ,)) . expect ("1330ac8d")) ;
                }
                acc_37909420
            })
        }
        fn rd_ids_and_cr_into_opt_v_rd(
            rd_ids: <Self::PgJson as pg_crud::PgJson>::RdIds,
            cr: <Self::PgJson as pg_crud::PgJson>::Cr,
        ) -> Option<pg_crud::V<<Self::PgJson as pg_crud::PgJson>::Rd>> {
            Some(pg_crud::V {
                v: <Self as pg_crud::PgJsonTestCases>::rd_ids_and_cr_into_rd(rd_ids, cr),
            })
        }
        fn rd_ids_and_cr_into_tt(
            rd_ids: <Self::PgJson as pg_crud::PgJson>::RdIds,
            cr: <Self::PgJson as pg_crud::PgJson>::Cr,
        ) -> <Self::PgJson as pg_crud::PgJson>::Tt {
            VecOfObjExampleWithIdAsNnArrOfNnJsonbObjWithIdTt::new({
                assert_eq!(rd_ids.0.v.len(), cr.0.len(), "7776a146");
                let mut acc_319e1fb1 = Vec::new();
                for (rd_ids_94b49496, cr_24629087) in rd_ids.0.v.into_iter().zip(cr.0) {
                    acc_319e1fb1 . push (ObjExampleWithIdAsNnJsonbObjWithIdTt :: new (< pg_crud :: UuidUuidAsNnJsonbString as pg_crud :: PgJsonTestCases > :: rd_ids_and_cr_into_tt (rd_ids_94b49496 . 0. v . id , pg_crud :: DfltSomeOneEl :: dflt_some_one_el () ,) , < pg_crud :: I8AsNnJsonbNbr as pg_crud :: PgJsonTestCases > :: rd_ids_and_cr_into_tt (rd_ids_94b49496 . 0. v . field_0 , cr_24629087 . field_0 ,) , < pg_crud :: OptI8AsNlJsonbNbr as pg_crud :: PgJsonTestCases > :: rd_ids_and_cr_into_tt (rd_ids_94b49496 . 0. v . field_1 , cr_24629087 . field_1 ,) , < pg_crud :: VecOfI8AsNnArrOfNnJsonbNbr as pg_crud :: PgJsonTestCases > :: rd_ids_and_cr_into_tt (rd_ids_94b49496 . 0. v . field_2 , cr_24629087 . field_2 ,))) ;
                }
                acc_319e1fb1
            })
        }
        fn rd_ids_and_cr_into_wh_eq(
            rd_ids: <Self::PgJson as pg_crud::PgJson>::RdIds,
            cr: <Self::PgJson as pg_crud::PgJson>::Cr,
        ) -> <Self::PgJson as pg_crud::PgJson>::Wh {
            VecOfObjExampleWithIdAsNnArrOfNnJsonbObjWithIdWh::Eq(pg_crud::PgJsonWhEq {
                oprtr: pg_crud::Oprtr::And,
                v: VecOfObjExampleWithIdAsNnArrOfNnJsonbObjWithIdTt::new({
                    let mut acc_321b3fcd = Vec::new();
                    for (rd_ids_ea32954c, cr_3cbe8967) in rd_ids.0.v.into_iter().zip(cr.0) {
                        acc_321b3fcd . push (ObjExampleWithIdAsNnJsonbObjWithIdTt :: new (< pg_crud :: UuidUuidAsNnJsonbString as pg_crud :: PgJsonTestCases > :: rd_ids_and_cr_into_tt (rd_ids_ea32954c . 0. v . id , pg_crud :: DfltSomeOneEl :: dflt_some_one_el ()) , < pg_crud :: I8AsNnJsonbNbr as pg_crud :: PgJsonTestCases > :: rd_ids_and_cr_into_tt (rd_ids_ea32954c . 0. v . field_0 , cr_3cbe8967 . field_0) , < pg_crud :: OptI8AsNlJsonbNbr as pg_crud :: PgJsonTestCases > :: rd_ids_and_cr_into_tt (rd_ids_ea32954c . 0. v . field_1 , cr_3cbe8967 . field_1) , < pg_crud :: VecOfI8AsNnArrOfNnJsonbNbr as pg_crud :: PgJsonTestCases > :: rd_ids_and_cr_into_tt (rd_ids_ea32954c . 0. v . field_2 , cr_3cbe8967 . field_2))) ;
                    }
                    acc_321b3fcd
                }),
            })
        }
        fn rd_ids_and_cr_into_vec_wh_eq_using_fields(
            rd_ids: <Self::PgJson as pg_crud::PgJson>::RdIds,
            cr: <Self::PgJson as pg_crud::PgJson>::Cr,
        ) -> pg_crud::NotEmptyUnqVec<<Self::PgJson as pg_crud::PgJson>::Wh> {
            pg_crud :: NotEmptyUnqVec :: try_new (vec ! [VecOfObjExampleWithIdAsNnArrOfNnJsonbObjWithIdWh :: Eq (pg_crud :: PgJsonWhEq { oprtr : pg_crud :: Oprtr :: And , v : VecOfObjExampleWithIdAsNnArrOfNnJsonbObjWithIdTt :: new ({ let mut acc_97ebf7d6 = Vec :: new () ; for (rd_ids_319c9e78 , cr_00ae06d2) in rd_ids . 0. v . into_iter () . zip (cr . 0) { acc_97ebf7d6 . push (ObjExampleWithIdAsNnJsonbObjWithIdTt :: new (< pg_crud :: UuidUuidAsNnJsonbString as pg_crud :: PgJsonTestCases > :: rd_ids_and_cr_into_tt (rd_ids_319c9e78 . 0. v . id , pg_crud :: DfltSomeOneEl :: dflt_some_one_el ()) , < pg_crud :: I8AsNnJsonbNbr as pg_crud :: PgJsonTestCases > :: rd_ids_and_cr_into_tt (rd_ids_319c9e78 . 0. v . field_0 , cr_00ae06d2 . field_0) , < pg_crud :: OptI8AsNlJsonbNbr as pg_crud :: PgJsonTestCases > :: rd_ids_and_cr_into_tt (rd_ids_319c9e78 . 0. v . field_1 , cr_00ae06d2 . field_1) , < pg_crud :: VecOfI8AsNnArrOfNnJsonbNbr as pg_crud :: PgJsonTestCases > :: rd_ids_and_cr_into_tt (rd_ids_319c9e78 . 0. v . field_2 , cr_00ae06d2 . field_2))) ; } acc_97ebf7d6 }) })]) . expect ("ba9c52c1")
        }
        fn rd_ids_and_cr_into_vec_wh_eq_to_json_field(
            rd_ids: <Self::PgJson as pg_crud::PgJson>::RdIds,
            cr: <Self::PgJson as pg_crud::PgJson>::Cr,
        ) -> pg_crud::NotEmptyUnqVec<<Self::PgJson as pg_crud::PgJson>::Wh> {
            <Self as pg_crud::PgJsonTestCases>::rd_ids_and_cr_into_vec_wh_eq_using_fields(
                rd_ids, cr,
            )
        }
        fn rd_ids_and_cr_into_pg_json_opt_vec_wh_dim_one_eq(
            rd_ids: <Self::PgJson as pg_crud::PgJson>::RdIds,
            cr: <Self::PgJson as pg_crud::PgJson>::Cr,
        ) -> Option<pg_crud::NotEmptyUnqVec<<Self::PgJson as pg_crud::PgJson>::Wh>> {
            match pg_crud::NotEmptyUnqVec::try_new({
                let mut acc_dd377eb1 = Vec::new();
                for (i_47620dcf, (rd_ids_420d38ca, cr_76f032c1)) in
                    rd_ids.0.v.into_iter().zip(cr.0).enumerate()
                {
                    if let Some (v_bf84026e) = < pg_crud :: I8AsNnJsonbNbr as pg_crud :: PgJsonTestCases > :: rd_ids_and_cr_into_pg_json_opt_vec_wh_dim_one_eq (rd_ids_420d38ca . 0. v . field_0 . clone () , cr_76f032c1 . field_0 . clone ()) { for el in v_bf84026e . clone () . into_vec () { let v_592e6b5f = VecOfObjExampleWithIdAsNnArrOfNnJsonbObjWithIdWh :: ElField0 (pg_crud :: PgTypeWh :: try_new (pg_crud :: Oprtr :: And , vec ! [el]) . expect ("1f7ae335")) ; if ! acc_dd377eb1 . contains (& v_592e6b5f) { acc_dd377eb1 . push (v_592e6b5f) ; } } let v_03205172 = VecOfObjExampleWithIdAsNnArrOfNnJsonbObjWithIdWh :: ElField0 (pg_crud :: PgTypeWh :: new (pg_crud :: Oprtr :: And , v_bf84026e)) ; if ! acc_dd377eb1 . contains (& v_03205172) { acc_dd377eb1 . push (v_03205172) ; } }
                    if let Some (v_bf84026e) = < pg_crud :: OptI8AsNlJsonbNbr as pg_crud :: PgJsonTestCases > :: rd_ids_and_cr_into_pg_json_opt_vec_wh_dim_one_eq (rd_ids_420d38ca . 0. v . field_1 . clone () , cr_76f032c1 . field_1 . clone ()) { for el in v_bf84026e . clone () . into_vec () { let v_592e6b5f = VecOfObjExampleWithIdAsNnArrOfNnJsonbObjWithIdWh :: ElField1 (pg_crud :: PgTypeWh :: try_new (pg_crud :: Oprtr :: And , vec ! [el]) . expect ("1f7ae335")) ; if ! acc_dd377eb1 . contains (& v_592e6b5f) { acc_dd377eb1 . push (v_592e6b5f) ; } } let v_03205172 = VecOfObjExampleWithIdAsNnArrOfNnJsonbObjWithIdWh :: ElField1 (pg_crud :: PgTypeWh :: new (pg_crud :: Oprtr :: And , v_bf84026e)) ; if ! acc_dd377eb1 . contains (& v_03205172) { acc_dd377eb1 . push (v_03205172) ; } }
                    if let Some (v_bf84026e) = < pg_crud :: VecOfI8AsNnArrOfNnJsonbNbr as pg_crud :: PgJsonTestCases > :: rd_ids_and_cr_into_pg_json_opt_vec_wh_dim_one_eq (rd_ids_420d38ca . 0. v . field_2 . clone () , cr_76f032c1 . field_2 . clone ()) { for el in v_bf84026e . clone () . into_vec () { let v_592e6b5f = VecOfObjExampleWithIdAsNnArrOfNnJsonbObjWithIdWh :: ElField2 (pg_crud :: PgTypeWh :: try_new (pg_crud :: Oprtr :: And , vec ! [el]) . expect ("1f7ae335")) ; if ! acc_dd377eb1 . contains (& v_592e6b5f) { acc_dd377eb1 . push (v_592e6b5f) ; } } let v_03205172 = VecOfObjExampleWithIdAsNnArrOfNnJsonbObjWithIdWh :: ElField2 (pg_crud :: PgTypeWh :: new (pg_crud :: Oprtr :: And , v_bf84026e)) ; if ! acc_dd377eb1 . contains (& v_03205172) { acc_dd377eb1 . push (v_03205172) ; } }
                    acc_dd377eb1 . push (VecOfObjExampleWithIdAsNnArrOfNnJsonbObjWithIdWh :: DimOneEq (pg_crud :: PgJsonWhDimOneEq { oprtr : pg_crud :: Oprtr :: And , dims : pg_crud :: BoundedVec :: try_from (vec ! [pg_crud :: UnsignedPartOfI32 :: try_from (i32 :: try_from (i_47620dcf) . expect ("5341936f")) . expect ("76906f3c")]) . expect ("8a624c70") , v : ObjExampleWithIdAsNnJsonbObjWithIdTt :: new (< pg_crud :: UuidUuidAsNnJsonbString as pg_crud :: PgJsonTestCases > :: rd_ids_and_cr_into_tt (rd_ids_420d38ca . 0. v . id , pg_crud :: DfltSomeOneEl :: dflt_some_one_el ()) , < pg_crud :: I8AsNnJsonbNbr as pg_crud :: PgJsonTestCases > :: rd_ids_and_cr_into_tt (rd_ids_420d38ca . 0. v . field_0 , cr_76f032c1 . field_0) , < pg_crud :: OptI8AsNlJsonbNbr as pg_crud :: PgJsonTestCases > :: rd_ids_and_cr_into_tt (rd_ids_420d38ca . 0. v . field_1 , cr_76f032c1 . field_1) , < pg_crud :: VecOfI8AsNnArrOfNnJsonbNbr as pg_crud :: PgJsonTestCases > :: rd_ids_and_cr_into_tt (rd_ids_420d38ca . 0. v . field_2 , cr_76f032c1 . field_2)) , })) ;
                }
                acc_dd377eb1
            }) {
                Ok(v_dfac36e4) => Some(v_dfac36e4),
                Err(er) => match er {
                    pg_crud::NotEmptyUnqVecTryNewEr::IsEmpty { .. } => None,
                    pg_crud::NotEmptyUnqVecTryNewEr::NotUnq { .. } => panic!("93390f1a"),
                },
            }
        }
        fn rd_ids_and_cr_into_pg_json_opt_vec_wh_dim_two_eq(
            rd_ids: <Self::PgJson as pg_crud::PgJson>::RdIds,
            cr: <Self::PgJson as pg_crud::PgJson>::Cr,
        ) -> Option<pg_crud::NotEmptyUnqVec<<Self::PgJson as pg_crud::PgJson>::Wh>> {
            match pg_crud::NotEmptyUnqVec::try_new({
                let mut acc_dd377eb1 = Vec::new();
                for (rd_ids_420d38ca, cr_76f032c1) in rd_ids.0.v.into_iter().zip(cr.0) {
                    if let Some (v_bf84026e) = < pg_crud :: I8AsNnJsonbNbr as pg_crud :: PgJsonTestCases > :: rd_ids_and_cr_into_pg_json_opt_vec_wh_dim_two_eq (rd_ids_420d38ca . 0. v . field_0 . clone () , cr_76f032c1 . field_0 . clone ()) { for el in v_bf84026e . clone () . into_vec () { let v_592e6b5f = VecOfObjExampleWithIdAsNnArrOfNnJsonbObjWithIdWh :: ElField0 (pg_crud :: PgTypeWh :: try_new (pg_crud :: Oprtr :: And , vec ! [el]) . expect ("1f7ae335")) ; if ! acc_dd377eb1 . contains (& v_592e6b5f) { acc_dd377eb1 . push (v_592e6b5f) ; } } let v_03205172 = VecOfObjExampleWithIdAsNnArrOfNnJsonbObjWithIdWh :: ElField0 (pg_crud :: PgTypeWh :: new (pg_crud :: Oprtr :: And , v_bf84026e)) ; if ! acc_dd377eb1 . contains (& v_03205172) { acc_dd377eb1 . push (v_03205172) ; } }
                    if let Some (v_bf84026e) = < pg_crud :: OptI8AsNlJsonbNbr as pg_crud :: PgJsonTestCases > :: rd_ids_and_cr_into_pg_json_opt_vec_wh_dim_two_eq (rd_ids_420d38ca . 0. v . field_1 . clone () , cr_76f032c1 . field_1 . clone ()) { for el in v_bf84026e . clone () . into_vec () { let v_592e6b5f = VecOfObjExampleWithIdAsNnArrOfNnJsonbObjWithIdWh :: ElField1 (pg_crud :: PgTypeWh :: try_new (pg_crud :: Oprtr :: And , vec ! [el]) . expect ("1f7ae335")) ; if ! acc_dd377eb1 . contains (& v_592e6b5f) { acc_dd377eb1 . push (v_592e6b5f) ; } } let v_03205172 = VecOfObjExampleWithIdAsNnArrOfNnJsonbObjWithIdWh :: ElField1 (pg_crud :: PgTypeWh :: new (pg_crud :: Oprtr :: And , v_bf84026e)) ; if ! acc_dd377eb1 . contains (& v_03205172) { acc_dd377eb1 . push (v_03205172) ; } }
                    if let Some (v_bf84026e) = < pg_crud :: VecOfI8AsNnArrOfNnJsonbNbr as pg_crud :: PgJsonTestCases > :: rd_ids_and_cr_into_pg_json_opt_vec_wh_dim_two_eq (rd_ids_420d38ca . 0. v . field_2 . clone () , cr_76f032c1 . field_2 . clone ()) { for el in v_bf84026e . clone () . into_vec () { let v_592e6b5f = VecOfObjExampleWithIdAsNnArrOfNnJsonbObjWithIdWh :: ElField2 (pg_crud :: PgTypeWh :: try_new (pg_crud :: Oprtr :: And , vec ! [el]) . expect ("1f7ae335")) ; if ! acc_dd377eb1 . contains (& v_592e6b5f) { acc_dd377eb1 . push (v_592e6b5f) ; } } let v_03205172 = VecOfObjExampleWithIdAsNnArrOfNnJsonbObjWithIdWh :: ElField2 (pg_crud :: PgTypeWh :: new (pg_crud :: Oprtr :: And , v_bf84026e)) ; if ! acc_dd377eb1 . contains (& v_03205172) { acc_dd377eb1 . push (v_03205172) ; } }
                }
                acc_dd377eb1
            }) {
                Ok(v_dfac36e4) => Some(v_dfac36e4),
                Err(er) => match er {
                    pg_crud::NotEmptyUnqVecTryNewEr::IsEmpty { .. } => None,
                    pg_crud::NotEmptyUnqVecTryNewEr::NotUnq { .. } => panic!("93390f1a"),
                },
            }
        }
        fn rd_ids_and_cr_into_pg_json_opt_vec_wh_dim_three_eq(
            rd_ids: <Self::PgJson as pg_crud::PgJson>::RdIds,
            cr: <Self::PgJson as pg_crud::PgJson>::Cr,
        ) -> Option<pg_crud::NotEmptyUnqVec<<Self::PgJson as pg_crud::PgJson>::Wh>> {
            match pg_crud::NotEmptyUnqVec::try_new({
                let mut acc_dd377eb1 = Vec::new();
                for (rd_ids_420d38ca, cr_76f032c1) in rd_ids.0.v.into_iter().zip(cr.0) {
                    if let Some (v_bf84026e) = < pg_crud :: I8AsNnJsonbNbr as pg_crud :: PgJsonTestCases > :: rd_ids_and_cr_into_pg_json_opt_vec_wh_dim_three_eq (rd_ids_420d38ca . 0. v . field_0 . clone () , cr_76f032c1 . field_0 . clone ()) { for el in v_bf84026e . clone () . into_vec () { let v_592e6b5f = VecOfObjExampleWithIdAsNnArrOfNnJsonbObjWithIdWh :: ElField0 (pg_crud :: PgTypeWh :: try_new (pg_crud :: Oprtr :: And , vec ! [el]) . expect ("1f7ae335")) ; if ! acc_dd377eb1 . contains (& v_592e6b5f) { acc_dd377eb1 . push (v_592e6b5f) ; } } let v_03205172 = VecOfObjExampleWithIdAsNnArrOfNnJsonbObjWithIdWh :: ElField0 (pg_crud :: PgTypeWh :: new (pg_crud :: Oprtr :: And , v_bf84026e)) ; if ! acc_dd377eb1 . contains (& v_03205172) { acc_dd377eb1 . push (v_03205172) ; } }
                    if let Some (v_bf84026e) = < pg_crud :: OptI8AsNlJsonbNbr as pg_crud :: PgJsonTestCases > :: rd_ids_and_cr_into_pg_json_opt_vec_wh_dim_three_eq (rd_ids_420d38ca . 0. v . field_1 . clone () , cr_76f032c1 . field_1 . clone ()) { for el in v_bf84026e . clone () . into_vec () { let v_592e6b5f = VecOfObjExampleWithIdAsNnArrOfNnJsonbObjWithIdWh :: ElField1 (pg_crud :: PgTypeWh :: try_new (pg_crud :: Oprtr :: And , vec ! [el]) . expect ("1f7ae335")) ; if ! acc_dd377eb1 . contains (& v_592e6b5f) { acc_dd377eb1 . push (v_592e6b5f) ; } } let v_03205172 = VecOfObjExampleWithIdAsNnArrOfNnJsonbObjWithIdWh :: ElField1 (pg_crud :: PgTypeWh :: new (pg_crud :: Oprtr :: And , v_bf84026e)) ; if ! acc_dd377eb1 . contains (& v_03205172) { acc_dd377eb1 . push (v_03205172) ; } }
                    if let Some (v_bf84026e) = < pg_crud :: VecOfI8AsNnArrOfNnJsonbNbr as pg_crud :: PgJsonTestCases > :: rd_ids_and_cr_into_pg_json_opt_vec_wh_dim_three_eq (rd_ids_420d38ca . 0. v . field_2 . clone () , cr_76f032c1 . field_2 . clone ()) { for el in v_bf84026e . clone () . into_vec () { let v_592e6b5f = VecOfObjExampleWithIdAsNnArrOfNnJsonbObjWithIdWh :: ElField2 (pg_crud :: PgTypeWh :: try_new (pg_crud :: Oprtr :: And , vec ! [el]) . expect ("1f7ae335")) ; if ! acc_dd377eb1 . contains (& v_592e6b5f) { acc_dd377eb1 . push (v_592e6b5f) ; } } let v_03205172 = VecOfObjExampleWithIdAsNnArrOfNnJsonbObjWithIdWh :: ElField2 (pg_crud :: PgTypeWh :: new (pg_crud :: Oprtr :: And , v_bf84026e)) ; if ! acc_dd377eb1 . contains (& v_03205172) { acc_dd377eb1 . push (v_03205172) ; } }
                }
                acc_dd377eb1
            }) {
                Ok(v_dfac36e4) => Some(v_dfac36e4),
                Err(er) => match er {
                    pg_crud::NotEmptyUnqVecTryNewEr::IsEmpty { .. } => None,
                    pg_crud::NotEmptyUnqVecTryNewEr::NotUnq { .. } => panic!("93390f1a"),
                },
            }
        }
        fn rd_ids_and_cr_into_pg_json_opt_vec_wh_dim_four_eq(
            rd_ids: <Self::PgJson as pg_crud::PgJson>::RdIds,
            cr: <Self::PgJson as pg_crud::PgJson>::Cr,
        ) -> Option<pg_crud::NotEmptyUnqVec<<Self::PgJson as pg_crud::PgJson>::Wh>> {
            match pg_crud::NotEmptyUnqVec::try_new({
                let mut acc_dd377eb1 = Vec::new();
                for (rd_ids_420d38ca, cr_76f032c1) in rd_ids.0.v.into_iter().zip(cr.0) {
                    if let Some (v_bf84026e) = < pg_crud :: I8AsNnJsonbNbr as pg_crud :: PgJsonTestCases > :: rd_ids_and_cr_into_pg_json_opt_vec_wh_dim_four_eq (rd_ids_420d38ca . 0. v . field_0 . clone () , cr_76f032c1 . field_0 . clone ()) { for el in v_bf84026e . clone () . into_vec () { let v_592e6b5f = VecOfObjExampleWithIdAsNnArrOfNnJsonbObjWithIdWh :: ElField0 (pg_crud :: PgTypeWh :: try_new (pg_crud :: Oprtr :: And , vec ! [el]) . expect ("1f7ae335")) ; if ! acc_dd377eb1 . contains (& v_592e6b5f) { acc_dd377eb1 . push (v_592e6b5f) ; } } let v_03205172 = VecOfObjExampleWithIdAsNnArrOfNnJsonbObjWithIdWh :: ElField0 (pg_crud :: PgTypeWh :: new (pg_crud :: Oprtr :: And , v_bf84026e)) ; if ! acc_dd377eb1 . contains (& v_03205172) { acc_dd377eb1 . push (v_03205172) ; } }
                    if let Some (v_bf84026e) = < pg_crud :: OptI8AsNlJsonbNbr as pg_crud :: PgJsonTestCases > :: rd_ids_and_cr_into_pg_json_opt_vec_wh_dim_four_eq (rd_ids_420d38ca . 0. v . field_1 . clone () , cr_76f032c1 . field_1 . clone ()) { for el in v_bf84026e . clone () . into_vec () { let v_592e6b5f = VecOfObjExampleWithIdAsNnArrOfNnJsonbObjWithIdWh :: ElField1 (pg_crud :: PgTypeWh :: try_new (pg_crud :: Oprtr :: And , vec ! [el]) . expect ("1f7ae335")) ; if ! acc_dd377eb1 . contains (& v_592e6b5f) { acc_dd377eb1 . push (v_592e6b5f) ; } } let v_03205172 = VecOfObjExampleWithIdAsNnArrOfNnJsonbObjWithIdWh :: ElField1 (pg_crud :: PgTypeWh :: new (pg_crud :: Oprtr :: And , v_bf84026e)) ; if ! acc_dd377eb1 . contains (& v_03205172) { acc_dd377eb1 . push (v_03205172) ; } }
                    if let Some (v_bf84026e) = < pg_crud :: VecOfI8AsNnArrOfNnJsonbNbr as pg_crud :: PgJsonTestCases > :: rd_ids_and_cr_into_pg_json_opt_vec_wh_dim_four_eq (rd_ids_420d38ca . 0. v . field_2 . clone () , cr_76f032c1 . field_2 . clone ()) { for el in v_bf84026e . clone () . into_vec () { let v_592e6b5f = VecOfObjExampleWithIdAsNnArrOfNnJsonbObjWithIdWh :: ElField2 (pg_crud :: PgTypeWh :: try_new (pg_crud :: Oprtr :: And , vec ! [el]) . expect ("1f7ae335")) ; if ! acc_dd377eb1 . contains (& v_592e6b5f) { acc_dd377eb1 . push (v_592e6b5f) ; } } let v_03205172 = VecOfObjExampleWithIdAsNnArrOfNnJsonbObjWithIdWh :: ElField2 (pg_crud :: PgTypeWh :: new (pg_crud :: Oprtr :: And , v_bf84026e)) ; if ! acc_dd377eb1 . contains (& v_03205172) { acc_dd377eb1 . push (v_03205172) ; } }
                }
                acc_dd377eb1
            }) {
                Ok(v_dfac36e4) => Some(v_dfac36e4),
                Err(er) => match er {
                    pg_crud::NotEmptyUnqVecTryNewEr::IsEmpty { .. } => None,
                    pg_crud::NotEmptyUnqVecTryNewEr::NotUnq { .. } => panic!("93390f1a"),
                },
            }
        }
        fn cr_into_pg_json_opt_vec_wh_len_eq(
            cr: <Self::PgJson as pg_crud::PgJson>::Cr,
        ) -> Option<pg_crud::NotEmptyUnqVec<<Self::PgJson as pg_crud::PgJson>::Wh>> {
            match pg_crud::NotEmptyUnqVec::try_new({
                let mut acc_480d72e5 = Vec::new();
                for cr_e06a9fe2 in cr.0.clone() {
                    if let Some (v_ee015fcc) = < pg_crud :: I8AsNnJsonbNbr as pg_crud :: PgJsonTestCases > :: cr_into_pg_json_opt_vec_wh_len_eq (cr_e06a9fe2 . field_0) { for el in v_ee015fcc . clone () . into_vec () { let v_0ae29f5f = VecOfObjExampleWithIdAsNnArrOfNnJsonbObjWithIdWh :: ElField0 (pg_crud :: PgTypeWh :: try_new (pg_crud :: Oprtr :: And , vec ! [el]) . expect ("38ca88dc") ,) ; if ! acc_480d72e5 . contains (& v_0ae29f5f) { acc_480d72e5 . push (v_0ae29f5f) ; } } let v_4e4cfda3 = VecOfObjExampleWithIdAsNnArrOfNnJsonbObjWithIdWh :: ElField0 (pg_crud :: PgTypeWh :: new (pg_crud :: Oprtr :: And , v_ee015fcc)) ; if ! acc_480d72e5 . contains (& v_4e4cfda3) { acc_480d72e5 . push (v_4e4cfda3) ; } }
                }
                for cr_e06a9fe2 in cr.0.clone() {
                    if let Some (v_ee015fcc) = < pg_crud :: OptI8AsNlJsonbNbr as pg_crud :: PgJsonTestCases > :: cr_into_pg_json_opt_vec_wh_len_eq (cr_e06a9fe2 . field_1) { for el in v_ee015fcc . clone () . into_vec () { let v_0ae29f5f = VecOfObjExampleWithIdAsNnArrOfNnJsonbObjWithIdWh :: ElField1 (pg_crud :: PgTypeWh :: try_new (pg_crud :: Oprtr :: And , vec ! [el]) . expect ("38ca88dc") ,) ; if ! acc_480d72e5 . contains (& v_0ae29f5f) { acc_480d72e5 . push (v_0ae29f5f) ; } } let v_4e4cfda3 = VecOfObjExampleWithIdAsNnArrOfNnJsonbObjWithIdWh :: ElField1 (pg_crud :: PgTypeWh :: new (pg_crud :: Oprtr :: And , v_ee015fcc)) ; if ! acc_480d72e5 . contains (& v_4e4cfda3) { acc_480d72e5 . push (v_4e4cfda3) ; } }
                }
                for cr_e06a9fe2 in cr.0.clone() {
                    if let Some (v_ee015fcc) = < pg_crud :: VecOfI8AsNnArrOfNnJsonbNbr as pg_crud :: PgJsonTestCases > :: cr_into_pg_json_opt_vec_wh_len_eq (cr_e06a9fe2 . field_2) { for el in v_ee015fcc . clone () . into_vec () { let v_0ae29f5f = VecOfObjExampleWithIdAsNnArrOfNnJsonbObjWithIdWh :: ElField2 (pg_crud :: PgTypeWh :: try_new (pg_crud :: Oprtr :: And , vec ! [el]) . expect ("38ca88dc") ,) ; if ! acc_480d72e5 . contains (& v_0ae29f5f) { acc_480d72e5 . push (v_0ae29f5f) ; } } let v_4e4cfda3 = VecOfObjExampleWithIdAsNnArrOfNnJsonbObjWithIdWh :: ElField2 (pg_crud :: PgTypeWh :: new (pg_crud :: Oprtr :: And , v_ee015fcc)) ; if ! acc_480d72e5 . contains (& v_4e4cfda3) { acc_480d72e5 . push (v_4e4cfda3) ; } }
                }
                acc_480d72e5.push(VecOfObjExampleWithIdAsNnArrOfNnJsonbObjWithIdWh::LenEq(
                    pg_crud::PgJsonWhLenEq {
                        oprtr: pg_crud::Oprtr::And,
                        v: pg_crud::UnsignedPartOfI32::try_from(
                            i32::try_from(cr.0.len()).expect("1811faf7"),
                        )
                        .expect("a590f39b"),
                    },
                ));
                acc_480d72e5
            }) {
                Ok(v_cc01db9a) => Some(v_cc01db9a),
                Err(er) => match er {
                    pg_crud::NotEmptyUnqVecTryNewEr::IsEmpty { .. } => None,
                    pg_crud::NotEmptyUnqVecTryNewEr::NotUnq { .. } => panic!("bad01dd0"),
                },
            }
        }
        fn cr_into_pg_json_opt_vec_wh_len_greater_than(
            cr: <Self::PgJson as pg_crud::PgJson>::Cr,
        ) -> Option<pg_crud::NotEmptyUnqVec<<Self::PgJson as pg_crud::PgJson>::Wh>> {
            match pg_crud::NotEmptyUnqVec::try_new({
                let mut acc_acceb7eb = Vec::new();
                for cr_34a1e540 in cr.0.clone() {
                    if let Some (v_51fe384b) = < pg_crud :: I8AsNnJsonbNbr as pg_crud :: PgJsonTestCases > :: cr_into_pg_json_opt_vec_wh_len_greater_than (cr_34a1e540 . field_0) { for el_4a00ab02 in v_51fe384b . clone () . into_vec () { let el_938f8b34 = VecOfObjExampleWithIdAsNnArrOfNnJsonbObjWithIdWh :: ElField0 (pg_crud :: PgTypeWh :: try_new (pg_crud :: Oprtr :: And , vec ! [el_4a00ab02]) . expect ("955c6c27") ,) ; if ! acc_acceb7eb . contains (& el_938f8b34) { acc_acceb7eb . push (el_938f8b34) ; } } let el_e17d9fba = VecOfObjExampleWithIdAsNnArrOfNnJsonbObjWithIdWh :: ElField0 (pg_crud :: PgTypeWh :: new (pg_crud :: Oprtr :: And , v_51fe384b)) ; if ! acc_acceb7eb . contains (& el_e17d9fba) { acc_acceb7eb . push (el_e17d9fba) ; } }
                }
                for cr_34a1e540 in cr.0.clone() {
                    if let Some (v_51fe384b) = < pg_crud :: OptI8AsNlJsonbNbr as pg_crud :: PgJsonTestCases > :: cr_into_pg_json_opt_vec_wh_len_greater_than (cr_34a1e540 . field_1) { for el_4a00ab02 in v_51fe384b . clone () . into_vec () { let el_938f8b34 = VecOfObjExampleWithIdAsNnArrOfNnJsonbObjWithIdWh :: ElField1 (pg_crud :: PgTypeWh :: try_new (pg_crud :: Oprtr :: And , vec ! [el_4a00ab02]) . expect ("955c6c27") ,) ; if ! acc_acceb7eb . contains (& el_938f8b34) { acc_acceb7eb . push (el_938f8b34) ; } } let el_e17d9fba = VecOfObjExampleWithIdAsNnArrOfNnJsonbObjWithIdWh :: ElField1 (pg_crud :: PgTypeWh :: new (pg_crud :: Oprtr :: And , v_51fe384b)) ; if ! acc_acceb7eb . contains (& el_e17d9fba) { acc_acceb7eb . push (el_e17d9fba) ; } }
                }
                for cr_34a1e540 in cr.0.clone() {
                    if let Some (v_51fe384b) = < pg_crud :: VecOfI8AsNnArrOfNnJsonbNbr as pg_crud :: PgJsonTestCases > :: cr_into_pg_json_opt_vec_wh_len_greater_than (cr_34a1e540 . field_2) { for el_4a00ab02 in v_51fe384b . clone () . into_vec () { let el_938f8b34 = VecOfObjExampleWithIdAsNnArrOfNnJsonbObjWithIdWh :: ElField2 (pg_crud :: PgTypeWh :: try_new (pg_crud :: Oprtr :: And , vec ! [el_4a00ab02]) . expect ("955c6c27") ,) ; if ! acc_acceb7eb . contains (& el_938f8b34) { acc_acceb7eb . push (el_938f8b34) ; } } let el_e17d9fba = VecOfObjExampleWithIdAsNnArrOfNnJsonbObjWithIdWh :: ElField2 (pg_crud :: PgTypeWh :: new (pg_crud :: Oprtr :: And , v_51fe384b)) ; if ! acc_acceb7eb . contains (& el_e17d9fba) { acc_acceb7eb . push (el_e17d9fba) ; } }
                }
                acc_acceb7eb.push(
                    VecOfObjExampleWithIdAsNnArrOfNnJsonbObjWithIdWh::LenGreaterThan(
                        pg_crud::PgJsonWhLenGreaterThan {
                            oprtr: pg_crud::Oprtr::And,
                            v: pg_crud::UnsignedPartOfI32::try_from(
                                i32::try_from(cr.0.len().checked_sub(1).unwrap_or_else(|| {
                                    panic!("e411b8ca");
                                }))
                                .expect("1fbbd897"),
                            )
                            .expect("0eb5d915"),
                        },
                    ),
                );
                acc_acceb7eb
            }) {
                Ok(v_a889de37) => Some(v_a889de37),
                Err(er) => match er {
                    pg_crud::NotEmptyUnqVecTryNewEr::IsEmpty { .. } => None,
                    pg_crud::NotEmptyUnqVecTryNewEr::NotUnq { .. } => panic!("a9e99f81"),
                },
            }
        }
        fn rd_ids_and_cr_into_pg_json_opt_vec_wh_greater_than(
            rd_ids: <Self::PgJson as pg_crud::PgJson>::RdIds,
            cr: <Self::PgJson as pg_crud::PgJson>::Cr,
        ) -> Option<
            pg_crud::NotEmptyUnqVec<
                pg_crud::SingleOrMultiple<<Self::PgJson as pg_crud::PgJson>::Wh>,
            >,
        > {
            match pg_crud::NotEmptyUnqVec::try_new({
                let mut acc_359c0b3f = Vec::new();
                for (rd_ids_629675e2, cr_82796400) in rd_ids.0.v.into_iter().zip(cr.0) {
                    let and = pg_crud::Oprtr::And;
                    let id = VecOfObjExampleWithIdAsNnArrOfNnJsonbObjWithIdWh::ElId(
                        pg_crud::PgTypeWh::try_new(
                            and,
                            vec![pg_crud::UuidUuidAsNnJsonbStringWh::Eq(
                                pg_crud::PgJsonWhEq {
                                    oprtr: pg_crud::Oprtr::Or,
                                    v: pg_crud::UuidUuidAsNnJsonbStringTt::new(
                                        rd_ids_629675e2.0.v.id.0.v,
                                    ),
                                },
                            )],
                        )
                        .expect("31db8e1e"),
                    );
                    let field_0 = < pg_crud :: I8AsNnJsonbNbr as pg_crud :: PgJsonTestCases > :: rd_ids_and_cr_into_pg_json_opt_vec_wh_greater_than (rd_ids_629675e2 . 0. v . field_0 , cr_82796400 . field_0) ;
                    let field_1 = < pg_crud :: OptI8AsNlJsonbNbr as pg_crud :: PgJsonTestCases > :: rd_ids_and_cr_into_pg_json_opt_vec_wh_greater_than (rd_ids_629675e2 . 0. v . field_1 , cr_82796400 . field_1) ;
                    let field_2 = < pg_crud :: VecOfI8AsNnArrOfNnJsonbNbr as pg_crud :: PgJsonTestCases > :: rd_ids_and_cr_into_pg_json_opt_vec_wh_greater_than (rd_ids_629675e2 . 0. v . field_2 , cr_82796400 . field_2) ;
                    if field_0.is_some() || field_1.is_some() || field_2.is_some() {
                        let mut all_fields_acc = vec![];
                        if let Some(v_f190793e) = field_0 {
                            for el_22ac4087 in v_f190793e.clone().into_vec() {
                                let wh_f8a4319c =
                                    VecOfObjExampleWithIdAsNnArrOfNnJsonbObjWithIdWh::ElField0(
                                        match el_22ac4087 {
                                            pg_crud::SingleOrMultiple::Multiple(multiple) => {
                                                pg_crud::PgTypeWh::new(and, multiple.clone())
                                            }
                                            pg_crud::SingleOrMultiple::Single(single) => {
                                                pg_crud::PgTypeWh::try_new(and, vec![single])
                                                    .expect("2ed4dc5e")
                                            }
                                        },
                                    );
                                all_fields_acc.push(wh_f8a4319c.clone());
                                match pg_crud::NotEmptyUnqVec::try_new(vec![
                                    id.clone(),
                                    wh_f8a4319c,
                                ]) {
                                    Ok(v_fdd1b3eb) => {
                                        let multiple_wh_with_id_f8a4319c =
                                            pg_crud::SingleOrMultiple::Multiple(v_fdd1b3eb);
                                        if !acc_359c0b3f.contains(&multiple_wh_with_id_f8a4319c) {
                                            acc_359c0b3f.push(multiple_wh_with_id_f8a4319c);
                                        }
                                    }
                                    Err(er) => match er {
                                        pg_crud::NotEmptyUnqVecTryNewEr::IsEmpty { .. } => (),
                                        pg_crud::NotEmptyUnqVecTryNewEr::NotUnq { .. } => {
                                            panic!("f0e3d01b")
                                        }
                                    },
                                }
                            }
                            match pg_crud::NotEmptyUnqVec::try_new(
                                v_f190793e
                                    .into_vec()
                                    .into_iter()
                                    .flat_map(|el| match el {
                                        pg_crud::SingleOrMultiple::Multiple(multiple) => {
                                            multiple.into_vec()
                                        }
                                        pg_crud::SingleOrMultiple::Single(single) => {
                                            std::iter::once(single).collect()
                                        }
                                    })
                                    .fold(Vec::new(), |mut acc_01265629, el| {
                                        if !acc_01265629.contains(&el) {
                                            acc_01265629.push(el);
                                        }
                                        acc_01265629
                                    }),
                            ) {
                                Ok(v_a4000d70) => {
                                    let v_d6218307 =
                                        VecOfObjExampleWithIdAsNnArrOfNnJsonbObjWithIdWh::ElField0(
                                            pg_crud::PgTypeWh::new(and, v_a4000d70),
                                        );
                                    if !all_fields_acc.contains(&v_d6218307) {
                                        all_fields_acc.push(v_d6218307);
                                    }
                                }
                                Err(er) => match er {
                                    pg_crud::NotEmptyUnqVecTryNewEr::IsEmpty { .. } => (),
                                    pg_crud::NotEmptyUnqVecTryNewEr::NotUnq { .. } => {
                                        panic!("f8fcc434")
                                    }
                                },
                            }
                        }
                        if let Some(v_f190793e) = field_1 {
                            for el_22ac4087 in v_f190793e.clone().into_vec() {
                                let wh_f8a4319c =
                                    VecOfObjExampleWithIdAsNnArrOfNnJsonbObjWithIdWh::ElField1(
                                        match el_22ac4087 {
                                            pg_crud::SingleOrMultiple::Multiple(multiple) => {
                                                pg_crud::PgTypeWh::new(and, multiple.clone())
                                            }
                                            pg_crud::SingleOrMultiple::Single(single) => {
                                                pg_crud::PgTypeWh::try_new(and, vec![single])
                                                    .expect("2ed4dc5e")
                                            }
                                        },
                                    );
                                all_fields_acc.push(wh_f8a4319c.clone());
                                match pg_crud::NotEmptyUnqVec::try_new(vec![
                                    id.clone(),
                                    wh_f8a4319c,
                                ]) {
                                    Ok(v_fdd1b3eb) => {
                                        let multiple_wh_with_id_f8a4319c =
                                            pg_crud::SingleOrMultiple::Multiple(v_fdd1b3eb);
                                        if !acc_359c0b3f.contains(&multiple_wh_with_id_f8a4319c) {
                                            acc_359c0b3f.push(multiple_wh_with_id_f8a4319c);
                                        }
                                    }
                                    Err(er) => match er {
                                        pg_crud::NotEmptyUnqVecTryNewEr::IsEmpty { .. } => (),
                                        pg_crud::NotEmptyUnqVecTryNewEr::NotUnq { .. } => {
                                            panic!("f0e3d01b")
                                        }
                                    },
                                }
                            }
                            match pg_crud::NotEmptyUnqVec::try_new(
                                v_f190793e
                                    .into_vec()
                                    .into_iter()
                                    .flat_map(|el| match el {
                                        pg_crud::SingleOrMultiple::Multiple(multiple) => {
                                            multiple.into_vec()
                                        }
                                        pg_crud::SingleOrMultiple::Single(single) => {
                                            std::iter::once(single).collect()
                                        }
                                    })
                                    .fold(Vec::new(), |mut acc_01265629, el| {
                                        if !acc_01265629.contains(&el) {
                                            acc_01265629.push(el);
                                        }
                                        acc_01265629
                                    }),
                            ) {
                                Ok(v_a4000d70) => {
                                    let v_d6218307 =
                                        VecOfObjExampleWithIdAsNnArrOfNnJsonbObjWithIdWh::ElField1(
                                            pg_crud::PgTypeWh::new(and, v_a4000d70),
                                        );
                                    if !all_fields_acc.contains(&v_d6218307) {
                                        all_fields_acc.push(v_d6218307);
                                    }
                                }
                                Err(er) => match er {
                                    pg_crud::NotEmptyUnqVecTryNewEr::IsEmpty { .. } => (),
                                    pg_crud::NotEmptyUnqVecTryNewEr::NotUnq { .. } => {
                                        panic!("f8fcc434")
                                    }
                                },
                            }
                        }
                        if let Some(v_f190793e) = field_2 {
                            for el_22ac4087 in v_f190793e.clone().into_vec() {
                                let wh_f8a4319c =
                                    VecOfObjExampleWithIdAsNnArrOfNnJsonbObjWithIdWh::ElField2(
                                        match el_22ac4087 {
                                            pg_crud::SingleOrMultiple::Multiple(multiple) => {
                                                pg_crud::PgTypeWh::new(and, multiple.clone())
                                            }
                                            pg_crud::SingleOrMultiple::Single(single) => {
                                                pg_crud::PgTypeWh::try_new(and, vec![single])
                                                    .expect("2ed4dc5e")
                                            }
                                        },
                                    );
                                all_fields_acc.push(wh_f8a4319c.clone());
                                match pg_crud::NotEmptyUnqVec::try_new(vec![
                                    id.clone(),
                                    wh_f8a4319c,
                                ]) {
                                    Ok(v_fdd1b3eb) => {
                                        let multiple_wh_with_id_f8a4319c =
                                            pg_crud::SingleOrMultiple::Multiple(v_fdd1b3eb);
                                        if !acc_359c0b3f.contains(&multiple_wh_with_id_f8a4319c) {
                                            acc_359c0b3f.push(multiple_wh_with_id_f8a4319c);
                                        }
                                    }
                                    Err(er) => match er {
                                        pg_crud::NotEmptyUnqVecTryNewEr::IsEmpty { .. } => (),
                                        pg_crud::NotEmptyUnqVecTryNewEr::NotUnq { .. } => {
                                            panic!("f0e3d01b")
                                        }
                                    },
                                }
                            }
                            match pg_crud::NotEmptyUnqVec::try_new(
                                v_f190793e
                                    .into_vec()
                                    .into_iter()
                                    .flat_map(|el| match el {
                                        pg_crud::SingleOrMultiple::Multiple(multiple) => {
                                            multiple.into_vec()
                                        }
                                        pg_crud::SingleOrMultiple::Single(single) => {
                                            std::iter::once(single).collect()
                                        }
                                    })
                                    .fold(Vec::new(), |mut acc_01265629, el| {
                                        if !acc_01265629.contains(&el) {
                                            acc_01265629.push(el);
                                        }
                                        acc_01265629
                                    }),
                            ) {
                                Ok(v_a4000d70) => {
                                    let v_d6218307 =
                                        VecOfObjExampleWithIdAsNnArrOfNnJsonbObjWithIdWh::ElField2(
                                            pg_crud::PgTypeWh::new(and, v_a4000d70),
                                        );
                                    if !all_fields_acc.contains(&v_d6218307) {
                                        all_fields_acc.push(v_d6218307);
                                    }
                                }
                                Err(er) => match er {
                                    pg_crud::NotEmptyUnqVecTryNewEr::IsEmpty { .. } => (),
                                    pg_crud::NotEmptyUnqVecTryNewEr::NotUnq { .. } => {
                                        panic!("f8fcc434")
                                    }
                                },
                            }
                        }
                        match pg_crud::NotEmptyUnqVec::try_new({
                            all_fields_acc.push(id);
                            all_fields_acc
                        }) {
                            Ok(v_80199720) => {
                                acc_359c0b3f.push(pg_crud::SingleOrMultiple::Multiple(v_80199720));
                            }
                            Err(er) => match er {
                                pg_crud::NotEmptyUnqVecTryNewEr::IsEmpty { .. } => (),
                                pg_crud::NotEmptyUnqVecTryNewEr::NotUnq { .. } => {
                                    panic!("32a3da97")
                                }
                            },
                        }
                    }
                }
                acc_359c0b3f
            }) {
                Ok(v_752f0e8d) => Some(v_752f0e8d),
                Err(er) => match er {
                    pg_crud::NotEmptyUnqVecTryNewEr::IsEmpty { .. } => None,
                    pg_crud::NotEmptyUnqVecTryNewEr::NotUnq { .. } => panic!("76542a11"),
                },
            }
        }
        fn rd_ids_and_cr_into_pg_json_opt_vec_wh_btwn(
            rd_ids: <Self::PgJson as pg_crud::PgJson>::RdIds,
            cr: <Self::PgJson as pg_crud::PgJson>::Cr,
        ) -> Option<
            pg_crud::NotEmptyUnqVec<
                pg_crud::SingleOrMultiple<<Self::PgJson as pg_crud::PgJson>::Wh>,
            >,
        > {
            match pg_crud::NotEmptyUnqVec::try_new({
                let mut acc_359c0b3f = Vec::new();
                for (rd_ids_629675e2, cr_82796400) in rd_ids.0.v.into_iter().zip(cr.0) {
                    let and = pg_crud::Oprtr::And;
                    let id = VecOfObjExampleWithIdAsNnArrOfNnJsonbObjWithIdWh::ElId(
                        pg_crud::PgTypeWh::try_new(
                            and,
                            vec![pg_crud::UuidUuidAsNnJsonbStringWh::Eq(
                                pg_crud::PgJsonWhEq {
                                    oprtr: pg_crud::Oprtr::Or,
                                    v: pg_crud::UuidUuidAsNnJsonbStringTt::new(
                                        rd_ids_629675e2.0.v.id.0.v,
                                    ),
                                },
                            )],
                        )
                        .expect("31db8e1e"),
                    );
                    let field_0 = < pg_crud :: I8AsNnJsonbNbr as pg_crud :: PgJsonTestCases > :: rd_ids_and_cr_into_pg_json_opt_vec_wh_btwn (rd_ids_629675e2 . 0. v . field_0 , cr_82796400 . field_0) ;
                    let field_1 = < pg_crud :: OptI8AsNlJsonbNbr as pg_crud :: PgJsonTestCases > :: rd_ids_and_cr_into_pg_json_opt_vec_wh_btwn (rd_ids_629675e2 . 0. v . field_1 , cr_82796400 . field_1) ;
                    let field_2 = < pg_crud :: VecOfI8AsNnArrOfNnJsonbNbr as pg_crud :: PgJsonTestCases > :: rd_ids_and_cr_into_pg_json_opt_vec_wh_btwn (rd_ids_629675e2 . 0. v . field_2 , cr_82796400 . field_2) ;
                    if field_0.is_some() || field_1.is_some() || field_2.is_some() {
                        let mut all_fields_acc = vec![];
                        if let Some(v_f190793e) = field_0 {
                            for el_22ac4087 in v_f190793e.clone().into_vec() {
                                let wh_f8a4319c =
                                    VecOfObjExampleWithIdAsNnArrOfNnJsonbObjWithIdWh::ElField0(
                                        match el_22ac4087 {
                                            pg_crud::SingleOrMultiple::Multiple(multiple) => {
                                                pg_crud::PgTypeWh::new(and, multiple.clone())
                                            }
                                            pg_crud::SingleOrMultiple::Single(single) => {
                                                pg_crud::PgTypeWh::try_new(and, vec![single])
                                                    .expect("2ed4dc5e")
                                            }
                                        },
                                    );
                                all_fields_acc.push(wh_f8a4319c.clone());
                                match pg_crud::NotEmptyUnqVec::try_new(vec![
                                    id.clone(),
                                    wh_f8a4319c,
                                ]) {
                                    Ok(v_fdd1b3eb) => {
                                        let multiple_wh_with_id_f8a4319c =
                                            pg_crud::SingleOrMultiple::Multiple(v_fdd1b3eb);
                                        if !acc_359c0b3f.contains(&multiple_wh_with_id_f8a4319c) {
                                            acc_359c0b3f.push(multiple_wh_with_id_f8a4319c);
                                        }
                                    }
                                    Err(er) => match er {
                                        pg_crud::NotEmptyUnqVecTryNewEr::IsEmpty { .. } => (),
                                        pg_crud::NotEmptyUnqVecTryNewEr::NotUnq { .. } => {
                                            panic!("f0e3d01b")
                                        }
                                    },
                                }
                            }
                            match pg_crud::NotEmptyUnqVec::try_new(
                                v_f190793e
                                    .into_vec()
                                    .into_iter()
                                    .flat_map(|el| match el {
                                        pg_crud::SingleOrMultiple::Multiple(multiple) => {
                                            multiple.into_vec()
                                        }
                                        pg_crud::SingleOrMultiple::Single(single) => {
                                            std::iter::once(single).collect()
                                        }
                                    })
                                    .fold(Vec::new(), |mut acc_01265629, el| {
                                        if !acc_01265629.contains(&el) {
                                            acc_01265629.push(el);
                                        }
                                        acc_01265629
                                    }),
                            ) {
                                Ok(v_a4000d70) => {
                                    let v_d6218307 =
                                        VecOfObjExampleWithIdAsNnArrOfNnJsonbObjWithIdWh::ElField0(
                                            pg_crud::PgTypeWh::new(and, v_a4000d70),
                                        );
                                    if !all_fields_acc.contains(&v_d6218307) {
                                        all_fields_acc.push(v_d6218307);
                                    }
                                }
                                Err(er) => match er {
                                    pg_crud::NotEmptyUnqVecTryNewEr::IsEmpty { .. } => (),
                                    pg_crud::NotEmptyUnqVecTryNewEr::NotUnq { .. } => {
                                        panic!("f8fcc434")
                                    }
                                },
                            }
                        }
                        if let Some(v_f190793e) = field_1 {
                            for el_22ac4087 in v_f190793e.clone().into_vec() {
                                let wh_f8a4319c =
                                    VecOfObjExampleWithIdAsNnArrOfNnJsonbObjWithIdWh::ElField1(
                                        match el_22ac4087 {
                                            pg_crud::SingleOrMultiple::Multiple(multiple) => {
                                                pg_crud::PgTypeWh::new(and, multiple.clone())
                                            }
                                            pg_crud::SingleOrMultiple::Single(single) => {
                                                pg_crud::PgTypeWh::try_new(and, vec![single])
                                                    .expect("2ed4dc5e")
                                            }
                                        },
                                    );
                                all_fields_acc.push(wh_f8a4319c.clone());
                                match pg_crud::NotEmptyUnqVec::try_new(vec![
                                    id.clone(),
                                    wh_f8a4319c,
                                ]) {
                                    Ok(v_fdd1b3eb) => {
                                        let multiple_wh_with_id_f8a4319c =
                                            pg_crud::SingleOrMultiple::Multiple(v_fdd1b3eb);
                                        if !acc_359c0b3f.contains(&multiple_wh_with_id_f8a4319c) {
                                            acc_359c0b3f.push(multiple_wh_with_id_f8a4319c);
                                        }
                                    }
                                    Err(er) => match er {
                                        pg_crud::NotEmptyUnqVecTryNewEr::IsEmpty { .. } => (),
                                        pg_crud::NotEmptyUnqVecTryNewEr::NotUnq { .. } => {
                                            panic!("f0e3d01b")
                                        }
                                    },
                                }
                            }
                            match pg_crud::NotEmptyUnqVec::try_new(
                                v_f190793e
                                    .into_vec()
                                    .into_iter()
                                    .flat_map(|el| match el {
                                        pg_crud::SingleOrMultiple::Multiple(multiple) => {
                                            multiple.into_vec()
                                        }
                                        pg_crud::SingleOrMultiple::Single(single) => {
                                            std::iter::once(single).collect()
                                        }
                                    })
                                    .fold(Vec::new(), |mut acc_01265629, el| {
                                        if !acc_01265629.contains(&el) {
                                            acc_01265629.push(el);
                                        }
                                        acc_01265629
                                    }),
                            ) {
                                Ok(v_a4000d70) => {
                                    let v_d6218307 =
                                        VecOfObjExampleWithIdAsNnArrOfNnJsonbObjWithIdWh::ElField1(
                                            pg_crud::PgTypeWh::new(and, v_a4000d70),
                                        );
                                    if !all_fields_acc.contains(&v_d6218307) {
                                        all_fields_acc.push(v_d6218307);
                                    }
                                }
                                Err(er) => match er {
                                    pg_crud::NotEmptyUnqVecTryNewEr::IsEmpty { .. } => (),
                                    pg_crud::NotEmptyUnqVecTryNewEr::NotUnq { .. } => {
                                        panic!("f8fcc434")
                                    }
                                },
                            }
                        }
                        if let Some(v_f190793e) = field_2 {
                            for el_22ac4087 in v_f190793e.clone().into_vec() {
                                let wh_f8a4319c =
                                    VecOfObjExampleWithIdAsNnArrOfNnJsonbObjWithIdWh::ElField2(
                                        match el_22ac4087 {
                                            pg_crud::SingleOrMultiple::Multiple(multiple) => {
                                                pg_crud::PgTypeWh::new(and, multiple.clone())
                                            }
                                            pg_crud::SingleOrMultiple::Single(single) => {
                                                pg_crud::PgTypeWh::try_new(and, vec![single])
                                                    .expect("2ed4dc5e")
                                            }
                                        },
                                    );
                                all_fields_acc.push(wh_f8a4319c.clone());
                                match pg_crud::NotEmptyUnqVec::try_new(vec![
                                    id.clone(),
                                    wh_f8a4319c,
                                ]) {
                                    Ok(v_fdd1b3eb) => {
                                        let multiple_wh_with_id_f8a4319c =
                                            pg_crud::SingleOrMultiple::Multiple(v_fdd1b3eb);
                                        if !acc_359c0b3f.contains(&multiple_wh_with_id_f8a4319c) {
                                            acc_359c0b3f.push(multiple_wh_with_id_f8a4319c);
                                        }
                                    }
                                    Err(er) => match er {
                                        pg_crud::NotEmptyUnqVecTryNewEr::IsEmpty { .. } => (),
                                        pg_crud::NotEmptyUnqVecTryNewEr::NotUnq { .. } => {
                                            panic!("f0e3d01b")
                                        }
                                    },
                                }
                            }
                            match pg_crud::NotEmptyUnqVec::try_new(
                                v_f190793e
                                    .into_vec()
                                    .into_iter()
                                    .flat_map(|el| match el {
                                        pg_crud::SingleOrMultiple::Multiple(multiple) => {
                                            multiple.into_vec()
                                        }
                                        pg_crud::SingleOrMultiple::Single(single) => {
                                            std::iter::once(single).collect()
                                        }
                                    })
                                    .fold(Vec::new(), |mut acc_01265629, el| {
                                        if !acc_01265629.contains(&el) {
                                            acc_01265629.push(el);
                                        }
                                        acc_01265629
                                    }),
                            ) {
                                Ok(v_a4000d70) => {
                                    let v_d6218307 =
                                        VecOfObjExampleWithIdAsNnArrOfNnJsonbObjWithIdWh::ElField2(
                                            pg_crud::PgTypeWh::new(and, v_a4000d70),
                                        );
                                    if !all_fields_acc.contains(&v_d6218307) {
                                        all_fields_acc.push(v_d6218307);
                                    }
                                }
                                Err(er) => match er {
                                    pg_crud::NotEmptyUnqVecTryNewEr::IsEmpty { .. } => (),
                                    pg_crud::NotEmptyUnqVecTryNewEr::NotUnq { .. } => {
                                        panic!("f8fcc434")
                                    }
                                },
                            }
                        }
                        match pg_crud::NotEmptyUnqVec::try_new({
                            all_fields_acc.push(id);
                            all_fields_acc
                        }) {
                            Ok(v_80199720) => {
                                acc_359c0b3f.push(pg_crud::SingleOrMultiple::Multiple(v_80199720));
                            }
                            Err(er) => match er {
                                pg_crud::NotEmptyUnqVecTryNewEr::IsEmpty { .. } => (),
                                pg_crud::NotEmptyUnqVecTryNewEr::NotUnq { .. } => {
                                    panic!("32a3da97")
                                }
                            },
                        }
                    }
                }
                acc_359c0b3f
            }) {
                Ok(v_752f0e8d) => Some(v_752f0e8d),
                Err(er) => match er {
                    pg_crud::NotEmptyUnqVecTryNewEr::IsEmpty { .. } => None,
                    pg_crud::NotEmptyUnqVecTryNewEr::NotUnq { .. } => panic!("76542a11"),
                },
            }
        }
        fn rd_ids_and_cr_into_pg_json_opt_vec_wh_in(
            rd_ids: <Self::PgJson as pg_crud::PgJson>::RdIds,
            cr: <Self::PgJson as pg_crud::PgJson>::Cr,
        ) -> Option<
            pg_crud::NotEmptyUnqVec<
                pg_crud::SingleOrMultiple<<Self::PgJson as pg_crud::PgJson>::Wh>,
            >,
        > {
            match pg_crud::NotEmptyUnqVec::try_new({
                let mut acc_359c0b3f = Vec::new();
                for (rd_ids_629675e2, cr_82796400) in rd_ids.0.v.into_iter().zip(cr.0) {
                    let and = pg_crud::Oprtr::And;
                    let id = VecOfObjExampleWithIdAsNnArrOfNnJsonbObjWithIdWh::ElId(
                        pg_crud::PgTypeWh::try_new(
                            and,
                            vec![pg_crud::UuidUuidAsNnJsonbStringWh::Eq(
                                pg_crud::PgJsonWhEq {
                                    oprtr: pg_crud::Oprtr::Or,
                                    v: pg_crud::UuidUuidAsNnJsonbStringTt::new(
                                        rd_ids_629675e2.0.v.id.0.v,
                                    ),
                                },
                            )],
                        )
                        .expect("31db8e1e"),
                    );
                    let field_0 = < pg_crud :: I8AsNnJsonbNbr as pg_crud :: PgJsonTestCases > :: rd_ids_and_cr_into_pg_json_opt_vec_wh_in (rd_ids_629675e2 . 0. v . field_0 , cr_82796400 . field_0) ;
                    let field_1 = < pg_crud :: OptI8AsNlJsonbNbr as pg_crud :: PgJsonTestCases > :: rd_ids_and_cr_into_pg_json_opt_vec_wh_in (rd_ids_629675e2 . 0. v . field_1 , cr_82796400 . field_1) ;
                    let field_2 = < pg_crud :: VecOfI8AsNnArrOfNnJsonbNbr as pg_crud :: PgJsonTestCases > :: rd_ids_and_cr_into_pg_json_opt_vec_wh_in (rd_ids_629675e2 . 0. v . field_2 , cr_82796400 . field_2) ;
                    if field_0.is_some() || field_1.is_some() || field_2.is_some() {
                        let mut all_fields_acc = vec![];
                        if let Some(v_f190793e) = field_0 {
                            for el_22ac4087 in v_f190793e.clone().into_vec() {
                                let wh_f8a4319c =
                                    VecOfObjExampleWithIdAsNnArrOfNnJsonbObjWithIdWh::ElField0(
                                        match el_22ac4087 {
                                            pg_crud::SingleOrMultiple::Multiple(multiple) => {
                                                pg_crud::PgTypeWh::new(and, multiple.clone())
                                            }
                                            pg_crud::SingleOrMultiple::Single(single) => {
                                                pg_crud::PgTypeWh::try_new(and, vec![single])
                                                    .expect("2ed4dc5e")
                                            }
                                        },
                                    );
                                all_fields_acc.push(wh_f8a4319c.clone());
                                match pg_crud::NotEmptyUnqVec::try_new(vec![
                                    id.clone(),
                                    wh_f8a4319c,
                                ]) {
                                    Ok(v_fdd1b3eb) => {
                                        let multiple_wh_with_id_f8a4319c =
                                            pg_crud::SingleOrMultiple::Multiple(v_fdd1b3eb);
                                        if !acc_359c0b3f.contains(&multiple_wh_with_id_f8a4319c) {
                                            acc_359c0b3f.push(multiple_wh_with_id_f8a4319c);
                                        }
                                    }
                                    Err(er) => match er {
                                        pg_crud::NotEmptyUnqVecTryNewEr::IsEmpty { .. } => (),
                                        pg_crud::NotEmptyUnqVecTryNewEr::NotUnq { .. } => {
                                            panic!("f0e3d01b")
                                        }
                                    },
                                }
                            }
                            match pg_crud::NotEmptyUnqVec::try_new(
                                v_f190793e
                                    .into_vec()
                                    .into_iter()
                                    .flat_map(|el| match el {
                                        pg_crud::SingleOrMultiple::Multiple(multiple) => {
                                            multiple.into_vec()
                                        }
                                        pg_crud::SingleOrMultiple::Single(single) => {
                                            std::iter::once(single).collect()
                                        }
                                    })
                                    .fold(Vec::new(), |mut acc_01265629, el| {
                                        if !acc_01265629.contains(&el) {
                                            acc_01265629.push(el);
                                        }
                                        acc_01265629
                                    }),
                            ) {
                                Ok(v_a4000d70) => {
                                    let v_d6218307 =
                                        VecOfObjExampleWithIdAsNnArrOfNnJsonbObjWithIdWh::ElField0(
                                            pg_crud::PgTypeWh::new(and, v_a4000d70),
                                        );
                                    if !all_fields_acc.contains(&v_d6218307) {
                                        all_fields_acc.push(v_d6218307);
                                    }
                                }
                                Err(er) => match er {
                                    pg_crud::NotEmptyUnqVecTryNewEr::IsEmpty { .. } => (),
                                    pg_crud::NotEmptyUnqVecTryNewEr::NotUnq { .. } => {
                                        panic!("f8fcc434")
                                    }
                                },
                            }
                        }
                        if let Some(v_f190793e) = field_1 {
                            for el_22ac4087 in v_f190793e.clone().into_vec() {
                                let wh_f8a4319c =
                                    VecOfObjExampleWithIdAsNnArrOfNnJsonbObjWithIdWh::ElField1(
                                        match el_22ac4087 {
                                            pg_crud::SingleOrMultiple::Multiple(multiple) => {
                                                pg_crud::PgTypeWh::new(and, multiple.clone())
                                            }
                                            pg_crud::SingleOrMultiple::Single(single) => {
                                                pg_crud::PgTypeWh::try_new(and, vec![single])
                                                    .expect("2ed4dc5e")
                                            }
                                        },
                                    );
                                all_fields_acc.push(wh_f8a4319c.clone());
                                match pg_crud::NotEmptyUnqVec::try_new(vec![
                                    id.clone(),
                                    wh_f8a4319c,
                                ]) {
                                    Ok(v_fdd1b3eb) => {
                                        let multiple_wh_with_id_f8a4319c =
                                            pg_crud::SingleOrMultiple::Multiple(v_fdd1b3eb);
                                        if !acc_359c0b3f.contains(&multiple_wh_with_id_f8a4319c) {
                                            acc_359c0b3f.push(multiple_wh_with_id_f8a4319c);
                                        }
                                    }
                                    Err(er) => match er {
                                        pg_crud::NotEmptyUnqVecTryNewEr::IsEmpty { .. } => (),
                                        pg_crud::NotEmptyUnqVecTryNewEr::NotUnq { .. } => {
                                            panic!("f0e3d01b")
                                        }
                                    },
                                }
                            }
                            match pg_crud::NotEmptyUnqVec::try_new(
                                v_f190793e
                                    .into_vec()
                                    .into_iter()
                                    .flat_map(|el| match el {
                                        pg_crud::SingleOrMultiple::Multiple(multiple) => {
                                            multiple.into_vec()
                                        }
                                        pg_crud::SingleOrMultiple::Single(single) => {
                                            std::iter::once(single).collect()
                                        }
                                    })
                                    .fold(Vec::new(), |mut acc_01265629, el| {
                                        if !acc_01265629.contains(&el) {
                                            acc_01265629.push(el);
                                        }
                                        acc_01265629
                                    }),
                            ) {
                                Ok(v_a4000d70) => {
                                    let v_d6218307 =
                                        VecOfObjExampleWithIdAsNnArrOfNnJsonbObjWithIdWh::ElField1(
                                            pg_crud::PgTypeWh::new(and, v_a4000d70),
                                        );
                                    if !all_fields_acc.contains(&v_d6218307) {
                                        all_fields_acc.push(v_d6218307);
                                    }
                                }
                                Err(er) => match er {
                                    pg_crud::NotEmptyUnqVecTryNewEr::IsEmpty { .. } => (),
                                    pg_crud::NotEmptyUnqVecTryNewEr::NotUnq { .. } => {
                                        panic!("f8fcc434")
                                    }
                                },
                            }
                        }
                        if let Some(v_f190793e) = field_2 {
                            for el_22ac4087 in v_f190793e.clone().into_vec() {
                                let wh_f8a4319c =
                                    VecOfObjExampleWithIdAsNnArrOfNnJsonbObjWithIdWh::ElField2(
                                        match el_22ac4087 {
                                            pg_crud::SingleOrMultiple::Multiple(multiple) => {
                                                pg_crud::PgTypeWh::new(and, multiple.clone())
                                            }
                                            pg_crud::SingleOrMultiple::Single(single) => {
                                                pg_crud::PgTypeWh::try_new(and, vec![single])
                                                    .expect("2ed4dc5e")
                                            }
                                        },
                                    );
                                all_fields_acc.push(wh_f8a4319c.clone());
                                match pg_crud::NotEmptyUnqVec::try_new(vec![
                                    id.clone(),
                                    wh_f8a4319c,
                                ]) {
                                    Ok(v_fdd1b3eb) => {
                                        let multiple_wh_with_id_f8a4319c =
                                            pg_crud::SingleOrMultiple::Multiple(v_fdd1b3eb);
                                        if !acc_359c0b3f.contains(&multiple_wh_with_id_f8a4319c) {
                                            acc_359c0b3f.push(multiple_wh_with_id_f8a4319c);
                                        }
                                    }
                                    Err(er) => match er {
                                        pg_crud::NotEmptyUnqVecTryNewEr::IsEmpty { .. } => (),
                                        pg_crud::NotEmptyUnqVecTryNewEr::NotUnq { .. } => {
                                            panic!("f0e3d01b")
                                        }
                                    },
                                }
                            }
                            match pg_crud::NotEmptyUnqVec::try_new(
                                v_f190793e
                                    .into_vec()
                                    .into_iter()
                                    .flat_map(|el| match el {
                                        pg_crud::SingleOrMultiple::Multiple(multiple) => {
                                            multiple.into_vec()
                                        }
                                        pg_crud::SingleOrMultiple::Single(single) => {
                                            std::iter::once(single).collect()
                                        }
                                    })
                                    .fold(Vec::new(), |mut acc_01265629, el| {
                                        if !acc_01265629.contains(&el) {
                                            acc_01265629.push(el);
                                        }
                                        acc_01265629
                                    }),
                            ) {
                                Ok(v_a4000d70) => {
                                    let v_d6218307 =
                                        VecOfObjExampleWithIdAsNnArrOfNnJsonbObjWithIdWh::ElField2(
                                            pg_crud::PgTypeWh::new(and, v_a4000d70),
                                        );
                                    if !all_fields_acc.contains(&v_d6218307) {
                                        all_fields_acc.push(v_d6218307);
                                    }
                                }
                                Err(er) => match er {
                                    pg_crud::NotEmptyUnqVecTryNewEr::IsEmpty { .. } => (),
                                    pg_crud::NotEmptyUnqVecTryNewEr::NotUnq { .. } => {
                                        panic!("f8fcc434")
                                    }
                                },
                            }
                        }
                        match pg_crud::NotEmptyUnqVec::try_new({
                            all_fields_acc.push(id);
                            all_fields_acc
                        }) {
                            Ok(v_80199720) => {
                                acc_359c0b3f.push(pg_crud::SingleOrMultiple::Multiple(v_80199720));
                            }
                            Err(er) => match er {
                                pg_crud::NotEmptyUnqVecTryNewEr::IsEmpty { .. } => (),
                                pg_crud::NotEmptyUnqVecTryNewEr::NotUnq { .. } => {
                                    panic!("32a3da97")
                                }
                            },
                        }
                    }
                }
                acc_359c0b3f
            }) {
                Ok(v_752f0e8d) => Some(v_752f0e8d),
                Err(er) => match er {
                    pg_crud::NotEmptyUnqVecTryNewEr::IsEmpty { .. } => None,
                    pg_crud::NotEmptyUnqVecTryNewEr::NotUnq { .. } => panic!("76542a11"),
                },
            }
        }
        fn rd_ids_and_cr_into_pg_json_opt_vec_wh_rgx(
            rd_ids: <Self::PgJson as pg_crud::PgJson>::RdIds,
            cr: <Self::PgJson as pg_crud::PgJson>::Cr,
        ) -> Option<
            pg_crud::NotEmptyUnqVec<
                pg_crud::SingleOrMultiple<<Self::PgJson as pg_crud::PgJson>::Wh>,
            >,
        > {
            match pg_crud::NotEmptyUnqVec::try_new({
                let mut acc_359c0b3f = Vec::new();
                for (rd_ids_629675e2, cr_82796400) in rd_ids.0.v.into_iter().zip(cr.0) {
                    let and = pg_crud::Oprtr::And;
                    let id = VecOfObjExampleWithIdAsNnArrOfNnJsonbObjWithIdWh::ElId(
                        pg_crud::PgTypeWh::try_new(
                            and,
                            vec![pg_crud::UuidUuidAsNnJsonbStringWh::Eq(
                                pg_crud::PgJsonWhEq {
                                    oprtr: pg_crud::Oprtr::Or,
                                    v: pg_crud::UuidUuidAsNnJsonbStringTt::new(
                                        rd_ids_629675e2.0.v.id.0.v,
                                    ),
                                },
                            )],
                        )
                        .expect("31db8e1e"),
                    );
                    let field_0 = < pg_crud :: I8AsNnJsonbNbr as pg_crud :: PgJsonTestCases > :: rd_ids_and_cr_into_pg_json_opt_vec_wh_rgx (rd_ids_629675e2 . 0. v . field_0 , cr_82796400 . field_0) ;
                    let field_1 = < pg_crud :: OptI8AsNlJsonbNbr as pg_crud :: PgJsonTestCases > :: rd_ids_and_cr_into_pg_json_opt_vec_wh_rgx (rd_ids_629675e2 . 0. v . field_1 , cr_82796400 . field_1) ;
                    let field_2 = < pg_crud :: VecOfI8AsNnArrOfNnJsonbNbr as pg_crud :: PgJsonTestCases > :: rd_ids_and_cr_into_pg_json_opt_vec_wh_rgx (rd_ids_629675e2 . 0. v . field_2 , cr_82796400 . field_2) ;
                    if field_0.is_some() || field_1.is_some() || field_2.is_some() {
                        let mut all_fields_acc = vec![];
                        if let Some(v_f190793e) = field_0 {
                            for el_22ac4087 in v_f190793e.clone().into_vec() {
                                let wh_f8a4319c =
                                    VecOfObjExampleWithIdAsNnArrOfNnJsonbObjWithIdWh::ElField0(
                                        match el_22ac4087 {
                                            pg_crud::SingleOrMultiple::Multiple(multiple) => {
                                                pg_crud::PgTypeWh::new(and, multiple.clone())
                                            }
                                            pg_crud::SingleOrMultiple::Single(single) => {
                                                pg_crud::PgTypeWh::try_new(and, vec![single])
                                                    .expect("2ed4dc5e")
                                            }
                                        },
                                    );
                                all_fields_acc.push(wh_f8a4319c.clone());
                                match pg_crud::NotEmptyUnqVec::try_new(vec![
                                    id.clone(),
                                    wh_f8a4319c,
                                ]) {
                                    Ok(v_fdd1b3eb) => {
                                        let multiple_wh_with_id_f8a4319c =
                                            pg_crud::SingleOrMultiple::Multiple(v_fdd1b3eb);
                                        if !acc_359c0b3f.contains(&multiple_wh_with_id_f8a4319c) {
                                            acc_359c0b3f.push(multiple_wh_with_id_f8a4319c);
                                        }
                                    }
                                    Err(er) => match er {
                                        pg_crud::NotEmptyUnqVecTryNewEr::IsEmpty { .. } => (),
                                        pg_crud::NotEmptyUnqVecTryNewEr::NotUnq { .. } => {
                                            panic!("f0e3d01b")
                                        }
                                    },
                                }
                            }
                            match pg_crud::NotEmptyUnqVec::try_new(
                                v_f190793e
                                    .into_vec()
                                    .into_iter()
                                    .flat_map(|el| match el {
                                        pg_crud::SingleOrMultiple::Multiple(multiple) => {
                                            multiple.into_vec()
                                        }
                                        pg_crud::SingleOrMultiple::Single(single) => {
                                            std::iter::once(single).collect()
                                        }
                                    })
                                    .fold(Vec::new(), |mut acc_01265629, el| {
                                        if !acc_01265629.contains(&el) {
                                            acc_01265629.push(el);
                                        }
                                        acc_01265629
                                    }),
                            ) {
                                Ok(v_a4000d70) => {
                                    let v_d6218307 =
                                        VecOfObjExampleWithIdAsNnArrOfNnJsonbObjWithIdWh::ElField0(
                                            pg_crud::PgTypeWh::new(and, v_a4000d70),
                                        );
                                    if !all_fields_acc.contains(&v_d6218307) {
                                        all_fields_acc.push(v_d6218307);
                                    }
                                }
                                Err(er) => match er {
                                    pg_crud::NotEmptyUnqVecTryNewEr::IsEmpty { .. } => (),
                                    pg_crud::NotEmptyUnqVecTryNewEr::NotUnq { .. } => {
                                        panic!("f8fcc434")
                                    }
                                },
                            }
                        }
                        if let Some(v_f190793e) = field_1 {
                            for el_22ac4087 in v_f190793e.clone().into_vec() {
                                let wh_f8a4319c =
                                    VecOfObjExampleWithIdAsNnArrOfNnJsonbObjWithIdWh::ElField1(
                                        match el_22ac4087 {
                                            pg_crud::SingleOrMultiple::Multiple(multiple) => {
                                                pg_crud::PgTypeWh::new(and, multiple.clone())
                                            }
                                            pg_crud::SingleOrMultiple::Single(single) => {
                                                pg_crud::PgTypeWh::try_new(and, vec![single])
                                                    .expect("2ed4dc5e")
                                            }
                                        },
                                    );
                                all_fields_acc.push(wh_f8a4319c.clone());
                                match pg_crud::NotEmptyUnqVec::try_new(vec![
                                    id.clone(),
                                    wh_f8a4319c,
                                ]) {
                                    Ok(v_fdd1b3eb) => {
                                        let multiple_wh_with_id_f8a4319c =
                                            pg_crud::SingleOrMultiple::Multiple(v_fdd1b3eb);
                                        if !acc_359c0b3f.contains(&multiple_wh_with_id_f8a4319c) {
                                            acc_359c0b3f.push(multiple_wh_with_id_f8a4319c);
                                        }
                                    }
                                    Err(er) => match er {
                                        pg_crud::NotEmptyUnqVecTryNewEr::IsEmpty { .. } => (),
                                        pg_crud::NotEmptyUnqVecTryNewEr::NotUnq { .. } => {
                                            panic!("f0e3d01b")
                                        }
                                    },
                                }
                            }
                            match pg_crud::NotEmptyUnqVec::try_new(
                                v_f190793e
                                    .into_vec()
                                    .into_iter()
                                    .flat_map(|el| match el {
                                        pg_crud::SingleOrMultiple::Multiple(multiple) => {
                                            multiple.into_vec()
                                        }
                                        pg_crud::SingleOrMultiple::Single(single) => {
                                            std::iter::once(single).collect()
                                        }
                                    })
                                    .fold(Vec::new(), |mut acc_01265629, el| {
                                        if !acc_01265629.contains(&el) {
                                            acc_01265629.push(el);
                                        }
                                        acc_01265629
                                    }),
                            ) {
                                Ok(v_a4000d70) => {
                                    let v_d6218307 =
                                        VecOfObjExampleWithIdAsNnArrOfNnJsonbObjWithIdWh::ElField1(
                                            pg_crud::PgTypeWh::new(and, v_a4000d70),
                                        );
                                    if !all_fields_acc.contains(&v_d6218307) {
                                        all_fields_acc.push(v_d6218307);
                                    }
                                }
                                Err(er) => match er {
                                    pg_crud::NotEmptyUnqVecTryNewEr::IsEmpty { .. } => (),
                                    pg_crud::NotEmptyUnqVecTryNewEr::NotUnq { .. } => {
                                        panic!("f8fcc434")
                                    }
                                },
                            }
                        }
                        if let Some(v_f190793e) = field_2 {
                            for el_22ac4087 in v_f190793e.clone().into_vec() {
                                let wh_f8a4319c =
                                    VecOfObjExampleWithIdAsNnArrOfNnJsonbObjWithIdWh::ElField2(
                                        match el_22ac4087 {
                                            pg_crud::SingleOrMultiple::Multiple(multiple) => {
                                                pg_crud::PgTypeWh::new(and, multiple.clone())
                                            }
                                            pg_crud::SingleOrMultiple::Single(single) => {
                                                pg_crud::PgTypeWh::try_new(and, vec![single])
                                                    .expect("2ed4dc5e")
                                            }
                                        },
                                    );
                                all_fields_acc.push(wh_f8a4319c.clone());
                                match pg_crud::NotEmptyUnqVec::try_new(vec![
                                    id.clone(),
                                    wh_f8a4319c,
                                ]) {
                                    Ok(v_fdd1b3eb) => {
                                        let multiple_wh_with_id_f8a4319c =
                                            pg_crud::SingleOrMultiple::Multiple(v_fdd1b3eb);
                                        if !acc_359c0b3f.contains(&multiple_wh_with_id_f8a4319c) {
                                            acc_359c0b3f.push(multiple_wh_with_id_f8a4319c);
                                        }
                                    }
                                    Err(er) => match er {
                                        pg_crud::NotEmptyUnqVecTryNewEr::IsEmpty { .. } => (),
                                        pg_crud::NotEmptyUnqVecTryNewEr::NotUnq { .. } => {
                                            panic!("f0e3d01b")
                                        }
                                    },
                                }
                            }
                            match pg_crud::NotEmptyUnqVec::try_new(
                                v_f190793e
                                    .into_vec()
                                    .into_iter()
                                    .flat_map(|el| match el {
                                        pg_crud::SingleOrMultiple::Multiple(multiple) => {
                                            multiple.into_vec()
                                        }
                                        pg_crud::SingleOrMultiple::Single(single) => {
                                            std::iter::once(single).collect()
                                        }
                                    })
                                    .fold(Vec::new(), |mut acc_01265629, el| {
                                        if !acc_01265629.contains(&el) {
                                            acc_01265629.push(el);
                                        }
                                        acc_01265629
                                    }),
                            ) {
                                Ok(v_a4000d70) => {
                                    let v_d6218307 =
                                        VecOfObjExampleWithIdAsNnArrOfNnJsonbObjWithIdWh::ElField2(
                                            pg_crud::PgTypeWh::new(and, v_a4000d70),
                                        );
                                    if !all_fields_acc.contains(&v_d6218307) {
                                        all_fields_acc.push(v_d6218307);
                                    }
                                }
                                Err(er) => match er {
                                    pg_crud::NotEmptyUnqVecTryNewEr::IsEmpty { .. } => (),
                                    pg_crud::NotEmptyUnqVecTryNewEr::NotUnq { .. } => {
                                        panic!("f8fcc434")
                                    }
                                },
                            }
                        }
                        match pg_crud::NotEmptyUnqVec::try_new({
                            all_fields_acc.push(id);
                            all_fields_acc
                        }) {
                            Ok(v_80199720) => {
                                acc_359c0b3f.push(pg_crud::SingleOrMultiple::Multiple(v_80199720));
                            }
                            Err(er) => match er {
                                pg_crud::NotEmptyUnqVecTryNewEr::IsEmpty { .. } => (),
                                pg_crud::NotEmptyUnqVecTryNewEr::NotUnq { .. } => {
                                    panic!("32a3da97")
                                }
                            },
                        }
                    }
                }
                acc_359c0b3f
            }) {
                Ok(v_752f0e8d) => Some(v_752f0e8d),
                Err(er) => match er {
                    pg_crud::NotEmptyUnqVecTryNewEr::IsEmpty { .. } => None,
                    pg_crud::NotEmptyUnqVecTryNewEr::NotUnq { .. } => panic!("76542a11"),
                },
            }
        }
        fn rd_ids_and_cr_into_pg_json_opt_vec_wh_contains_el_greater_than(
            rd_ids: <Self::PgJson as pg_crud::PgJson>::RdIds,
            cr: <Self::PgJson as pg_crud::PgJson>::Cr,
        ) -> Option<
            pg_crud::NotEmptyUnqVec<
                pg_crud::SingleOrMultiple<<Self::PgJson as pg_crud::PgJson>::Wh>,
            >,
        > {
            match pg_crud::NotEmptyUnqVec::try_new({
                let mut acc_359c0b3f = Vec::new();
                for (rd_ids_629675e2, cr_82796400) in rd_ids.0.v.into_iter().zip(cr.0) {
                    let and = pg_crud::Oprtr::And;
                    let id = VecOfObjExampleWithIdAsNnArrOfNnJsonbObjWithIdWh::ElId(
                        pg_crud::PgTypeWh::try_new(
                            and,
                            vec![pg_crud::UuidUuidAsNnJsonbStringWh::Eq(
                                pg_crud::PgJsonWhEq {
                                    oprtr: pg_crud::Oprtr::Or,
                                    v: pg_crud::UuidUuidAsNnJsonbStringTt::new(
                                        rd_ids_629675e2.0.v.id.0.v,
                                    ),
                                },
                            )],
                        )
                        .expect("31db8e1e"),
                    );
                    let field_0 = < pg_crud :: I8AsNnJsonbNbr as pg_crud :: PgJsonTestCases > :: rd_ids_and_cr_into_pg_json_opt_vec_wh_contains_el_greater_than (rd_ids_629675e2 . 0. v . field_0 , cr_82796400 . field_0) ;
                    let field_1 = < pg_crud :: OptI8AsNlJsonbNbr as pg_crud :: PgJsonTestCases > :: rd_ids_and_cr_into_pg_json_opt_vec_wh_contains_el_greater_than (rd_ids_629675e2 . 0. v . field_1 , cr_82796400 . field_1) ;
                    let field_2 = < pg_crud :: VecOfI8AsNnArrOfNnJsonbNbr as pg_crud :: PgJsonTestCases > :: rd_ids_and_cr_into_pg_json_opt_vec_wh_contains_el_greater_than (rd_ids_629675e2 . 0. v . field_2 , cr_82796400 . field_2) ;
                    if field_0.is_some() || field_1.is_some() || field_2.is_some() {
                        let mut all_fields_acc = vec![];
                        if let Some(v_f190793e) = field_0 {
                            for el_22ac4087 in v_f190793e.clone().into_vec() {
                                let wh_f8a4319c =
                                    VecOfObjExampleWithIdAsNnArrOfNnJsonbObjWithIdWh::ElField0(
                                        match el_22ac4087 {
                                            pg_crud::SingleOrMultiple::Multiple(multiple) => {
                                                pg_crud::PgTypeWh::new(and, multiple.clone())
                                            }
                                            pg_crud::SingleOrMultiple::Single(single) => {
                                                pg_crud::PgTypeWh::try_new(and, vec![single])
                                                    .expect("2ed4dc5e")
                                            }
                                        },
                                    );
                                all_fields_acc.push(wh_f8a4319c.clone());
                                match pg_crud::NotEmptyUnqVec::try_new(vec![
                                    id.clone(),
                                    wh_f8a4319c,
                                ]) {
                                    Ok(v_fdd1b3eb) => {
                                        let multiple_wh_with_id_f8a4319c =
                                            pg_crud::SingleOrMultiple::Multiple(v_fdd1b3eb);
                                        if !acc_359c0b3f.contains(&multiple_wh_with_id_f8a4319c) {
                                            acc_359c0b3f.push(multiple_wh_with_id_f8a4319c);
                                        }
                                    }
                                    Err(er) => match er {
                                        pg_crud::NotEmptyUnqVecTryNewEr::IsEmpty { .. } => (),
                                        pg_crud::NotEmptyUnqVecTryNewEr::NotUnq { .. } => {
                                            panic!("f0e3d01b")
                                        }
                                    },
                                }
                            }
                            match pg_crud::NotEmptyUnqVec::try_new(
                                v_f190793e
                                    .into_vec()
                                    .into_iter()
                                    .flat_map(|el| match el {
                                        pg_crud::SingleOrMultiple::Multiple(multiple) => {
                                            multiple.into_vec()
                                        }
                                        pg_crud::SingleOrMultiple::Single(single) => {
                                            std::iter::once(single).collect()
                                        }
                                    })
                                    .fold(Vec::new(), |mut acc_01265629, el| {
                                        if !acc_01265629.contains(&el) {
                                            acc_01265629.push(el);
                                        }
                                        acc_01265629
                                    }),
                            ) {
                                Ok(v_a4000d70) => {
                                    let v_d6218307 =
                                        VecOfObjExampleWithIdAsNnArrOfNnJsonbObjWithIdWh::ElField0(
                                            pg_crud::PgTypeWh::new(and, v_a4000d70),
                                        );
                                    if !all_fields_acc.contains(&v_d6218307) {
                                        all_fields_acc.push(v_d6218307);
                                    }
                                }
                                Err(er) => match er {
                                    pg_crud::NotEmptyUnqVecTryNewEr::IsEmpty { .. } => (),
                                    pg_crud::NotEmptyUnqVecTryNewEr::NotUnq { .. } => {
                                        panic!("f8fcc434")
                                    }
                                },
                            }
                        }
                        if let Some(v_f190793e) = field_1 {
                            for el_22ac4087 in v_f190793e.clone().into_vec() {
                                let wh_f8a4319c =
                                    VecOfObjExampleWithIdAsNnArrOfNnJsonbObjWithIdWh::ElField1(
                                        match el_22ac4087 {
                                            pg_crud::SingleOrMultiple::Multiple(multiple) => {
                                                pg_crud::PgTypeWh::new(and, multiple.clone())
                                            }
                                            pg_crud::SingleOrMultiple::Single(single) => {
                                                pg_crud::PgTypeWh::try_new(and, vec![single])
                                                    .expect("2ed4dc5e")
                                            }
                                        },
                                    );
                                all_fields_acc.push(wh_f8a4319c.clone());
                                match pg_crud::NotEmptyUnqVec::try_new(vec![
                                    id.clone(),
                                    wh_f8a4319c,
                                ]) {
                                    Ok(v_fdd1b3eb) => {
                                        let multiple_wh_with_id_f8a4319c =
                                            pg_crud::SingleOrMultiple::Multiple(v_fdd1b3eb);
                                        if !acc_359c0b3f.contains(&multiple_wh_with_id_f8a4319c) {
                                            acc_359c0b3f.push(multiple_wh_with_id_f8a4319c);
                                        }
                                    }
                                    Err(er) => match er {
                                        pg_crud::NotEmptyUnqVecTryNewEr::IsEmpty { .. } => (),
                                        pg_crud::NotEmptyUnqVecTryNewEr::NotUnq { .. } => {
                                            panic!("f0e3d01b")
                                        }
                                    },
                                }
                            }
                            match pg_crud::NotEmptyUnqVec::try_new(
                                v_f190793e
                                    .into_vec()
                                    .into_iter()
                                    .flat_map(|el| match el {
                                        pg_crud::SingleOrMultiple::Multiple(multiple) => {
                                            multiple.into_vec()
                                        }
                                        pg_crud::SingleOrMultiple::Single(single) => {
                                            std::iter::once(single).collect()
                                        }
                                    })
                                    .fold(Vec::new(), |mut acc_01265629, el| {
                                        if !acc_01265629.contains(&el) {
                                            acc_01265629.push(el);
                                        }
                                        acc_01265629
                                    }),
                            ) {
                                Ok(v_a4000d70) => {
                                    let v_d6218307 =
                                        VecOfObjExampleWithIdAsNnArrOfNnJsonbObjWithIdWh::ElField1(
                                            pg_crud::PgTypeWh::new(and, v_a4000d70),
                                        );
                                    if !all_fields_acc.contains(&v_d6218307) {
                                        all_fields_acc.push(v_d6218307);
                                    }
                                }
                                Err(er) => match er {
                                    pg_crud::NotEmptyUnqVecTryNewEr::IsEmpty { .. } => (),
                                    pg_crud::NotEmptyUnqVecTryNewEr::NotUnq { .. } => {
                                        panic!("f8fcc434")
                                    }
                                },
                            }
                        }
                        if let Some(v_f190793e) = field_2 {
                            for el_22ac4087 in v_f190793e.clone().into_vec() {
                                let wh_f8a4319c =
                                    VecOfObjExampleWithIdAsNnArrOfNnJsonbObjWithIdWh::ElField2(
                                        match el_22ac4087 {
                                            pg_crud::SingleOrMultiple::Multiple(multiple) => {
                                                pg_crud::PgTypeWh::new(and, multiple.clone())
                                            }
                                            pg_crud::SingleOrMultiple::Single(single) => {
                                                pg_crud::PgTypeWh::try_new(and, vec![single])
                                                    .expect("2ed4dc5e")
                                            }
                                        },
                                    );
                                all_fields_acc.push(wh_f8a4319c.clone());
                                match pg_crud::NotEmptyUnqVec::try_new(vec![
                                    id.clone(),
                                    wh_f8a4319c,
                                ]) {
                                    Ok(v_fdd1b3eb) => {
                                        let multiple_wh_with_id_f8a4319c =
                                            pg_crud::SingleOrMultiple::Multiple(v_fdd1b3eb);
                                        if !acc_359c0b3f.contains(&multiple_wh_with_id_f8a4319c) {
                                            acc_359c0b3f.push(multiple_wh_with_id_f8a4319c);
                                        }
                                    }
                                    Err(er) => match er {
                                        pg_crud::NotEmptyUnqVecTryNewEr::IsEmpty { .. } => (),
                                        pg_crud::NotEmptyUnqVecTryNewEr::NotUnq { .. } => {
                                            panic!("f0e3d01b")
                                        }
                                    },
                                }
                            }
                            match pg_crud::NotEmptyUnqVec::try_new(
                                v_f190793e
                                    .into_vec()
                                    .into_iter()
                                    .flat_map(|el| match el {
                                        pg_crud::SingleOrMultiple::Multiple(multiple) => {
                                            multiple.into_vec()
                                        }
                                        pg_crud::SingleOrMultiple::Single(single) => {
                                            std::iter::once(single).collect()
                                        }
                                    })
                                    .fold(Vec::new(), |mut acc_01265629, el| {
                                        if !acc_01265629.contains(&el) {
                                            acc_01265629.push(el);
                                        }
                                        acc_01265629
                                    }),
                            ) {
                                Ok(v_a4000d70) => {
                                    let v_d6218307 =
                                        VecOfObjExampleWithIdAsNnArrOfNnJsonbObjWithIdWh::ElField2(
                                            pg_crud::PgTypeWh::new(and, v_a4000d70),
                                        );
                                    if !all_fields_acc.contains(&v_d6218307) {
                                        all_fields_acc.push(v_d6218307);
                                    }
                                }
                                Err(er) => match er {
                                    pg_crud::NotEmptyUnqVecTryNewEr::IsEmpty { .. } => (),
                                    pg_crud::NotEmptyUnqVecTryNewEr::NotUnq { .. } => {
                                        panic!("f8fcc434")
                                    }
                                },
                            }
                        }
                        match pg_crud::NotEmptyUnqVec::try_new({
                            all_fields_acc.push(id);
                            all_fields_acc
                        }) {
                            Ok(v_80199720) => {
                                acc_359c0b3f.push(pg_crud::SingleOrMultiple::Multiple(v_80199720));
                            }
                            Err(er) => match er {
                                pg_crud::NotEmptyUnqVecTryNewEr::IsEmpty { .. } => (),
                                pg_crud::NotEmptyUnqVecTryNewEr::NotUnq { .. } => {
                                    panic!("32a3da97")
                                }
                            },
                        }
                    }
                }
                acc_359c0b3f
            }) {
                Ok(v_752f0e8d) => Some(v_752f0e8d),
                Err(er) => match er {
                    pg_crud::NotEmptyUnqVecTryNewEr::IsEmpty { .. } => None,
                    pg_crud::NotEmptyUnqVecTryNewEr::NotUnq { .. } => panic!("76542a11"),
                },
            }
        }
        fn rd_ids_and_cr_into_pg_json_opt_vec_wh_contains_el_rgx(
            rd_ids: <Self::PgJson as pg_crud::PgJson>::RdIds,
            cr: <Self::PgJson as pg_crud::PgJson>::Cr,
        ) -> Option<
            pg_crud::NotEmptyUnqVec<
                pg_crud::SingleOrMultiple<<Self::PgJson as pg_crud::PgJson>::Wh>,
            >,
        > {
            match pg_crud::NotEmptyUnqVec::try_new({
                let mut acc_359c0b3f = Vec::new();
                for (rd_ids_629675e2, cr_82796400) in rd_ids.0.v.into_iter().zip(cr.0) {
                    let and = pg_crud::Oprtr::And;
                    let id = VecOfObjExampleWithIdAsNnArrOfNnJsonbObjWithIdWh::ElId(
                        pg_crud::PgTypeWh::try_new(
                            and,
                            vec![pg_crud::UuidUuidAsNnJsonbStringWh::Eq(
                                pg_crud::PgJsonWhEq {
                                    oprtr: pg_crud::Oprtr::Or,
                                    v: pg_crud::UuidUuidAsNnJsonbStringTt::new(
                                        rd_ids_629675e2.0.v.id.0.v,
                                    ),
                                },
                            )],
                        )
                        .expect("31db8e1e"),
                    );
                    let field_0 = < pg_crud :: I8AsNnJsonbNbr as pg_crud :: PgJsonTestCases > :: rd_ids_and_cr_into_pg_json_opt_vec_wh_contains_el_rgx (rd_ids_629675e2 . 0. v . field_0 , cr_82796400 . field_0) ;
                    let field_1 = < pg_crud :: OptI8AsNlJsonbNbr as pg_crud :: PgJsonTestCases > :: rd_ids_and_cr_into_pg_json_opt_vec_wh_contains_el_rgx (rd_ids_629675e2 . 0. v . field_1 , cr_82796400 . field_1) ;
                    let field_2 = < pg_crud :: VecOfI8AsNnArrOfNnJsonbNbr as pg_crud :: PgJsonTestCases > :: rd_ids_and_cr_into_pg_json_opt_vec_wh_contains_el_rgx (rd_ids_629675e2 . 0. v . field_2 , cr_82796400 . field_2) ;
                    if field_0.is_some() || field_1.is_some() || field_2.is_some() {
                        let mut all_fields_acc = vec![];
                        if let Some(v_f190793e) = field_0 {
                            for el_22ac4087 in v_f190793e.clone().into_vec() {
                                let wh_f8a4319c =
                                    VecOfObjExampleWithIdAsNnArrOfNnJsonbObjWithIdWh::ElField0(
                                        match el_22ac4087 {
                                            pg_crud::SingleOrMultiple::Multiple(multiple) => {
                                                pg_crud::PgTypeWh::new(and, multiple.clone())
                                            }
                                            pg_crud::SingleOrMultiple::Single(single) => {
                                                pg_crud::PgTypeWh::try_new(and, vec![single])
                                                    .expect("2ed4dc5e")
                                            }
                                        },
                                    );
                                all_fields_acc.push(wh_f8a4319c.clone());
                                match pg_crud::NotEmptyUnqVec::try_new(vec![
                                    id.clone(),
                                    wh_f8a4319c,
                                ]) {
                                    Ok(v_fdd1b3eb) => {
                                        let multiple_wh_with_id_f8a4319c =
                                            pg_crud::SingleOrMultiple::Multiple(v_fdd1b3eb);
                                        if !acc_359c0b3f.contains(&multiple_wh_with_id_f8a4319c) {
                                            acc_359c0b3f.push(multiple_wh_with_id_f8a4319c);
                                        }
                                    }
                                    Err(er) => match er {
                                        pg_crud::NotEmptyUnqVecTryNewEr::IsEmpty { .. } => (),
                                        pg_crud::NotEmptyUnqVecTryNewEr::NotUnq { .. } => {
                                            panic!("f0e3d01b")
                                        }
                                    },
                                }
                            }
                            match pg_crud::NotEmptyUnqVec::try_new(
                                v_f190793e
                                    .into_vec()
                                    .into_iter()
                                    .flat_map(|el| match el {
                                        pg_crud::SingleOrMultiple::Multiple(multiple) => {
                                            multiple.into_vec()
                                        }
                                        pg_crud::SingleOrMultiple::Single(single) => {
                                            std::iter::once(single).collect()
                                        }
                                    })
                                    .fold(Vec::new(), |mut acc_01265629, el| {
                                        if !acc_01265629.contains(&el) {
                                            acc_01265629.push(el);
                                        }
                                        acc_01265629
                                    }),
                            ) {
                                Ok(v_a4000d70) => {
                                    let v_d6218307 =
                                        VecOfObjExampleWithIdAsNnArrOfNnJsonbObjWithIdWh::ElField0(
                                            pg_crud::PgTypeWh::new(and, v_a4000d70),
                                        );
                                    if !all_fields_acc.contains(&v_d6218307) {
                                        all_fields_acc.push(v_d6218307);
                                    }
                                }
                                Err(er) => match er {
                                    pg_crud::NotEmptyUnqVecTryNewEr::IsEmpty { .. } => (),
                                    pg_crud::NotEmptyUnqVecTryNewEr::NotUnq { .. } => {
                                        panic!("f8fcc434")
                                    }
                                },
                            }
                        }
                        if let Some(v_f190793e) = field_1 {
                            for el_22ac4087 in v_f190793e.clone().into_vec() {
                                let wh_f8a4319c =
                                    VecOfObjExampleWithIdAsNnArrOfNnJsonbObjWithIdWh::ElField1(
                                        match el_22ac4087 {
                                            pg_crud::SingleOrMultiple::Multiple(multiple) => {
                                                pg_crud::PgTypeWh::new(and, multiple.clone())
                                            }
                                            pg_crud::SingleOrMultiple::Single(single) => {
                                                pg_crud::PgTypeWh::try_new(and, vec![single])
                                                    .expect("2ed4dc5e")
                                            }
                                        },
                                    );
                                all_fields_acc.push(wh_f8a4319c.clone());
                                match pg_crud::NotEmptyUnqVec::try_new(vec![
                                    id.clone(),
                                    wh_f8a4319c,
                                ]) {
                                    Ok(v_fdd1b3eb) => {
                                        let multiple_wh_with_id_f8a4319c =
                                            pg_crud::SingleOrMultiple::Multiple(v_fdd1b3eb);
                                        if !acc_359c0b3f.contains(&multiple_wh_with_id_f8a4319c) {
                                            acc_359c0b3f.push(multiple_wh_with_id_f8a4319c);
                                        }
                                    }
                                    Err(er) => match er {
                                        pg_crud::NotEmptyUnqVecTryNewEr::IsEmpty { .. } => (),
                                        pg_crud::NotEmptyUnqVecTryNewEr::NotUnq { .. } => {
                                            panic!("f0e3d01b")
                                        }
                                    },
                                }
                            }
                            match pg_crud::NotEmptyUnqVec::try_new(
                                v_f190793e
                                    .into_vec()
                                    .into_iter()
                                    .flat_map(|el| match el {
                                        pg_crud::SingleOrMultiple::Multiple(multiple) => {
                                            multiple.into_vec()
                                        }
                                        pg_crud::SingleOrMultiple::Single(single) => {
                                            std::iter::once(single).collect()
                                        }
                                    })
                                    .fold(Vec::new(), |mut acc_01265629, el| {
                                        if !acc_01265629.contains(&el) {
                                            acc_01265629.push(el);
                                        }
                                        acc_01265629
                                    }),
                            ) {
                                Ok(v_a4000d70) => {
                                    let v_d6218307 =
                                        VecOfObjExampleWithIdAsNnArrOfNnJsonbObjWithIdWh::ElField1(
                                            pg_crud::PgTypeWh::new(and, v_a4000d70),
                                        );
                                    if !all_fields_acc.contains(&v_d6218307) {
                                        all_fields_acc.push(v_d6218307);
                                    }
                                }
                                Err(er) => match er {
                                    pg_crud::NotEmptyUnqVecTryNewEr::IsEmpty { .. } => (),
                                    pg_crud::NotEmptyUnqVecTryNewEr::NotUnq { .. } => {
                                        panic!("f8fcc434")
                                    }
                                },
                            }
                        }
                        if let Some(v_f190793e) = field_2 {
                            for el_22ac4087 in v_f190793e.clone().into_vec() {
                                let wh_f8a4319c =
                                    VecOfObjExampleWithIdAsNnArrOfNnJsonbObjWithIdWh::ElField2(
                                        match el_22ac4087 {
                                            pg_crud::SingleOrMultiple::Multiple(multiple) => {
                                                pg_crud::PgTypeWh::new(and, multiple.clone())
                                            }
                                            pg_crud::SingleOrMultiple::Single(single) => {
                                                pg_crud::PgTypeWh::try_new(and, vec![single])
                                                    .expect("2ed4dc5e")
                                            }
                                        },
                                    );
                                all_fields_acc.push(wh_f8a4319c.clone());
                                match pg_crud::NotEmptyUnqVec::try_new(vec![
                                    id.clone(),
                                    wh_f8a4319c,
                                ]) {
                                    Ok(v_fdd1b3eb) => {
                                        let multiple_wh_with_id_f8a4319c =
                                            pg_crud::SingleOrMultiple::Multiple(v_fdd1b3eb);
                                        if !acc_359c0b3f.contains(&multiple_wh_with_id_f8a4319c) {
                                            acc_359c0b3f.push(multiple_wh_with_id_f8a4319c);
                                        }
                                    }
                                    Err(er) => match er {
                                        pg_crud::NotEmptyUnqVecTryNewEr::IsEmpty { .. } => (),
                                        pg_crud::NotEmptyUnqVecTryNewEr::NotUnq { .. } => {
                                            panic!("f0e3d01b")
                                        }
                                    },
                                }
                            }
                            match pg_crud::NotEmptyUnqVec::try_new(
                                v_f190793e
                                    .into_vec()
                                    .into_iter()
                                    .flat_map(|el| match el {
                                        pg_crud::SingleOrMultiple::Multiple(multiple) => {
                                            multiple.into_vec()
                                        }
                                        pg_crud::SingleOrMultiple::Single(single) => {
                                            std::iter::once(single).collect()
                                        }
                                    })
                                    .fold(Vec::new(), |mut acc_01265629, el| {
                                        if !acc_01265629.contains(&el) {
                                            acc_01265629.push(el);
                                        }
                                        acc_01265629
                                    }),
                            ) {
                                Ok(v_a4000d70) => {
                                    let v_d6218307 =
                                        VecOfObjExampleWithIdAsNnArrOfNnJsonbObjWithIdWh::ElField2(
                                            pg_crud::PgTypeWh::new(and, v_a4000d70),
                                        );
                                    if !all_fields_acc.contains(&v_d6218307) {
                                        all_fields_acc.push(v_d6218307);
                                    }
                                }
                                Err(er) => match er {
                                    pg_crud::NotEmptyUnqVecTryNewEr::IsEmpty { .. } => (),
                                    pg_crud::NotEmptyUnqVecTryNewEr::NotUnq { .. } => {
                                        panic!("f8fcc434")
                                    }
                                },
                            }
                        }
                        match pg_crud::NotEmptyUnqVec::try_new({
                            all_fields_acc.push(id);
                            all_fields_acc
                        }) {
                            Ok(v_80199720) => {
                                acc_359c0b3f.push(pg_crud::SingleOrMultiple::Multiple(v_80199720));
                            }
                            Err(er) => match er {
                                pg_crud::NotEmptyUnqVecTryNewEr::IsEmpty { .. } => (),
                                pg_crud::NotEmptyUnqVecTryNewEr::NotUnq { .. } => {
                                    panic!("32a3da97")
                                }
                            },
                        }
                    }
                }
                acc_359c0b3f
            }) {
                Ok(v_752f0e8d) => Some(v_752f0e8d),
                Err(er) => match er {
                    pg_crud::NotEmptyUnqVecTryNewEr::IsEmpty { .. } => None,
                    pg_crud::NotEmptyUnqVecTryNewEr::NotUnq { .. } => panic!("76542a11"),
                },
            }
        }
    }
    #[allow(unused_qualifications)]
    #[allow(clippy::absolute_paths)]
    #[allow(clippy::arbitrary_source_item_ordering)]
    #[cfg(feature = "test-utils")]
    #[allow(clippy::float_arithmetic)]
    impl pg_crud::PgTypeTestCases for VecOfObjExampleWithIdAsNnArrOfNnJsonbObjWithId {
        type PgType = Self;
        type Sel = VecOfObjExampleWithIdAsNnArrOfNnJsonbObjWithIdSel;
        fn opt_vec_cr() -> Option<Vec<<Self::PgType as pg_crud::PgType>::Cr>> {
            <Self as pg_crud::PgJsonTestCases>::opt_vec_cr()
        }
        fn rd_ids_to_2_dims_vec_rd_inn(
            rd_ids: &<Self::PgType as pg_crud::PgType>::RdIds,
        ) -> Vec<Vec<<Self::PgType as pg_crud::PgType>::RdInn>> {
            <Self as pg_crud::PgJsonTestCases>::rd_ids_to_2_dims_vec_rd_inn(rd_ids)
        }
        fn rd_inn_into_rd_with_new_or_try_new_unwraped(
            v: VecOfObjExampleWithIdAsNnArrOfNnJsonbObjWithIdRdInn,
        ) -> <Self::PgType as pg_crud::PgType>::Rd {
            <Self as pg_crud::PgJsonTestCases>::rd_inn_into_rd_with_new_or_try_new_unwraped(v)
        }
        fn rd_inn_into_upd_with_new_or_try_new_unwraped(
            v: VecOfObjExampleWithIdAsNnArrOfNnJsonbObjWithIdRdInn,
        ) -> <Self::PgType as pg_crud::PgType>::Upd {
            <Self as pg_crud::PgJsonTestCases>::rd_inn_into_upd_with_new_or_try_new_unwraped(v)
        }
        fn upd_to_rd_ids(
            v: &<Self::PgType as pg_crud::PgType>::Upd,
        ) -> <Self::PgType as pg_crud::PgType>::RdIds {
            <Self as pg_crud::PgJsonTestCases>::upd_to_rd_ids(v)
        }
        fn rd_ids_to_opt_v_rd_dflt_some_one_el(
            v: &<Self::PgType as pg_crud::PgType>::RdIds,
        ) -> Option<pg_crud::V<<Self::PgType as pg_crud::PgType>::Rd>> {
            <Self as pg_crud::PgJsonTestCases>::rd_ids_to_opt_v_rd_dflt_some_one_el(v)
        }
        fn previous_rd_and_opt_upd_into_rd(
            rd: <Self::PgType as pg_crud::PgType>::Rd,
            opt_upd: Option<<Self::PgType as pg_crud::PgType>::Upd>,
        ) -> <Self::PgType as pg_crud::PgType>::Rd {
            <Self as pg_crud::PgJsonTestCases>::previous_rd_and_opt_upd_into_rd(rd, opt_upd)
        }
        fn rd_ids_and_cr_into_rd(
            rd_ids: <Self::PgType as pg_crud::PgType>::RdIds,
            cr: <Self::PgType as pg_crud::PgType>::Cr,
        ) -> <Self::PgType as pg_crud::PgType>::Rd {
            <Self as pg_crud::PgJsonTestCases>::rd_ids_and_cr_into_rd(rd_ids, cr)
        }
        fn rd_ids_and_cr_into_opt_v_rd(
            rd_ids: <Self::PgType as pg_crud::PgType>::RdIds,
            cr: <Self::PgType as pg_crud::PgType>::Cr,
        ) -> Option<pg_crud::V<<Self::PgType as pg_crud::PgType>::Rd>> {
            <Self as pg_crud::PgJsonTestCases>::rd_ids_and_cr_into_opt_v_rd(rd_ids, cr)
        }
        fn rd_ids_and_cr_into_tt(
            rd_ids: <Self::PgType as pg_crud::PgType>::RdIds,
            cr: <Self::PgType as pg_crud::PgType>::Cr,
        ) -> <Self::PgType as pg_crud::PgType>::Tt {
            <Self as pg_crud::PgJsonTestCases>::rd_ids_and_cr_into_tt(rd_ids, cr)
        }
        fn rd_ids_and_cr_into_wh_eq(
            rd_ids: <Self::PgType as pg_crud::PgType>::RdIds,
            cr: <Self::PgType as pg_crud::PgType>::Cr,
        ) -> <Self::PgType as pg_crud::PgType>::Wh {
            <Self as pg_crud::PgJsonTestCases>::rd_ids_and_cr_into_wh_eq(rd_ids, cr)
        }
        fn rd_ids_and_cr_into_vec_wh_eq_using_fields(
            rd_ids: <Self::PgType as pg_crud::PgType>::RdIds,
            cr: <Self::PgType as pg_crud::PgType>::Cr,
        ) -> pg_crud::NotEmptyUnqVec<<Self::PgType as pg_crud::PgType>::Wh> {
            <Self as pg_crud::PgJsonTestCases>::rd_ids_and_cr_into_vec_wh_eq_using_fields(
                rd_ids, cr,
            )
        }
        fn rd_ids_and_cr_into_opt_vec_wh_eq_to_json_field(
            rd_ids: <Self::PgType as pg_crud::PgType>::RdIds,
            cr: <Self::PgType as pg_crud::PgType>::Cr,
        ) -> Option<pg_crud::NotEmptyUnqVec<<Self::PgType as pg_crud::PgType>::Wh>> {
            Some(
                <Self as pg_crud::PgJsonTestCases>::rd_ids_and_cr_into_vec_wh_eq_to_json_field(
                    rd_ids, cr,
                ),
            )
        }
        fn rd_ids_and_cr_into_pg_json_opt_vec_wh_dim_one_eq(
            rd_ids: <Self::PgType as pg_crud::PgType>::RdIds,
            cr: <Self::PgType as pg_crud::PgType>::Cr,
        ) -> Option<pg_crud::NotEmptyUnqVec<<Self::PgType as pg_crud::PgType>::Wh>> {
            <Self as pg_crud::PgJsonTestCases>::rd_ids_and_cr_into_pg_json_opt_vec_wh_dim_one_eq(
                rd_ids, cr,
            )
        }
        fn rd_ids_and_cr_into_pg_json_opt_vec_wh_dim_two_eq(
            rd_ids: <Self::PgType as pg_crud::PgType>::RdIds,
            cr: <Self::PgType as pg_crud::PgType>::Cr,
        ) -> Option<pg_crud::NotEmptyUnqVec<<Self::PgType as pg_crud::PgType>::Wh>> {
            <Self as pg_crud::PgJsonTestCases>::rd_ids_and_cr_into_pg_json_opt_vec_wh_dim_two_eq(
                rd_ids, cr,
            )
        }
        fn rd_ids_and_cr_into_pg_json_opt_vec_wh_dim_three_eq(
            rd_ids: <Self::PgType as pg_crud::PgType>::RdIds,
            cr: <Self::PgType as pg_crud::PgType>::Cr,
        ) -> Option<pg_crud::NotEmptyUnqVec<<Self::PgType as pg_crud::PgType>::Wh>> {
            <Self as pg_crud::PgJsonTestCases>::rd_ids_and_cr_into_pg_json_opt_vec_wh_dim_three_eq(
                rd_ids, cr,
            )
        }
        fn rd_ids_and_cr_into_pg_json_opt_vec_wh_dim_four_eq(
            rd_ids: <Self::PgType as pg_crud::PgType>::RdIds,
            cr: <Self::PgType as pg_crud::PgType>::Cr,
        ) -> Option<pg_crud::NotEmptyUnqVec<<Self::PgType as pg_crud::PgType>::Wh>> {
            <Self as pg_crud::PgJsonTestCases>::rd_ids_and_cr_into_pg_json_opt_vec_wh_dim_four_eq(
                rd_ids, cr,
            )
        }
        fn cr_into_pg_json_opt_vec_wh_len_eq(
            cr: <Self::PgType as pg_crud::PgType>::Cr,
        ) -> Option<pg_crud::NotEmptyUnqVec<<Self::PgType as pg_crud::PgType>::Wh>> {
            <Self as pg_crud::PgJsonTestCases>::cr_into_pg_json_opt_vec_wh_len_eq(cr)
        }
        fn cr_into_pg_json_opt_vec_wh_len_greater_than(
            cr: <Self::PgType as pg_crud::PgType>::Cr,
        ) -> Option<pg_crud::NotEmptyUnqVec<<Self::PgType as pg_crud::PgType>::Wh>> {
            <Self as pg_crud::PgJsonTestCases>::cr_into_pg_json_opt_vec_wh_len_greater_than(cr)
        }
        fn rd_ids_and_cr_into_pg_json_opt_vec_wh_greater_than(
            rd_ids: <Self::PgType as pg_crud::PgType>::RdIds,
            cr: <Self::PgType as pg_crud::PgType>::Cr,
        ) -> Option<
            pg_crud::NotEmptyUnqVec<
                pg_crud::SingleOrMultiple<<Self::PgType as pg_crud::PgType>::Wh>,
            >,
        > {
            <Self as pg_crud::PgJsonTestCases>::rd_ids_and_cr_into_pg_json_opt_vec_wh_greater_than(
                rd_ids, cr,
            )
        }
        fn rd_ids_and_cr_into_pg_json_opt_vec_wh_btwn(
            rd_ids: <Self::PgType as pg_crud::PgType>::RdIds,
            cr: <Self::PgType as pg_crud::PgType>::Cr,
        ) -> Option<
            pg_crud::NotEmptyUnqVec<
                pg_crud::SingleOrMultiple<<Self::PgType as pg_crud::PgType>::Wh>,
            >,
        > {
            <Self as pg_crud::PgJsonTestCases>::rd_ids_and_cr_into_pg_json_opt_vec_wh_btwn(
                rd_ids, cr,
            )
        }
        fn rd_ids_and_cr_into_pg_json_opt_vec_wh_in(
            rd_ids: <Self::PgType as pg_crud::PgType>::RdIds,
            cr: <Self::PgType as pg_crud::PgType>::Cr,
        ) -> Option<
            pg_crud::NotEmptyUnqVec<
                pg_crud::SingleOrMultiple<<Self::PgType as pg_crud::PgType>::Wh>,
            >,
        > {
            <Self as pg_crud::PgJsonTestCases>::rd_ids_and_cr_into_pg_json_opt_vec_wh_in(rd_ids, cr)
        }
        fn rd_ids_and_cr_into_pg_json_opt_vec_wh_rgx(
            rd_ids: <Self::PgType as pg_crud::PgType>::RdIds,
            cr: <Self::PgType as pg_crud::PgType>::Cr,
        ) -> Option<
            pg_crud::NotEmptyUnqVec<
                pg_crud::SingleOrMultiple<<Self::PgType as pg_crud::PgType>::Wh>,
            >,
        > {
            <Self as pg_crud::PgJsonTestCases>::rd_ids_and_cr_into_pg_json_opt_vec_wh_rgx(
                rd_ids, cr,
            )
        }
        fn rd_ids_and_cr_into_pg_json_opt_vec_wh_contains_el_greater_than(
            rd_ids: <Self::PgType as pg_crud::PgType>::RdIds,
            cr: <Self::PgType as pg_crud::PgType>::Cr,
        ) -> Option<
            pg_crud::NotEmptyUnqVec<
                pg_crud::SingleOrMultiple<<Self::PgType as pg_crud::PgType>::Wh>,
            >,
        > {
            < Self as pg_crud :: PgJsonTestCases > :: rd_ids_and_cr_into_pg_json_opt_vec_wh_contains_el_greater_than (rd_ids , cr)
        }
        fn rd_ids_and_cr_into_pg_json_opt_vec_wh_contains_el_rgx(
            rd_ids: <Self::PgType as pg_crud::PgType>::RdIds,
            cr: <Self::PgType as pg_crud::PgType>::Cr,
        ) -> Option<
            pg_crud::NotEmptyUnqVec<
                pg_crud::SingleOrMultiple<<Self::PgType as pg_crud::PgType>::Wh>,
            >,
        > {
            < Self as pg_crud :: PgJsonTestCases > :: rd_ids_and_cr_into_pg_json_opt_vec_wh_contains_el_rgx (rd_ids , cr)
        }
    }
    #[allow(clippy::arbitrary_source_item_ordering)]
    impl pg_crud::PgTypeNotPk for VecOfObjExampleWithIdAsNnArrOfNnJsonbObjWithId {
        type PgType = Self;
        type Cr = VecOfObjExampleWithIdAsNnArrOfNnJsonbObjWithIdCr;
    }
    #[derive(Debug, Clone, Copy)]
    pub struct OptVecOfObjExampleWithIdAsNlArrOfNnJsonbObjWithId;
    #[allow(clippy::arbitrary_source_item_ordering)]
    #[derive(
        Debug,
        Clone,
        PartialEq,
        serde :: Serialize,
        serde :: Deserialize,
        utoipa :: ToSchema,
        schemars :: JsonSchema,
    )]
    pub struct OptVecOfObjExampleWithIdAsNlArrOfNnJsonbObjWithIdTt(
        Option<VecOfObjExampleWithIdAsNnArrOfNnJsonbObjWithIdTt>,
    );
    impl OptVecOfObjExampleWithIdAsNlArrOfNnJsonbObjWithIdTt {
        #[must_use]
        pub fn new(v: Option<Vec<ObjExampleWithIdAsNnJsonbObjWithIdTt>>) -> Self {
            Self(v.map(VecOfObjExampleWithIdAsNnArrOfNnJsonbObjWithIdTt::new))
        }
    }
    impl pg_crud::DfltSomeOneEl for OptVecOfObjExampleWithIdAsNlArrOfNnJsonbObjWithIdTt {
        fn dflt_some_one_el() -> Self {
            Self(Some(pg_crud::DfltSomeOneEl::dflt_some_one_el()))
        }
    }
    impl sqlx::Type<sqlx::Postgres> for OptVecOfObjExampleWithIdAsNlArrOfNnJsonbObjWithIdTt {
        fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> bool {
            <sqlx::types::Json<Self> as sqlx::Type<sqlx::Postgres>>::compatible(ty)
        }
        fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
            <sqlx::types::Json<Self> as sqlx::Type<sqlx::Postgres>>::type_info()
        }
    }
    impl sqlx::Encode<'_, sqlx::Postgres> for OptVecOfObjExampleWithIdAsNlArrOfNnJsonbObjWithIdTt {
        fn encode_by_ref(
            &self,
            buf: &mut sqlx::postgres::PgArgumentBuffer,
        ) -> Result<sqlx::encode::IsNull, Box<dyn std::error::Error + Send + Sync>> {
            sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&sqlx::types::Json(self), buf)
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        serde :: Serialize,
        serde :: Deserialize,
        utoipa :: ToSchema,
        schemars :: JsonSchema,
    )]
    pub struct OptVecOfObjExampleWithIdAsNlArrOfNnJsonbObjWithIdCr(
        Option<VecOfObjExampleWithIdAsNnArrOfNnJsonbObjWithIdCr>,
    );
    impl OptVecOfObjExampleWithIdAsNlArrOfNnJsonbObjWithIdCr {
        #[must_use]
        pub fn new(v: Option<Vec<ObjExampleWithIdAsNnJsonbObjWithIdCr>>) -> Self {
            Self(v.map(VecOfObjExampleWithIdAsNnArrOfNnJsonbObjWithIdCr::new))
        }
    }
    impl pg_crud::DfltSomeOneEl for OptVecOfObjExampleWithIdAsNlArrOfNnJsonbObjWithIdCr {
        fn dflt_some_one_el() -> Self {
            Self(Some(pg_crud::DfltSomeOneEl::dflt_some_one_el()))
        }
    }
    impl sqlx::Type<sqlx::Postgres> for OptVecOfObjExampleWithIdAsNlArrOfNnJsonbObjWithIdCr {
        fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> bool {
            <sqlx::types::Json<Self> as sqlx::Type<sqlx::Postgres>>::compatible(ty)
        }
        fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
            <sqlx::types::Json<Self> as sqlx::Type<sqlx::Postgres>>::type_info()
        }
    }
    impl sqlx::Encode<'_, sqlx::Postgres> for OptVecOfObjExampleWithIdAsNlArrOfNnJsonbObjWithIdCr {
        fn encode_by_ref(
            &self,
            buf: &mut sqlx::postgres::PgArgumentBuffer,
        ) -> Result<sqlx::encode::IsNull, Box<dyn std::error::Error + Send + Sync>> {
            sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&sqlx::types::Json(self), buf)
        }
    }
    impl std::fmt::Display for OptVecOfObjExampleWithIdAsNlArrOfNnJsonbObjWithIdCr {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{self:?}")
        }
    }
    impl loc_lib::ToErrString for OptVecOfObjExampleWithIdAsNlArrOfNnJsonbObjWithIdCr {
        fn to_err_string(&self) -> String {
            format!("{self:#?}")
        }
    }
    #[allow(clippy::arbitrary_source_item_ordering)]
    #[derive(Debug, Clone, PartialEq, serde :: Serialize)]
    pub struct OptVecOfObjExampleWithIdAsNlArrOfNnJsonbObjWithIdCrForQuery(
        Option<<VecOfObjExampleWithIdAsNnArrOfNnJsonbObjWithId as pg_crud::PgJson>::CrForQuery>,
    );
    impl From<OptVecOfObjExampleWithIdAsNlArrOfNnJsonbObjWithIdCr>
        for OptVecOfObjExampleWithIdAsNlArrOfNnJsonbObjWithIdCrForQuery
    {
        fn from(v: OptVecOfObjExampleWithIdAsNlArrOfNnJsonbObjWithIdCr) -> Self {
            Self (v . 0 . map (< VecOfObjExampleWithIdAsNnArrOfNnJsonbObjWithId as pg_crud :: PgJson > :: CrForQuery :: from))
        }
    }
    impl sqlx::Type<sqlx::Postgres> for OptVecOfObjExampleWithIdAsNlArrOfNnJsonbObjWithIdCrForQuery {
        fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> bool {
            <sqlx::types::Json<Self> as sqlx::Type<sqlx::Postgres>>::compatible(ty)
        }
        fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
            <sqlx::types::Json<Self> as sqlx::Type<sqlx::Postgres>>::type_info()
        }
    }
    impl sqlx::Encode<'_, sqlx::Postgres>
        for OptVecOfObjExampleWithIdAsNlArrOfNnJsonbObjWithIdCrForQuery
    {
        fn encode_by_ref(
            &self,
            buf: &mut sqlx::postgres::PgArgumentBuffer,
        ) -> Result<sqlx::encode::IsNull, Box<dyn std::error::Error + Send + Sync>> {
            sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&sqlx::types::Json(self), buf)
        }
    }
    #[allow(clippy::arbitrary_source_item_ordering)]
    #[derive(
        Debug,
        Clone,
        PartialEq,
        serde :: Serialize,
        serde :: Deserialize,
        utoipa :: ToSchema,
        schemars :: JsonSchema,
    )]
    pub struct OptVecOfObjExampleWithIdAsNlArrOfNnJsonbObjWithIdSel(
        Option<<VecOfObjExampleWithIdAsNnArrOfNnJsonbObjWithId as pg_crud::PgJson>::Sel>,
    );
    impl OptVecOfObjExampleWithIdAsNlArrOfNnJsonbObjWithIdSel {
        #[must_use]
        pub const fn new(
            v: Option<<VecOfObjExampleWithIdAsNnArrOfNnJsonbObjWithId as pg_crud::PgJson>::Sel>,
        ) -> Self {
            Self(v)
        }
        fn sel_qp_pg_type(&self, col: &str) -> Result<String, pg_crud::QpEr> {
            let v = self . 0 . as_ref () . map_or_else (< < VecOfObjExampleWithIdAsNnArrOfNnJsonbObjWithId as pg_crud :: PgJson > :: Sel as pg_crud :: DfltSomeOneEl > :: dflt_some_one_el , Clone :: clone) ;
            match v.sel_qp_pg_type(col) {
                Ok(v_c2ca032e) => Ok(format!(
                    "case when jsonb_typeof({col}) = 'null' then 'null'::jsonb else ({v_c2ca032e}) end"
                )),
                Err(er) => Err(er),
            }
        }
    }
    impl sqlx::Type<sqlx::Postgres> for OptVecOfObjExampleWithIdAsNlArrOfNnJsonbObjWithIdSel {
        fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> bool {
            <sqlx::types::Json<Self> as sqlx::Type<sqlx::Postgres>>::compatible(ty)
        }
        fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
            <sqlx::types::Json<Self> as sqlx::Type<sqlx::Postgres>>::type_info()
        }
    }
    impl sqlx::Decode<'_, sqlx::Postgres> for OptVecOfObjExampleWithIdAsNlArrOfNnJsonbObjWithIdSel {
        fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
            match <sqlx::types::Json<Self> as sqlx::Decode<sqlx::Postgres>>::decode(value) {
                Ok(v) => Ok(v.0),
                Err(er) => Err(er),
            }
        }
    }
    impl pg_crud::DfltSomeOneEl for OptVecOfObjExampleWithIdAsNlArrOfNnJsonbObjWithIdSel {
        fn dflt_some_one_el() -> Self {
            Self(Some(pg_crud::DfltSomeOneEl::dflt_some_one_el()))
        }
    }
    impl pg_crud::DfltSomeOneElMaxPageSize for OptVecOfObjExampleWithIdAsNlArrOfNnJsonbObjWithIdSel {
        fn dflt_some_one_el_max_page_size() -> Self {
            Self(Some(
                pg_crud::DfltSomeOneElMaxPageSize::dflt_some_one_el_max_page_size(),
            ))
        }
    }
    pub type OptVecOfObjExampleWithIdAsNlArrOfNnJsonbObjWithIdWh = pg_crud::NlJsonObjPgTypeWhFlt<
        <VecOfObjExampleWithIdAsNnArrOfNnJsonbObjWithId as pg_crud::PgJson>::Wh,
    >;
    #[allow(clippy::arbitrary_source_item_ordering)]
    #[derive(
        Debug,
        Clone,
        PartialEq,
        serde :: Serialize,
        serde :: Deserialize,
        utoipa :: ToSchema,
        schemars :: JsonSchema,
    )]
    pub struct OptVecOfObjExampleWithIdAsNlArrOfNnJsonbObjWithIdRd(
        Option<<VecOfObjExampleWithIdAsNnArrOfNnJsonbObjWithId as pg_crud::PgJson>::Rd>,
    );
    impl OptVecOfObjExampleWithIdAsNlArrOfNnJsonbObjWithIdRd {
        #[must_use]
        pub fn new(v: Option<Vec<ObjExampleWithIdAsNnJsonbObjWithIdRd>>) -> Self {
            Self(
                v.map(<VecOfObjExampleWithIdAsNnArrOfNnJsonbObjWithId as pg_crud::PgJson>::Rd::new),
            )
        }
    }
    impl pg_crud::DfltSomeOneEl for OptVecOfObjExampleWithIdAsNlArrOfNnJsonbObjWithIdRd {
        fn dflt_some_one_el() -> Self {
            Self(Some(pg_crud::DfltSomeOneEl::dflt_some_one_el()))
        }
    }
    impl sqlx::Type<sqlx::Postgres> for OptVecOfObjExampleWithIdAsNlArrOfNnJsonbObjWithIdRd {
        fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> bool {
            <sqlx::types::Json<Self> as sqlx::Type<sqlx::Postgres>>::compatible(ty)
        }
        fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
            <sqlx::types::Json<Self> as sqlx::Type<sqlx::Postgres>>::type_info()
        }
    }
    impl sqlx::Encode<'_, sqlx::Postgres> for OptVecOfObjExampleWithIdAsNlArrOfNnJsonbObjWithIdRd {
        fn encode_by_ref(
            &self,
            buf: &mut sqlx::postgres::PgArgumentBuffer,
        ) -> Result<sqlx::encode::IsNull, Box<dyn std::error::Error + Send + Sync>> {
            sqlx::Encode::<sqlx::Postgres>::encode_by_ref(&sqlx::types::Json(self), buf)
        }
    }
    impl sqlx::Decode<'_, sqlx::Postgres> for OptVecOfObjExampleWithIdAsNlArrOfNnJsonbObjWithIdRd {
        fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
            match <sqlx::types::Json<Self> as sqlx::Decode<sqlx::Postgres>>::decode(value) {
                Ok(v) => Ok(v.0),
                Err(er) => Err(er),
            }
        }
    }
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    pub struct OptVecOfObjExampleWithIdAsNlArrOfNnJsonbObjWithIdRdIds(
        pg_crud::V<Option<VecOfObjExampleWithIdAsNnArrOfNnJsonbObjWithIdRdIds>>,
    );
    impl sqlx::Decode<'_, sqlx::Postgres> for OptVecOfObjExampleWithIdAsNlArrOfNnJsonbObjWithIdRdIds {
        fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
            match <sqlx::types::Json<Self> as sqlx::Decode<sqlx::Postgres>>::decode(value) {
                Ok(v) => Ok(v.0),
                Err(er) => Err(er),
            }
        }
    }
    impl sqlx::Type<sqlx::Postgres> for OptVecOfObjExampleWithIdAsNlArrOfNnJsonbObjWithIdRdIds {
        fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> bool {
            <sqlx::types::Json<Self> as sqlx::Type<sqlx::Postgres>>::compatible(ty)
        }
        fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
            <sqlx::types::Json<Self> as sqlx::Type<sqlx::Postgres>>::type_info()
        }
    }
    pub type OptVecOfObjExampleWithIdAsNlArrOfNnJsonbObjWithIdRdInn =
        Option<<VecOfObjExampleWithIdAsNnArrOfNnJsonbObjWithId as pg_crud::PgJson>::RdInn>;
    #[allow(clippy::arbitrary_source_item_ordering)]
    #[derive(
        Debug,
        Clone,
        PartialEq,
        serde :: Serialize,
        serde :: Deserialize,
        utoipa :: ToSchema,
        schemars :: JsonSchema,
    )]
    pub struct OptVecOfObjExampleWithIdAsNlArrOfNnJsonbObjWithIdUpd(
        Option<<VecOfObjExampleWithIdAsNnArrOfNnJsonbObjWithId as pg_crud::PgJson>::Upd>,
    );
    impl OptVecOfObjExampleWithIdAsNlArrOfNnJsonbObjWithIdUpd {
        #[must_use]
        pub const fn new(
            v: Option<<VecOfObjExampleWithIdAsNnArrOfNnJsonbObjWithId as pg_crud::PgJson>::Upd>,
        ) -> Self {
            Self(v)
        }
    }
    impl pg_crud::DfltSomeOneEl for OptVecOfObjExampleWithIdAsNlArrOfNnJsonbObjWithIdUpd {
        fn dflt_some_one_el() -> Self {
            Self(Some(pg_crud::DfltSomeOneEl::dflt_some_one_el()))
        }
    }
    #[allow(clippy::arbitrary_source_item_ordering)]
    #[derive(Debug, Clone, PartialEq, serde :: Serialize)]
    pub struct OptVecOfObjExampleWithIdAsNlArrOfNnJsonbObjWithIdUpdForQuery(
        Option<<VecOfObjExampleWithIdAsNnArrOfNnJsonbObjWithId as pg_crud::PgJson>::UpdForQuery>,
    );
    impl OptVecOfObjExampleWithIdAsNlArrOfNnJsonbObjWithIdUpdForQuery {
        #[allow(clippy::single_call_fn)]
        fn sel_only_updd_ids_qp(
            &self,
            col_field: &str,
            incr: &mut u64,
        ) -> Result<String, pg_crud::QpEr> {
            Ok (match & self . 0 { Some (v_bc509c9a) => format ! ("jsonb_build_object('v',{})" , match VecOfObjExampleWithIdAsNnArrOfNnJsonbObjWithIdUpdForQuery :: sel_only_updd_ids_qp (v_bc509c9a , col_field , incr) { Ok (v_1e016751) => v_1e016751 , Err (er) => { return Err (er) ; } }) , None => pg_crud :: NULL_JSONB . to_owned () , })
        }
    }
    impl From<<OptVecOfObjExampleWithIdAsNlArrOfNnJsonbObjWithId as pg_crud::PgJson>::Upd>
        for <OptVecOfObjExampleWithIdAsNlArrOfNnJsonbObjWithId as pg_crud::PgJson>::UpdForQuery
    {
        fn from(
            v: <OptVecOfObjExampleWithIdAsNlArrOfNnJsonbObjWithId as pg_crud::PgJson>::Upd,
        ) -> Self {
            Self (v . 0 . map (< VecOfObjExampleWithIdAsNnArrOfNnJsonbObjWithId as pg_crud :: PgJson > :: UpdForQuery :: from))
        }
    }
    #[allow(clippy::arbitrary_source_item_ordering)]
    impl pg_crud::PgJson for OptVecOfObjExampleWithIdAsNlArrOfNnJsonbObjWithId {
        type Tt = OptVecOfObjExampleWithIdAsNlArrOfNnJsonbObjWithIdTt;
        type Cr = OptVecOfObjExampleWithIdAsNlArrOfNnJsonbObjWithIdCr;
        type CrForQuery = OptVecOfObjExampleWithIdAsNlArrOfNnJsonbObjWithIdCrForQuery;
        type Sel = OptVecOfObjExampleWithIdAsNlArrOfNnJsonbObjWithIdSel;
        fn sel_qp(
            v: &Self::Sel,
            fi: &str,
            col_field: &str,
            col_field_for_er_msg: &str,
            _: bool,
        ) -> Result<String, pg_crud::QpEr> {
            let v_174d33cd = v . 0 . as_ref () . map_or_else (< < VecOfObjExampleWithIdAsNnArrOfNnJsonbObjWithId as pg_crud :: PgJson > :: Sel as pg_crud :: DfltSomeOneEl > :: dflt_some_one_el , Clone :: clone) ;
            match <VecOfObjExampleWithIdAsNnArrOfNnJsonbObjWithId as pg_crud::PgJson>::sel_qp(
                &v_174d33cd,
                fi,
                col_field,
                col_field_for_er_msg,
                true,
            ) {
                Ok(v_d7bbd03c) => Ok(format!(
                    "case when jsonb_typeof({col_field}->'{fi}') = 'null' then jsonb_build_object('{fi}',jsonb_build_object('v','null'::jsonb)) else ({v_d7bbd03c}) end"
                )),
                Err(er) => Err(er),
            }
        }
        type Wh = OptVecOfObjExampleWithIdAsNlArrOfNnJsonbObjWithIdWh;
        type Rd = OptVecOfObjExampleWithIdAsNlArrOfNnJsonbObjWithIdRd;
        type RdIds = OptVecOfObjExampleWithIdAsNlArrOfNnJsonbObjWithIdRdIds;
        fn sel_only_ids_qp(col_field: &str) -> Result<String, pg_crud::QpEr> {
            match < VecOfObjExampleWithIdAsNnArrOfNnJsonbObjWithId as pg_crud :: PgJson > :: sel_only_ids_qp (col_field) { Ok (v_21000130) => Ok (format ! ("jsonb_build_object('v',case when jsonb_typeof({col_field}) = 'null' then 'null'::jsonb else ({v_21000130}) end)")) , Err (er) => Err (er) }
        }
        type RdInn = OptVecOfObjExampleWithIdAsNlArrOfNnJsonbObjWithIdRdInn;
        fn into_inn(v: Self::Rd) -> Self::RdInn {
            v.0.map(<VecOfObjExampleWithIdAsNnArrOfNnJsonbObjWithId as pg_crud::PgJson>::into_inn)
        }
        type Upd = OptVecOfObjExampleWithIdAsNlArrOfNnJsonbObjWithIdUpd;
        type UpdForQuery = OptVecOfObjExampleWithIdAsNlArrOfNnJsonbObjWithIdUpdForQuery;
        fn upd_qp(
            v: &Self::UpdForQuery,
            jsonb_set_accumulator: &str,
            jsonb_set_target: &str,
            jsonb_set_path: &str,
            incr: &mut u64,
        ) -> Result<String, pg_crud::QpEr> {
            let _: &str = jsonb_set_accumulator;
            let _: &str = jsonb_set_path;
            match &v.0 {
                Some(v_3245b79f) => {
                    <VecOfObjExampleWithIdAsNnArrOfNnJsonbObjWithId as pg_crud::PgJson>::upd_qp(
                        v_3245b79f,
                        jsonb_set_accumulator,
                        jsonb_set_target,
                        jsonb_set_path,
                        incr,
                    )
                }
                None => match pg_crud::incr_checked_add_one_returning_incr(incr) {
                    Ok(v_87e08bec) => Ok(format!(
                        "jsonb_set({jsonb_set_accumulator},'{{{jsonb_set_path}}}',${v_87e08bec})"
                    )),
                    Err(er) => Err(er),
                },
            }
        }
        fn upd_qb(
            v: Self::UpdForQuery,
            mut query: sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>,
        ) -> Result<sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>, String>
        {
            match v.0 {
                Some(v_a2156b3e) => {
                    <VecOfObjExampleWithIdAsNnArrOfNnJsonbObjWithId as pg_crud::PgJson>::upd_qb(
                        v_a2156b3e, query,
                    )
                }
                None => {
                    if let Err(er) =
                        query.try_bind(sqlx::types::Json(<Self as pg_crud::PgJson>::Upd::new(None)))
                    {
                        Err(er.to_string())
                    } else {
                        Ok(query)
                    }
                }
            }
        }
        fn sel_only_updd_ids_qp(
            v: &Self::UpdForQuery,
            fi: &str,
            col_field: &str,
            incr: &mut u64,
        ) -> Result<String, pg_crud::QpEr> {
            match v.sel_only_updd_ids_qp(&format!("{col_field}->'{fi}'"), incr) {
                Ok(v_e137951b) => Ok(format!("'{fi}',jsonb_build_object('v',{v_e137951b}),")),
                Err(er) => Err(er),
            }
        }
        fn sel_only_updd_ids_qb<'lt>(
            v: &'lt Self::UpdForQuery,
            mut query: sqlx::query::Query<'lt, sqlx::Postgres, sqlx::postgres::PgArguments>,
        ) -> Result<sqlx::query::Query<'lt, sqlx::Postgres, sqlx::postgres::PgArguments>, String>
        {
            if let Some(v_107e6639) = &v.0 {
                match < VecOfObjExampleWithIdAsNnArrOfNnJsonbObjWithId as pg_crud :: PgJson > :: sel_only_updd_ids_qb (v_107e6639 , query) { Ok (v_ecf1b8de) => { query = v_ecf1b8de ; } Err (er) => { return Err (er) ; } }
            }
            Ok(query)
        }
        fn sel_only_crd_ids_qp(
            v: &Self::CrForQuery,
            fi: &str,
            col_field: &str,
            incr: &mut u64,
        ) -> Result<String, pg_crud::QpEr> {
            Ok(format!(
                "'{fi}',jsonb_build_object('v',{}),",
                match &v.0 {
                    Some(v_3c415c92) => format!(
                        "jsonb_build_object('v',(select jsonb_agg({}) from jsonb_array_elements({}) as elem where elem->>'id' in ({})))",
                        {
                            let mut acc_1a91bdc7 = String::new();
                            for el_9bdcd847 in &v_3c415c92.0 {
                                match < pg_crud :: UuidUuidAsNnJsonbString as pg_crud :: PgJson > :: sel_only_crd_ids_qp (& el_9bdcd847 . id , "id" , "elem" , incr) { Ok (mut v_d49fe9d8) => { let _ : Option < char > = v_d49fe9d8 . pop () ; if { use std :: fmt :: Write as _ ; write ! (acc_1a91bdc7 , "jsonb_build_object({v_d49fe9d8})||") } . is_err () { return Err (pg_crud :: QpEr :: WriteIntoBuffer { loc : loc_lib :: loc ! () }) ; } } , Err (er) => { return Err (er) ; } }
                                match < pg_crud :: I8AsNnJsonbNbr as pg_crud :: PgJson > :: sel_only_crd_ids_qp (& el_9bdcd847 . field_0 , "field_0" , "elem" , incr) { Ok (mut v_d49fe9d8) => { let _ : Option < char > = v_d49fe9d8 . pop () ; if { use std :: fmt :: Write as _ ; write ! (acc_1a91bdc7 , "jsonb_build_object({v_d49fe9d8})||") } . is_err () { return Err (pg_crud :: QpEr :: WriteIntoBuffer { loc : loc_lib :: loc ! () }) ; } } , Err (er) => { return Err (er) ; } }
                                match < pg_crud :: OptI8AsNlJsonbNbr as pg_crud :: PgJson > :: sel_only_crd_ids_qp (& el_9bdcd847 . field_1 , "field_1" , "elem" , incr) { Ok (mut v_d49fe9d8) => { let _ : Option < char > = v_d49fe9d8 . pop () ; if { use std :: fmt :: Write as _ ; write ! (acc_1a91bdc7 , "jsonb_build_object({v_d49fe9d8})||") } . is_err () { return Err (pg_crud :: QpEr :: WriteIntoBuffer { loc : loc_lib :: loc ! () }) ; } } , Err (er) => { return Err (er) ; } }
                                match < pg_crud :: VecOfI8AsNnArrOfNnJsonbNbr as pg_crud :: PgJson > :: sel_only_crd_ids_qp (& el_9bdcd847 . field_2 , "field_2" , "elem" , incr) { Ok (mut v_d49fe9d8) => { let _ : Option < char > = v_d49fe9d8 . pop () ; if { use std :: fmt :: Write as _ ; write ! (acc_1a91bdc7 , "jsonb_build_object({v_d49fe9d8})||") } . is_err () { return Err (pg_crud :: QpEr :: WriteIntoBuffer { loc : loc_lib :: loc ! () }) ; } } , Err (er) => { return Err (er) ; } }
                            }
                            let _: Option<char> = acc_1a91bdc7.pop();
                            let _: Option<char> = acc_1a91bdc7.pop();
                            format!("jsonb_build_object('v',{acc_1a91bdc7})")
                        },
                        &format!("{col_field}->'{fi}'"),
                        {
                            let mut acc_857ce631 = String::new();
                            for _ in &v_3c415c92.0 {
                                match pg_crud::incr_checked_add_one_returning_incr(incr) {
                                    Ok(v_7f11bec0) => {
                                        if {
                                            use std::fmt::Write as _;
                                            write!(acc_857ce631, "${v_7f11bec0},")
                                        }
                                        .is_err()
                                        {
                                            return Err(pg_crud::QpEr::WriteIntoBuffer {
                                                loc: loc_lib::loc!(),
                                            });
                                        }
                                    }
                                    Err(er) => {
                                        return Err(er);
                                    }
                                }
                            }
                            let _: Option<char> = acc_857ce631.pop();
                            acc_857ce631
                        }
                    ),
                    None => pg_crud::NULL_JSONB.to_owned(),
                }
            ))
        }
        fn sel_only_crd_ids_qb<'lt>(
            v: &'lt Self::CrForQuery,
            mut query: sqlx::query::Query<'lt, sqlx::Postgres, sqlx::postgres::PgArguments>,
        ) -> Result<sqlx::query::Query<'lt, sqlx::Postgres, sqlx::postgres::PgArguments>, String>
        {
            if let Some(v_0b55a65a) = &v.0 {
                match < VecOfObjExampleWithIdAsNnArrOfNnJsonbObjWithId as pg_crud :: PgJson > :: sel_only_crd_ids_qb (v_0b55a65a , query) { Ok (v_ad6a1ac5) => { query = v_ad6a1ac5 ; } Err (er) => { return Err (er) ; } }
            }
            Ok(query)
        }
    }
    #[allow(clippy::arbitrary_source_item_ordering)]
    impl pg_crud::PgType for OptVecOfObjExampleWithIdAsNlArrOfNnJsonbObjWithId {
        type Tt = OptVecOfObjExampleWithIdAsNlArrOfNnJsonbObjWithIdTt;
        fn cr_tbl_col_qp(col: &dyn std::fmt::Display, _: bool) -> impl std::fmt::Display {
            format!(
                "{col} jsonb not null check (jsonb_matches_schema('{}', {col}))",
                serde_json::to_string(&schemars::schema_for!(
                    OptVecOfObjExampleWithIdAsNlArrOfNnJsonbObjWithIdTt
                ))
                .expect("59a1654b")
            )
        }
        type Cr = OptVecOfObjExampleWithIdAsNlArrOfNnJsonbObjWithIdCr;
        fn cr_qp(_: &Self::Cr, incr: &mut u64) -> Result<String, pg_crud::QpEr> {
            match pg_crud::incr_checked_add_one_returning_incr(incr) {
                Ok(v_7df9eb00) => Ok(format!("${v_7df9eb00}")),
                Err(er) => Err(er),
            }
        }
        fn cr_qb(
            v: Self::Cr,
            mut query: sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>,
        ) -> Result<sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>, String>
        {
            if let Err(er) = query.try_bind(<Self as pg_crud::PgJson>::CrForQuery::from(v)) {
                return Err(er.to_string());
            }
            Ok(query)
        }
        type Sel = OptVecOfObjExampleWithIdAsNlArrOfNnJsonbObjWithIdSel;
        fn sel_qp(v: &Self::Sel, col: &str) -> Result<String, pg_crud::QpEr> {
            match v.sel_qp_pg_type(col) {
                Ok(v_d91c19a6) => Ok(format!("{v_d91c19a6} as {col}")),
                Err(er) => Err(er),
            }
        }
        type Wh = OptVecOfObjExampleWithIdAsNlArrOfNnJsonbObjWithIdWh;
        type Rd = OptVecOfObjExampleWithIdAsNlArrOfNnJsonbObjWithIdRd;
        fn normalize(v: Self::Rd) -> Self::Rd {
            v
        }
        type RdIds = OptVecOfObjExampleWithIdAsNlArrOfNnJsonbObjWithIdRdIds;
        fn sel_only_ids_qp(col: &str) -> Result<String, pg_crud::QpEr> {
            match <Self as pg_crud::PgJson>::sel_only_ids_qp(col) {
                Ok(v_e776e9fa) => Ok(format!("{v_e776e9fa} as {col},")),
                Err(er) => Err(er),
            }
        }
        type RdInn = OptVecOfObjExampleWithIdAsNlArrOfNnJsonbObjWithIdRdInn;
        fn into_inn(v: Self::Rd) -> Self::RdInn {
            <Self as pg_crud::PgJson>::into_inn(v)
        }
        type Upd = OptVecOfObjExampleWithIdAsNlArrOfNnJsonbObjWithIdUpd;
        type UpdForQuery = OptVecOfObjExampleWithIdAsNlArrOfNnJsonbObjWithIdUpdForQuery;
        #[allow(unused_variables)]
        fn upd_qp(
            v: &Self::UpdForQuery,
            jsonb_set_accumulator: &str,
            jsonb_set_target: &str,
            jsonb_set_path: &str,
            incr: &mut u64,
        ) -> Result<String, pg_crud::QpEr> {
            match &v.0 {
                Some(v_58d685d3) => {
                    let upd_qp_acc = {
                        if v_58d685d3.upd.is_empty() {
                            String::from("elem")
                        } else {
                            let mut acc_2e2ad041 = String::default();
                            for el in v_58d685d3.upd.to_vec() {
                                let ident_with_id_h = {
                                    let id_incr = match < pg_crud :: UuidUuidAsNnJsonbString as pg_crud :: PgJsonObjVecElId > :: incr_checked_add_one (incr) { Ok (v_15b44b54) => v_15b44b54 , Err (er) => { return Err (er) ; } } ;
                                    match <ObjExampleAsNnJsonbObj as pg_crud::PgJson>::upd_qp(
                                        &el.fields, "", "elem", "", incr,
                                    ) {
                                        Ok(v_56c44461) => Ok(format!(
                                            "when elem->>'id' = ${id_incr} then {v_56c44461} "
                                        )),
                                        Err(er) => Err(er),
                                    }
                                };
                                match ident_with_id_h {
                                    Ok(v_8333f8f4) => {
                                        if {
                                            use std::fmt::Write as _;
                                            write!(acc_2e2ad041, "{v_8333f8f4}")
                                        }
                                        .is_err()
                                        {
                                            return Err(pg_crud::QpEr::WriteIntoBuffer {
                                                loc: loc_lib::loc!(),
                                            });
                                        }
                                    }
                                    Err(er) => {
                                        return Err(er);
                                    }
                                }
                            }
                            let _: Option<char> = acc_2e2ad041.pop();
                            format!("case {acc_2e2ad041} else elem end")
                        }
                    };
                    let del_qp_acc = {
                        let mut acc_5b4cd920 = String::default();
                        for _ in &v_58d685d3.del {
                            let incr_cb6ba4a7 = match < pg_crud :: UuidUuidAsNnJsonbString as pg_crud :: PgJsonObjVecElId > :: incr_checked_add_one (incr) { Ok (v_110650cc) => v_110650cc , Err (er) => { return Err (er) ; } } ;
                            let mb_space_and_space =
                                if acc_5b4cd920.is_empty() { "" } else { " and " };
                            if {
                                use std::fmt::Write as _;
                                write!(
                                    acc_5b4cd920,
                                    "{mb_space_and_space}elem->>'id' <> ${incr_cb6ba4a7}"
                                )
                            }
                            .is_err()
                            {
                                return Err(pg_crud::QpEr::WriteIntoBuffer {
                                    loc: loc_lib::loc!(),
                                });
                            }
                        }
                        acc_5b4cd920
                    };
                    let cr_qp_acc = {
                        let mut acc_8554f572 = String::default();
                        for _ in &v_58d685d3.cr {
                            let incr_5bbc4961 = match < pg_crud :: UuidUuidAsNnJsonbString as pg_crud :: PgJsonObjVecElId > :: incr_checked_add_one (incr) { Ok (v_27487842) => v_27487842 , Err (er) => { return Err (er) ; } } ;
                            if {
                                use std::fmt::Write as _;
                                write!(acc_8554f572, "${incr_5bbc4961},")
                            }
                            .is_err()
                            {
                                return Err(pg_crud::QpEr::WriteIntoBuffer {
                                    loc: loc_lib::loc!(),
                                });
                            }
                        }
                        let _: Option<char> = acc_8554f572.pop();
                        acc_8554f572
                    };
                    let mb_wh = if v_58d685d3.del.is_empty() {
                        String::default()
                    } else {
                        format!(" where {del_qp_acc}")
                    };
                    let mb_jsonb_build_arr = if v_58d685d3.cr.is_empty() {
                        String::default()
                    } else {
                        format!(" || jsonb_build_arr({cr_qp_acc})")
                    };
                    Ok(format!(
                        "(case when jsonb_typeof({jsonb_set_target}) = 'null' then '[]'::jsonb else (select coalesce((select jsonb_agg({upd_qp_acc}) from jsonb_array_elements({jsonb_set_target}) as elem {mb_wh}),'[]'::jsonb)) end {mb_jsonb_build_arr})"
                    ))
                }
                None => match pg_crud::incr_checked_add_one_returning_incr(incr) {
                    Ok(v_d31ab6f0) => Ok(format!("${v_d31ab6f0}")),
                    Err(er) => Err(er),
                },
            }
        }
        fn upd_qb(
            v: Self::UpdForQuery,
            query: sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>,
        ) -> Result<sqlx::query::Query<'_, sqlx::Postgres, sqlx::postgres::PgArguments>, String>
        {
            <Self as pg_crud::PgJson>::upd_qb(v, query)
        }
        fn sel_only_updd_ids_qp(
            v: &Self::UpdForQuery,
            col: &str,
            incr: &mut u64,
        ) -> Result<String, pg_crud::QpEr> {
            match v.sel_only_updd_ids_qp(col, incr) {
                Ok(v_f0787243) => Ok(format!("jsonb_build_object('v',{v_f0787243}) as {col},")),
                Err(er) => Err(er),
            }
        }
        fn sel_only_updd_ids_qb<'lt>(
            v: &'lt Self::UpdForQuery,
            query: sqlx::query::Query<'lt, sqlx::Postgres, sqlx::postgres::PgArguments>,
        ) -> Result<sqlx::query::Query<'lt, sqlx::Postgres, sqlx::postgres::PgArguments>, String>
        {
            <Self as pg_crud::PgJson>::sel_only_updd_ids_qb(v, query)
        }
    }
    #[allow(unused_qualifications)]
    #[allow(clippy::absolute_paths)]
    #[allow(clippy::arbitrary_source_item_ordering)]
    #[cfg(feature = "test-utils")]
    #[allow(clippy::float_arithmetic)]
    impl pg_crud::PgJsonTestCases for OptVecOfObjExampleWithIdAsNlArrOfNnJsonbObjWithId {
        type PgJson = Self;
        type Sel = OptVecOfObjExampleWithIdAsNlArrOfNnJsonbObjWithIdSel;
        fn opt_vec_cr() -> Option<Vec<<Self::PgJson as pg_crud::PgJson>::Cr>> {
            Some({
                let mut acc_ccd79a32 = Vec::new();
                if let Some (v_399e6a50) = < VecOfObjExampleWithIdAsNnArrOfNnJsonbObjWithId as pg_crud :: PgJsonTestCases > :: opt_vec_cr () { for el in v_399e6a50 { let v = < Self as pg_crud :: PgJson > :: Cr :: new (Some (el . 0)) ; if ! acc_ccd79a32 . contains (& v) { acc_ccd79a32 . push (v) ; } } }
                {
                    let v = <Self as pg_crud::PgJson>::Cr::new(None);
                    if !acc_ccd79a32.contains(&v) {
                        acc_ccd79a32.push(v);
                    }
                }
                acc_ccd79a32
            })
        }
        fn rd_ids_to_2_dims_vec_rd_inn(
            rd_ids: &<Self::PgJson as pg_crud::PgJson>::RdIds,
        ) -> Vec<Vec<<Self::PgJson as pg_crud::PgJson>::RdInn>> {
            let mut acc_fb5111f1 = Vec::new();
            if let Some(v_6ee5750e) = &rd_ids.0.v {
                for el_4a5a4b09 in < VecOfObjExampleWithIdAsNnArrOfNnJsonbObjWithId as pg_crud :: PgJsonTestCases > :: rd_ids_to_2_dims_vec_rd_inn (v_6ee5750e) { for el_264980ec in el_4a5a4b09 { acc_fb5111f1 . push (vec ! [Some (el_264980ec)]) ; } }
            }
            acc_fb5111f1.push(vec![None]);
            acc_fb5111f1
        }
        fn rd_inn_into_rd_with_new_or_try_new_unwraped(
            v: OptVecOfObjExampleWithIdAsNlArrOfNnJsonbObjWithIdRdInn,
        ) -> <Self::PgJson as pg_crud::PgJson>::Rd {
            < Self :: PgJson as pg_crud :: PgType > :: Rd :: new (v . map (| v_189e3c07 | v_189e3c07 . into_iter () . map (| el_ffed1bfc | ObjExampleWithIdAsNnJsonbObjWithIdRd { id : el_ffed1bfc . id . as_ref () . map (| el_5c1f7f63 | pg_crud :: V { v : < pg_crud :: UuidUuidAsNnJsonbString as pg_crud :: PgJsonTestCases > :: rd_inn_into_rd_with_new_or_try_new_unwraped (el_5c1f7f63 . v . clone ()) }) , field_0 : el_ffed1bfc . field_0 . as_ref () . map (| el_5c1f7f63 | pg_crud :: V { v : < pg_crud :: I8AsNnJsonbNbr as pg_crud :: PgJsonTestCases > :: rd_inn_into_rd_with_new_or_try_new_unwraped (el_5c1f7f63 . v . clone ()) }) , field_1 : el_ffed1bfc . field_1 . as_ref () . map (| el_5c1f7f63 | pg_crud :: V { v : < pg_crud :: OptI8AsNlJsonbNbr as pg_crud :: PgJsonTestCases > :: rd_inn_into_rd_with_new_or_try_new_unwraped (el_5c1f7f63 . v . clone ()) }) , field_2 : el_ffed1bfc . field_2 . as_ref () . map (| el_5c1f7f63 | pg_crud :: V { v : < pg_crud :: VecOfI8AsNnArrOfNnJsonbNbr as pg_crud :: PgJsonTestCases > :: rd_inn_into_rd_with_new_or_try_new_unwraped (el_5c1f7f63 . v . clone ()) }) }) . collect ()))
        }
        fn rd_inn_into_upd_with_new_or_try_new_unwraped(
            v: OptVecOfObjExampleWithIdAsNlArrOfNnJsonbObjWithIdRdInn,
        ) -> <Self::PgJson as pg_crud::PgJson>::Upd {
            < Self :: PgJson as pg_crud :: PgType > :: Upd :: new (v . map (< VecOfObjExampleWithIdAsNnArrOfNnJsonbObjWithId as pg_crud :: PgTypeTestCases > :: rd_inn_into_upd_with_new_or_try_new_unwraped))
        }
        fn rd_ids_into_opt_v_rd_inn(
            v: <Self::PgJson as pg_crud::PgJson>::RdIds,
        ) -> Option<pg_crud::V<<Self::PgJson as pg_crud::PgJson>::RdInn>> {
            Some (pg_crud :: V { v : v . 0. v . and_then (| v_f816032d | match < VecOfObjExampleWithIdAsNnArrOfNnJsonbObjWithId as pg_crud :: PgJsonTestCases > :: rd_ids_into_opt_v_rd_inn (v_f816032d) { Some (v_d0549423) => Some (v_d0549423 . v) , None => None , }) })
        }
        fn upd_to_rd_ids(
            v: &<Self::PgJson as pg_crud::PgJson>::Upd,
        ) -> <Self::PgJson as pg_crud::PgJson>::RdIds {
            OptVecOfObjExampleWithIdAsNlArrOfNnJsonbObjWithIdRdIds (pg_crud :: V { v : v . 0 . as_ref () . map (< VecOfObjExampleWithIdAsNnArrOfNnJsonbObjWithId as pg_crud :: PgJsonTestCases > :: upd_to_rd_ids) })
        }
        fn rd_ids_to_opt_v_rd_dflt_some_one_el(
            v: &<Self::PgJson as pg_crud::PgJson>::RdIds,
        ) -> Option<pg_crud::V<<Self::PgJson as pg_crud::PgJson>::Rd>> {
            Some (pg_crud :: V { v : OptVecOfObjExampleWithIdAsNlArrOfNnJsonbObjWithIdRd :: new (v . 0. v . as_ref () . and_then (| v_16ab4136 | match < VecOfObjExampleWithIdAsNnArrOfNnJsonbObjWithId as pg_crud :: PgJsonTestCases > :: rd_ids_to_opt_v_rd_dflt_some_one_el (v_16ab4136) { Some (v_71a66429) => Some (v_71a66429 . v . 0) , None => None , })) })
        }
        fn previous_rd_and_opt_upd_into_rd(
            rd: <Self::PgJson as pg_crud::PgJson>::Rd,
            opt_upd: Option<<Self::PgJson as pg_crud::PgJson>::Upd>,
        ) -> <Self::PgJson as pg_crud::PgJson>::Rd {
            match opt_upd { Some (v_fca601b5) => OptVecOfObjExampleWithIdAsNlArrOfNnJsonbObjWithIdRd (match v_fca601b5 . 0 { Some (v_8d7747f1) => Some (< VecOfObjExampleWithIdAsNnArrOfNnJsonbObjWithId as pg_crud :: PgJsonTestCases > :: previous_rd_and_opt_upd_into_rd (rd . 0 . unwrap_or_else (pg_crud :: DfltSomeOneEl :: dflt_some_one_el) , Some (v_8d7747f1) ,)) , None => None , }) , None => rd , }
        }
        fn rd_ids_and_cr_into_rd(
            rd_ids: <Self::PgJson as pg_crud::PgJson>::RdIds,
            cr: <Self::PgJson as pg_crud::PgJson>::Cr,
        ) -> <Self::PgJson as pg_crud::PgJson>::Rd {
            OptVecOfObjExampleWithIdAsNlArrOfNnJsonbObjWithIdRd :: new (match (rd_ids . 0. v , cr . 0) { (Some (rd_ids_2b2ab8a1) , Some (cr_4a1adaa3)) => { Some (< VecOfObjExampleWithIdAsNnArrOfNnJsonbObjWithId as pg_crud :: PgJsonTestCases > :: rd_ids_and_cr_into_opt_v_rd (rd_ids_2b2ab8a1 , cr_4a1adaa3) . expect ("56ac4450") . v . 0) } , (Some (_) , None) => panic ! ("75be9ae0") , (None , Some (_)) => panic ! ("6a95d7ae") , (None , None) => None , })
        }
        fn rd_ids_and_cr_into_opt_v_rd(
            rd_ids: <Self::PgJson as pg_crud::PgJson>::RdIds,
            cr: <Self::PgJson as pg_crud::PgJson>::Cr,
        ) -> Option<pg_crud::V<<Self::PgJson as pg_crud::PgJson>::Rd>> {
            Some(pg_crud::V {
                v: <Self as pg_crud::PgJsonTestCases>::rd_ids_and_cr_into_rd(rd_ids, cr),
            })
        }
        fn rd_ids_and_cr_into_tt(
            rd_ids: <Self::PgJson as pg_crud::PgJson>::RdIds,
            cr: <Self::PgJson as pg_crud::PgJson>::Cr,
        ) -> <Self::PgJson as pg_crud::PgJson>::Tt {
            OptVecOfObjExampleWithIdAsNlArrOfNnJsonbObjWithIdTt :: new (match (rd_ids . 0. v , cr . 0) { (Some (rd_ids_fb2ec2e4) , Some (cr_2f615d4f)) => { Some (< VecOfObjExampleWithIdAsNnArrOfNnJsonbObjWithId as pg_crud :: PgJsonTestCases > :: rd_ids_and_cr_into_tt (rd_ids_fb2ec2e4 , cr_2f615d4f) . 0) } , (Some (_) , None) => panic ! ("9349dcd5") , (None , Some (_)) => panic ! ("45f8e70a") , (None , None) => None , })
        }
        fn rd_ids_and_cr_into_wh_eq(
            rd_ids: <Self::PgJson as pg_crud::PgJson>::RdIds,
            cr: <Self::PgJson as pg_crud::PgJson>::Cr,
        ) -> <Self::PgJson as pg_crud::PgJson>::Wh {
            pg_crud :: NlJsonObjPgTypeWhFlt (match (rd_ids . 0. v , cr . 0) { (Some (rd_ids_ce30c0fe) , Some (cr_8fd81ed8)) => match pg_crud :: NotEmptyUnqVec :: try_new (vec ! [< VecOfObjExampleWithIdAsNnArrOfNnJsonbObjWithId as pg_crud :: PgJsonTestCases > :: rd_ids_and_cr_into_wh_eq (rd_ids_ce30c0fe , cr_8fd81ed8)]) { Ok (v_7a9cd49b) => Some (v_7a9cd49b) , Err (er) => match er { pg_crud :: NotEmptyUnqVecTryNewEr :: IsEmpty { .. } => None , pg_crud :: NotEmptyUnqVecTryNewEr :: NotUnq { .. } => panic ! ("463769fc") } } , (Some (_) , None) => panic ! ("1a2b314c") , (None , Some (_)) => panic ! ("9faea0f9") , (None , None) => None , })
        }
        fn rd_ids_and_cr_into_vec_wh_eq_using_fields(
            rd_ids: <Self::PgJson as pg_crud::PgJson>::RdIds,
            cr: <Self::PgJson as pg_crud::PgJson>::Cr,
        ) -> pg_crud::NotEmptyUnqVec<<Self::PgJson as pg_crud::PgJson>::Wh> {
            pg_crud :: NotEmptyUnqVec :: try_new (vec ! [pg_crud :: NlJsonObjPgTypeWhFlt (match (rd_ids . 0. v , cr . 0) { (Some (rd_ids_2898c440) , Some (cr_f1c4667c)) => Some (< VecOfObjExampleWithIdAsNnArrOfNnJsonbObjWithId as pg_crud :: PgJsonTestCases > :: rd_ids_and_cr_into_vec_wh_eq_using_fields (rd_ids_2898c440 , cr_f1c4667c)) , (Some (_) , None) => panic ! ("49e4c289") , (None , Some (_)) => panic ! ("ad71caa2") , (None , None) => None , })]) . expect ("ba9c52c1")
        }
        fn rd_ids_and_cr_into_vec_wh_eq_to_json_field(
            rd_ids: <Self::PgJson as pg_crud::PgJson>::RdIds,
            cr: <Self::PgJson as pg_crud::PgJson>::Cr,
        ) -> pg_crud::NotEmptyUnqVec<<Self::PgJson as pg_crud::PgJson>::Wh> {
            <Self as pg_crud::PgJsonTestCases>::rd_ids_and_cr_into_vec_wh_eq_using_fields(
                rd_ids, cr,
            )
        }
        fn rd_ids_and_cr_into_pg_json_opt_vec_wh_dim_one_eq(
            rd_ids: <Self::PgJson as pg_crud::PgJson>::RdIds,
            cr: <Self::PgJson as pg_crud::PgJson>::Cr,
        ) -> Option<pg_crud::NotEmptyUnqVec<<Self::PgJson as pg_crud::PgJson>::Wh>> {
            match pg_crud :: NotEmptyUnqVec :: try_new (match (rd_ids . 0. v , cr . 0) { (Some (rd_ids_cdcb6239) , Some (cr_fdd53941)) => match < VecOfObjExampleWithIdAsNnArrOfNnJsonbObjWithId as pg_crud :: PgJsonTestCases > :: rd_ids_and_cr_into_pg_json_opt_vec_wh_dim_one_eq (rd_ids_cdcb6239 , cr_fdd53941) { Some (v_d6124e21) => { let mut acc_bd78dc08 = Vec :: new () ; for el in v_d6124e21 . clone () . into_vec () { match pg_crud :: NotEmptyUnqVec :: try_new (vec ! [el]) { Ok (v_7ed84f3b) => { acc_bd78dc08 . push (pg_crud :: NlJsonObjPgTypeWhFlt (Some (v_7ed84f3b))) ; } , Err (er) => match er { pg_crud :: NotEmptyUnqVecTryNewEr :: IsEmpty { .. } => () , pg_crud :: NotEmptyUnqVecTryNewEr :: NotUnq { .. } => panic ! ("23dca12f") } } } let v_e48110ec = pg_crud :: NlJsonObjPgTypeWhFlt (Some (v_d6124e21)) ; if ! acc_bd78dc08 . contains (& v_e48110ec) { acc_bd78dc08 . push (v_e48110ec) ; } acc_bd78dc08 } , None => { return None ; } } , (Some (_) , None) => panic ! ("6abeac7b") , (None , Some (_)) => panic ! ("a2761cd2") , (None , None) => vec ! [pg_crud :: NlJsonObjPgTypeWhFlt (None)] }) { Ok (v_55f2dc3d) => Some (v_55f2dc3d) , Err (er) => match er { pg_crud :: NotEmptyUnqVecTryNewEr :: IsEmpty { .. } => None , pg_crud :: NotEmptyUnqVecTryNewEr :: NotUnq { .. } => panic ! ("88912e24") } }
        }
        fn rd_ids_and_cr_into_pg_json_opt_vec_wh_dim_two_eq(
            rd_ids: <Self::PgJson as pg_crud::PgJson>::RdIds,
            cr: <Self::PgJson as pg_crud::PgJson>::Cr,
        ) -> Option<pg_crud::NotEmptyUnqVec<<Self::PgJson as pg_crud::PgJson>::Wh>> {
            match pg_crud :: NotEmptyUnqVec :: try_new (match (rd_ids . 0. v , cr . 0) { (Some (rd_ids_cdcb6239) , Some (cr_fdd53941)) => match < VecOfObjExampleWithIdAsNnArrOfNnJsonbObjWithId as pg_crud :: PgJsonTestCases > :: rd_ids_and_cr_into_pg_json_opt_vec_wh_dim_two_eq (rd_ids_cdcb6239 , cr_fdd53941) { Some (v_d6124e21) => { let mut acc_bd78dc08 = Vec :: new () ; for el in v_d6124e21 . clone () . into_vec () { match pg_crud :: NotEmptyUnqVec :: try_new (vec ! [el]) { Ok (v_7ed84f3b) => { acc_bd78dc08 . push (pg_crud :: NlJsonObjPgTypeWhFlt (Some (v_7ed84f3b))) ; } , Err (er) => match er { pg_crud :: NotEmptyUnqVecTryNewEr :: IsEmpty { .. } => () , pg_crud :: NotEmptyUnqVecTryNewEr :: NotUnq { .. } => panic ! ("23dca12f") } } } let v_e48110ec = pg_crud :: NlJsonObjPgTypeWhFlt (Some (v_d6124e21)) ; if ! acc_bd78dc08 . contains (& v_e48110ec) { acc_bd78dc08 . push (v_e48110ec) ; } acc_bd78dc08 } , None => { return None ; } } , (Some (_) , None) => panic ! ("6abeac7b") , (None , Some (_)) => panic ! ("a2761cd2") , (None , None) => vec ! [pg_crud :: NlJsonObjPgTypeWhFlt (None)] }) { Ok (v_55f2dc3d) => Some (v_55f2dc3d) , Err (er) => match er { pg_crud :: NotEmptyUnqVecTryNewEr :: IsEmpty { .. } => None , pg_crud :: NotEmptyUnqVecTryNewEr :: NotUnq { .. } => panic ! ("88912e24") } }
        }
        fn rd_ids_and_cr_into_pg_json_opt_vec_wh_dim_three_eq(
            rd_ids: <Self::PgJson as pg_crud::PgJson>::RdIds,
            cr: <Self::PgJson as pg_crud::PgJson>::Cr,
        ) -> Option<pg_crud::NotEmptyUnqVec<<Self::PgJson as pg_crud::PgJson>::Wh>> {
            match pg_crud :: NotEmptyUnqVec :: try_new (match (rd_ids . 0. v , cr . 0) { (Some (rd_ids_cdcb6239) , Some (cr_fdd53941)) => match < VecOfObjExampleWithIdAsNnArrOfNnJsonbObjWithId as pg_crud :: PgJsonTestCases > :: rd_ids_and_cr_into_pg_json_opt_vec_wh_dim_three_eq (rd_ids_cdcb6239 , cr_fdd53941) { Some (v_d6124e21) => { let mut acc_bd78dc08 = Vec :: new () ; for el in v_d6124e21 . clone () . into_vec () { match pg_crud :: NotEmptyUnqVec :: try_new (vec ! [el]) { Ok (v_7ed84f3b) => { acc_bd78dc08 . push (pg_crud :: NlJsonObjPgTypeWhFlt (Some (v_7ed84f3b))) ; } , Err (er) => match er { pg_crud :: NotEmptyUnqVecTryNewEr :: IsEmpty { .. } => () , pg_crud :: NotEmptyUnqVecTryNewEr :: NotUnq { .. } => panic ! ("23dca12f") } } } let v_e48110ec = pg_crud :: NlJsonObjPgTypeWhFlt (Some (v_d6124e21)) ; if ! acc_bd78dc08 . contains (& v_e48110ec) { acc_bd78dc08 . push (v_e48110ec) ; } acc_bd78dc08 } , None => { return None ; } } , (Some (_) , None) => panic ! ("6abeac7b") , (None , Some (_)) => panic ! ("a2761cd2") , (None , None) => vec ! [pg_crud :: NlJsonObjPgTypeWhFlt (None)] }) { Ok (v_55f2dc3d) => Some (v_55f2dc3d) , Err (er) => match er { pg_crud :: NotEmptyUnqVecTryNewEr :: IsEmpty { .. } => None , pg_crud :: NotEmptyUnqVecTryNewEr :: NotUnq { .. } => panic ! ("88912e24") } }
        }
        fn rd_ids_and_cr_into_pg_json_opt_vec_wh_dim_four_eq(
            rd_ids: <Self::PgJson as pg_crud::PgJson>::RdIds,
            cr: <Self::PgJson as pg_crud::PgJson>::Cr,
        ) -> Option<pg_crud::NotEmptyUnqVec<<Self::PgJson as pg_crud::PgJson>::Wh>> {
            match pg_crud :: NotEmptyUnqVec :: try_new (match (rd_ids . 0. v , cr . 0) { (Some (rd_ids_cdcb6239) , Some (cr_fdd53941)) => match < VecOfObjExampleWithIdAsNnArrOfNnJsonbObjWithId as pg_crud :: PgJsonTestCases > :: rd_ids_and_cr_into_pg_json_opt_vec_wh_dim_four_eq (rd_ids_cdcb6239 , cr_fdd53941) { Some (v_d6124e21) => { let mut acc_bd78dc08 = Vec :: new () ; for el in v_d6124e21 . clone () . into_vec () { match pg_crud :: NotEmptyUnqVec :: try_new (vec ! [el]) { Ok (v_7ed84f3b) => { acc_bd78dc08 . push (pg_crud :: NlJsonObjPgTypeWhFlt (Some (v_7ed84f3b))) ; } , Err (er) => match er { pg_crud :: NotEmptyUnqVecTryNewEr :: IsEmpty { .. } => () , pg_crud :: NotEmptyUnqVecTryNewEr :: NotUnq { .. } => panic ! ("23dca12f") } } } let v_e48110ec = pg_crud :: NlJsonObjPgTypeWhFlt (Some (v_d6124e21)) ; if ! acc_bd78dc08 . contains (& v_e48110ec) { acc_bd78dc08 . push (v_e48110ec) ; } acc_bd78dc08 } , None => { return None ; } } , (Some (_) , None) => panic ! ("6abeac7b") , (None , Some (_)) => panic ! ("a2761cd2") , (None , None) => vec ! [pg_crud :: NlJsonObjPgTypeWhFlt (None)] }) { Ok (v_55f2dc3d) => Some (v_55f2dc3d) , Err (er) => match er { pg_crud :: NotEmptyUnqVecTryNewEr :: IsEmpty { .. } => None , pg_crud :: NotEmptyUnqVecTryNewEr :: NotUnq { .. } => panic ! ("88912e24") } }
        }
        fn cr_into_pg_json_opt_vec_wh_len_eq(
            cr: <Self::PgJson as pg_crud::PgJson>::Cr,
        ) -> Option<pg_crud::NotEmptyUnqVec<<Self::PgJson as pg_crud::PgJson>::Wh>> {
            match pg_crud :: NotEmptyUnqVec :: try_new (match cr . 0 { Some (cr_09a81dae) => match < VecOfObjExampleWithIdAsNnArrOfNnJsonbObjWithId as pg_crud :: PgJsonTestCases > :: cr_into_pg_json_opt_vec_wh_len_eq (cr_09a81dae) { Some (v_3680a4c9) => { let mut acc_5c441d3a = Vec :: new () ; for el_a8b181a0 in v_3680a4c9 . clone () . into_vec () { match pg_crud :: NotEmptyUnqVec :: try_new (vec ! [el_a8b181a0]) { Ok (v_15097b27) => { acc_5c441d3a . push (pg_crud :: NlJsonObjPgTypeWhFlt (Some (v_15097b27))) ; } , Err (er) => match er { pg_crud :: NotEmptyUnqVecTryNewEr :: IsEmpty { .. } => () , pg_crud :: NotEmptyUnqVecTryNewEr :: NotUnq { .. } => panic ! ("6c4da72e") } } } let v_84ea8e4c = pg_crud :: NlJsonObjPgTypeWhFlt (Some (v_3680a4c9)) ; if ! acc_5c441d3a . contains (& v_84ea8e4c) { acc_5c441d3a . push (v_84ea8e4c) ; } acc_5c441d3a } , None => { return None ; } } , None => vec ! [pg_crud :: NlJsonObjPgTypeWhFlt (None)] , }) { Ok (v_72dbefbc) => Some (v_72dbefbc) , Err (er) => match er { pg_crud :: NotEmptyUnqVecTryNewEr :: IsEmpty { .. } => None , pg_crud :: NotEmptyUnqVecTryNewEr :: NotUnq { .. } => panic ! ("d41bcbca") } }
        }
        fn cr_into_pg_json_opt_vec_wh_len_greater_than(
            cr: <Self::PgJson as pg_crud::PgJson>::Cr,
        ) -> Option<pg_crud::NotEmptyUnqVec<<Self::PgJson as pg_crud::PgJson>::Wh>> {
            cr . 0 . map_or_else (|| None , | cr_612f2a61 | < VecOfObjExampleWithIdAsNnArrOfNnJsonbObjWithId as pg_crud :: PgJsonTestCases > :: cr_into_pg_json_opt_vec_wh_len_greater_than (cr_612f2a61) . map_or_else (|| None , | v_1ea95b5d | match pg_crud :: NotEmptyUnqVec :: try_new ({ let mut acc_87f84b5c = Vec :: new () ; for el_9bbf8527 in v_1ea95b5d . clone () . into_vec () { match pg_crud :: NotEmptyUnqVec :: try_new (vec ! [el_9bbf8527]) { Ok (v_1d0202fc) => { acc_87f84b5c . push (pg_crud :: NlJsonObjPgTypeWhFlt (Some (v_1d0202fc))) ; } Err (er) => match er { pg_crud :: NotEmptyUnqVecTryNewEr :: IsEmpty { .. } => () , pg_crud :: NotEmptyUnqVecTryNewEr :: NotUnq { .. } => panic ! ("bdb0a112") , } , } } let v_4e4cfda3 = pg_crud :: NlJsonObjPgTypeWhFlt (Some (v_1ea95b5d)) ; if ! acc_87f84b5c . contains (& v_4e4cfda3) { acc_87f84b5c . push (v_4e4cfda3) ; } acc_87f84b5c }) { Ok (v_ea4ca151) => Some (v_ea4ca151) , Err (er) => match er { pg_crud :: NotEmptyUnqVecTryNewEr :: IsEmpty { .. } => None , pg_crud :: NotEmptyUnqVecTryNewEr :: NotUnq { .. } => panic ! ("c7ecc36f") , } , } ,))
        }
        fn rd_ids_and_cr_into_pg_json_opt_vec_wh_greater_than(
            rd_ids: <Self::PgJson as pg_crud::PgJson>::RdIds,
            cr: <Self::PgJson as pg_crud::PgJson>::Cr,
        ) -> Option<
            pg_crud::NotEmptyUnqVec<
                pg_crud::SingleOrMultiple<<Self::PgJson as pg_crud::PgJson>::Wh>,
            >,
        > {
            match (rd_ids . 0. v , cr . 0) { (Some (rd_ids_3e2e30c8) , Some (cr_79039a2f)) => < VecOfObjExampleWithIdAsNnArrOfNnJsonbObjWithId as pg_crud :: PgJsonTestCases > :: rd_ids_and_cr_into_pg_json_opt_vec_wh_greater_than (rd_ids_3e2e30c8 , cr_79039a2f) . map_or_else (|| None , | v_35662b3a | match pg_crud :: NotEmptyUnqVec :: try_new ({ let mut acc_e0d72451 = vec ! [] ; for el in v_35662b3a . into_vec () { match el { pg_crud :: SingleOrMultiple :: Multiple (multiple) => { acc_e0d72451 . push (pg_crud :: SingleOrMultiple :: Single (pg_crud :: NlJsonObjPgTypeWhFlt (Some (multiple)))) ; } , pg_crud :: SingleOrMultiple :: Single (single) => match pg_crud :: NotEmptyUnqVec :: try_new (vec ! [single]) { Ok (v_4ce6ecd3) => { acc_e0d72451 . push (pg_crud :: SingleOrMultiple :: Single (pg_crud :: NlJsonObjPgTypeWhFlt (Some (v_4ce6ecd3)))) ; } Err (er) => match er { pg_crud :: NotEmptyUnqVecTryNewEr :: IsEmpty { .. } => () , pg_crud :: NotEmptyUnqVecTryNewEr :: NotUnq { .. } => panic ! ("626ffa77") , } , } , } } acc_e0d72451 }) { Ok (v_5d381053) => Some (v_5d381053) , Err (er) => match er { pg_crud :: NotEmptyUnqVecTryNewEr :: IsEmpty { .. } => None , pg_crud :: NotEmptyUnqVecTryNewEr :: NotUnq { .. } => panic ! ("23a17416") , } , }) , (Some (_) , None) => panic ! ("994082bf") , (None , Some (_)) => panic ! ("04f4d016") , (None , None) => None , }
        }
        fn rd_ids_and_cr_into_pg_json_opt_vec_wh_btwn(
            rd_ids: <Self::PgJson as pg_crud::PgJson>::RdIds,
            cr: <Self::PgJson as pg_crud::PgJson>::Cr,
        ) -> Option<
            pg_crud::NotEmptyUnqVec<
                pg_crud::SingleOrMultiple<<Self::PgJson as pg_crud::PgJson>::Wh>,
            >,
        > {
            match (rd_ids . 0. v , cr . 0) { (Some (rd_ids_3e2e30c8) , Some (cr_79039a2f)) => < VecOfObjExampleWithIdAsNnArrOfNnJsonbObjWithId as pg_crud :: PgJsonTestCases > :: rd_ids_and_cr_into_pg_json_opt_vec_wh_btwn (rd_ids_3e2e30c8 , cr_79039a2f) . map_or_else (|| None , | v_35662b3a | match pg_crud :: NotEmptyUnqVec :: try_new ({ let mut acc_e0d72451 = vec ! [] ; for el in v_35662b3a . into_vec () { match el { pg_crud :: SingleOrMultiple :: Multiple (multiple) => { acc_e0d72451 . push (pg_crud :: SingleOrMultiple :: Single (pg_crud :: NlJsonObjPgTypeWhFlt (Some (multiple)))) ; } , pg_crud :: SingleOrMultiple :: Single (single) => match pg_crud :: NotEmptyUnqVec :: try_new (vec ! [single]) { Ok (v_4ce6ecd3) => { acc_e0d72451 . push (pg_crud :: SingleOrMultiple :: Single (pg_crud :: NlJsonObjPgTypeWhFlt (Some (v_4ce6ecd3)))) ; } Err (er) => match er { pg_crud :: NotEmptyUnqVecTryNewEr :: IsEmpty { .. } => () , pg_crud :: NotEmptyUnqVecTryNewEr :: NotUnq { .. } => panic ! ("626ffa77") , } , } , } } acc_e0d72451 }) { Ok (v_5d381053) => Some (v_5d381053) , Err (er) => match er { pg_crud :: NotEmptyUnqVecTryNewEr :: IsEmpty { .. } => None , pg_crud :: NotEmptyUnqVecTryNewEr :: NotUnq { .. } => panic ! ("23a17416") , } , }) , (Some (_) , None) => panic ! ("994082bf") , (None , Some (_)) => panic ! ("04f4d016") , (None , None) => None , }
        }
        fn rd_ids_and_cr_into_pg_json_opt_vec_wh_in(
            rd_ids: <Self::PgJson as pg_crud::PgJson>::RdIds,
            cr: <Self::PgJson as pg_crud::PgJson>::Cr,
        ) -> Option<
            pg_crud::NotEmptyUnqVec<
                pg_crud::SingleOrMultiple<<Self::PgJson as pg_crud::PgJson>::Wh>,
            >,
        > {
            match (rd_ids . 0. v , cr . 0) { (Some (rd_ids_3e2e30c8) , Some (cr_79039a2f)) => < VecOfObjExampleWithIdAsNnArrOfNnJsonbObjWithId as pg_crud :: PgJsonTestCases > :: rd_ids_and_cr_into_pg_json_opt_vec_wh_in (rd_ids_3e2e30c8 , cr_79039a2f) . map_or_else (|| None , | v_35662b3a | match pg_crud :: NotEmptyUnqVec :: try_new ({ let mut acc_e0d72451 = vec ! [] ; for el in v_35662b3a . into_vec () { match el { pg_crud :: SingleOrMultiple :: Multiple (multiple) => { acc_e0d72451 . push (pg_crud :: SingleOrMultiple :: Single (pg_crud :: NlJsonObjPgTypeWhFlt (Some (multiple)))) ; } , pg_crud :: SingleOrMultiple :: Single (single) => match pg_crud :: NotEmptyUnqVec :: try_new (vec ! [single]) { Ok (v_4ce6ecd3) => { acc_e0d72451 . push (pg_crud :: SingleOrMultiple :: Single (pg_crud :: NlJsonObjPgTypeWhFlt (Some (v_4ce6ecd3)))) ; } Err (er) => match er { pg_crud :: NotEmptyUnqVecTryNewEr :: IsEmpty { .. } => () , pg_crud :: NotEmptyUnqVecTryNewEr :: NotUnq { .. } => panic ! ("626ffa77") , } , } , } } acc_e0d72451 }) { Ok (v_5d381053) => Some (v_5d381053) , Err (er) => match er { pg_crud :: NotEmptyUnqVecTryNewEr :: IsEmpty { .. } => None , pg_crud :: NotEmptyUnqVecTryNewEr :: NotUnq { .. } => panic ! ("23a17416") , } , }) , (Some (_) , None) => panic ! ("994082bf") , (None , Some (_)) => panic ! ("04f4d016") , (None , None) => None , }
        }
        fn rd_ids_and_cr_into_pg_json_opt_vec_wh_rgx(
            rd_ids: <Self::PgJson as pg_crud::PgJson>::RdIds,
            cr: <Self::PgJson as pg_crud::PgJson>::Cr,
        ) -> Option<
            pg_crud::NotEmptyUnqVec<
                pg_crud::SingleOrMultiple<<Self::PgJson as pg_crud::PgJson>::Wh>,
            >,
        > {
            match (rd_ids . 0. v , cr . 0) { (Some (rd_ids_3e2e30c8) , Some (cr_79039a2f)) => < VecOfObjExampleWithIdAsNnArrOfNnJsonbObjWithId as pg_crud :: PgJsonTestCases > :: rd_ids_and_cr_into_pg_json_opt_vec_wh_rgx (rd_ids_3e2e30c8 , cr_79039a2f) . map_or_else (|| None , | v_35662b3a | match pg_crud :: NotEmptyUnqVec :: try_new ({ let mut acc_e0d72451 = vec ! [] ; for el in v_35662b3a . into_vec () { match el { pg_crud :: SingleOrMultiple :: Multiple (multiple) => { acc_e0d72451 . push (pg_crud :: SingleOrMultiple :: Single (pg_crud :: NlJsonObjPgTypeWhFlt (Some (multiple)))) ; } , pg_crud :: SingleOrMultiple :: Single (single) => match pg_crud :: NotEmptyUnqVec :: try_new (vec ! [single]) { Ok (v_4ce6ecd3) => { acc_e0d72451 . push (pg_crud :: SingleOrMultiple :: Single (pg_crud :: NlJsonObjPgTypeWhFlt (Some (v_4ce6ecd3)))) ; } Err (er) => match er { pg_crud :: NotEmptyUnqVecTryNewEr :: IsEmpty { .. } => () , pg_crud :: NotEmptyUnqVecTryNewEr :: NotUnq { .. } => panic ! ("626ffa77") , } , } , } } acc_e0d72451 }) { Ok (v_5d381053) => Some (v_5d381053) , Err (er) => match er { pg_crud :: NotEmptyUnqVecTryNewEr :: IsEmpty { .. } => None , pg_crud :: NotEmptyUnqVecTryNewEr :: NotUnq { .. } => panic ! ("23a17416") , } , }) , (Some (_) , None) => panic ! ("994082bf") , (None , Some (_)) => panic ! ("04f4d016") , (None , None) => None , }
        }
        fn rd_ids_and_cr_into_pg_json_opt_vec_wh_contains_el_greater_than(
            rd_ids: <Self::PgJson as pg_crud::PgJson>::RdIds,
            cr: <Self::PgJson as pg_crud::PgJson>::Cr,
        ) -> Option<
            pg_crud::NotEmptyUnqVec<
                pg_crud::SingleOrMultiple<<Self::PgJson as pg_crud::PgJson>::Wh>,
            >,
        > {
            match (rd_ids . 0. v , cr . 0) { (Some (rd_ids_3e2e30c8) , Some (cr_79039a2f)) => < VecOfObjExampleWithIdAsNnArrOfNnJsonbObjWithId as pg_crud :: PgJsonTestCases > :: rd_ids_and_cr_into_pg_json_opt_vec_wh_contains_el_greater_than (rd_ids_3e2e30c8 , cr_79039a2f) . map_or_else (|| None , | v_35662b3a | match pg_crud :: NotEmptyUnqVec :: try_new ({ let mut acc_e0d72451 = vec ! [] ; for el in v_35662b3a . into_vec () { match el { pg_crud :: SingleOrMultiple :: Multiple (multiple) => { acc_e0d72451 . push (pg_crud :: SingleOrMultiple :: Single (pg_crud :: NlJsonObjPgTypeWhFlt (Some (multiple)))) ; } , pg_crud :: SingleOrMultiple :: Single (single) => match pg_crud :: NotEmptyUnqVec :: try_new (vec ! [single]) { Ok (v_4ce6ecd3) => { acc_e0d72451 . push (pg_crud :: SingleOrMultiple :: Single (pg_crud :: NlJsonObjPgTypeWhFlt (Some (v_4ce6ecd3)))) ; } Err (er) => match er { pg_crud :: NotEmptyUnqVecTryNewEr :: IsEmpty { .. } => () , pg_crud :: NotEmptyUnqVecTryNewEr :: NotUnq { .. } => panic ! ("626ffa77") , } , } , } } acc_e0d72451 }) { Ok (v_5d381053) => Some (v_5d381053) , Err (er) => match er { pg_crud :: NotEmptyUnqVecTryNewEr :: IsEmpty { .. } => None , pg_crud :: NotEmptyUnqVecTryNewEr :: NotUnq { .. } => panic ! ("23a17416") , } , }) , (Some (_) , None) => panic ! ("994082bf") , (None , Some (_)) => panic ! ("04f4d016") , (None , None) => None , }
        }
        fn rd_ids_and_cr_into_pg_json_opt_vec_wh_contains_el_rgx(
            rd_ids: <Self::PgJson as pg_crud::PgJson>::RdIds,
            cr: <Self::PgJson as pg_crud::PgJson>::Cr,
        ) -> Option<
            pg_crud::NotEmptyUnqVec<
                pg_crud::SingleOrMultiple<<Self::PgJson as pg_crud::PgJson>::Wh>,
            >,
        > {
            match (rd_ids . 0. v , cr . 0) { (Some (rd_ids_3e2e30c8) , Some (cr_79039a2f)) => < VecOfObjExampleWithIdAsNnArrOfNnJsonbObjWithId as pg_crud :: PgJsonTestCases > :: rd_ids_and_cr_into_pg_json_opt_vec_wh_contains_el_rgx (rd_ids_3e2e30c8 , cr_79039a2f) . map_or_else (|| None , | v_35662b3a | match pg_crud :: NotEmptyUnqVec :: try_new ({ let mut acc_e0d72451 = vec ! [] ; for el in v_35662b3a . into_vec () { match el { pg_crud :: SingleOrMultiple :: Multiple (multiple) => { acc_e0d72451 . push (pg_crud :: SingleOrMultiple :: Single (pg_crud :: NlJsonObjPgTypeWhFlt (Some (multiple)))) ; } , pg_crud :: SingleOrMultiple :: Single (single) => match pg_crud :: NotEmptyUnqVec :: try_new (vec ! [single]) { Ok (v_4ce6ecd3) => { acc_e0d72451 . push (pg_crud :: SingleOrMultiple :: Single (pg_crud :: NlJsonObjPgTypeWhFlt (Some (v_4ce6ecd3)))) ; } Err (er) => match er { pg_crud :: NotEmptyUnqVecTryNewEr :: IsEmpty { .. } => () , pg_crud :: NotEmptyUnqVecTryNewEr :: NotUnq { .. } => panic ! ("626ffa77") , } , } , } } acc_e0d72451 }) { Ok (v_5d381053) => Some (v_5d381053) , Err (er) => match er { pg_crud :: NotEmptyUnqVecTryNewEr :: IsEmpty { .. } => None , pg_crud :: NotEmptyUnqVecTryNewEr :: NotUnq { .. } => panic ! ("23a17416") , } , }) , (Some (_) , None) => panic ! ("994082bf") , (None , Some (_)) => panic ! ("04f4d016") , (None , None) => None , }
        }
    }
    #[allow(unused_qualifications)]
    #[allow(clippy::absolute_paths)]
    #[allow(clippy::arbitrary_source_item_ordering)]
    #[cfg(feature = "test-utils")]
    #[allow(clippy::float_arithmetic)]
    impl pg_crud::PgTypeTestCases for OptVecOfObjExampleWithIdAsNlArrOfNnJsonbObjWithId {
        type PgType = Self;
        type Sel = OptVecOfObjExampleWithIdAsNlArrOfNnJsonbObjWithIdSel;
        fn opt_vec_cr() -> Option<Vec<<Self::PgType as pg_crud::PgType>::Cr>> {
            <Self as pg_crud::PgJsonTestCases>::opt_vec_cr()
        }
        fn rd_ids_to_2_dims_vec_rd_inn(
            rd_ids: &<Self::PgType as pg_crud::PgType>::RdIds,
        ) -> Vec<Vec<<Self::PgType as pg_crud::PgType>::RdInn>> {
            <Self as pg_crud::PgJsonTestCases>::rd_ids_to_2_dims_vec_rd_inn(rd_ids)
        }
        fn rd_inn_into_rd_with_new_or_try_new_unwraped(
            v: OptVecOfObjExampleWithIdAsNlArrOfNnJsonbObjWithIdRdInn,
        ) -> <Self::PgType as pg_crud::PgType>::Rd {
            <Self as pg_crud::PgJsonTestCases>::rd_inn_into_rd_with_new_or_try_new_unwraped(v)
        }
        fn rd_inn_into_upd_with_new_or_try_new_unwraped(
            v: OptVecOfObjExampleWithIdAsNlArrOfNnJsonbObjWithIdRdInn,
        ) -> <Self::PgType as pg_crud::PgType>::Upd {
            <Self as pg_crud::PgJsonTestCases>::rd_inn_into_upd_with_new_or_try_new_unwraped(v)
        }
        fn upd_to_rd_ids(
            v: &<Self::PgType as pg_crud::PgType>::Upd,
        ) -> <Self::PgType as pg_crud::PgType>::RdIds {
            <Self as pg_crud::PgJsonTestCases>::upd_to_rd_ids(v)
        }
        fn rd_ids_to_opt_v_rd_dflt_some_one_el(
            v: &<Self::PgType as pg_crud::PgType>::RdIds,
        ) -> Option<pg_crud::V<<Self::PgType as pg_crud::PgType>::Rd>> {
            <Self as pg_crud::PgJsonTestCases>::rd_ids_to_opt_v_rd_dflt_some_one_el(v)
        }
        fn previous_rd_and_opt_upd_into_rd(
            rd: <Self::PgType as pg_crud::PgType>::Rd,
            opt_upd: Option<<Self::PgType as pg_crud::PgType>::Upd>,
        ) -> <Self::PgType as pg_crud::PgType>::Rd {
            <Self as pg_crud::PgJsonTestCases>::previous_rd_and_opt_upd_into_rd(rd, opt_upd)
        }
        fn rd_ids_and_cr_into_rd(
            rd_ids: <Self::PgType as pg_crud::PgType>::RdIds,
            cr: <Self::PgType as pg_crud::PgType>::Cr,
        ) -> <Self::PgType as pg_crud::PgType>::Rd {
            <Self as pg_crud::PgJsonTestCases>::rd_ids_and_cr_into_rd(rd_ids, cr)
        }
        fn rd_ids_and_cr_into_opt_v_rd(
            rd_ids: <Self::PgType as pg_crud::PgType>::RdIds,
            cr: <Self::PgType as pg_crud::PgType>::Cr,
        ) -> Option<pg_crud::V<<Self::PgType as pg_crud::PgType>::Rd>> {
            <Self as pg_crud::PgJsonTestCases>::rd_ids_and_cr_into_opt_v_rd(rd_ids, cr)
        }
        fn rd_ids_and_cr_into_tt(
            rd_ids: <Self::PgType as pg_crud::PgType>::RdIds,
            cr: <Self::PgType as pg_crud::PgType>::Cr,
        ) -> <Self::PgType as pg_crud::PgType>::Tt {
            <Self as pg_crud::PgJsonTestCases>::rd_ids_and_cr_into_tt(rd_ids, cr)
        }
        fn rd_ids_and_cr_into_wh_eq(
            rd_ids: <Self::PgType as pg_crud::PgType>::RdIds,
            cr: <Self::PgType as pg_crud::PgType>::Cr,
        ) -> <Self::PgType as pg_crud::PgType>::Wh {
            <Self as pg_crud::PgJsonTestCases>::rd_ids_and_cr_into_wh_eq(rd_ids, cr)
        }
        fn rd_ids_and_cr_into_vec_wh_eq_using_fields(
            rd_ids: <Self::PgType as pg_crud::PgType>::RdIds,
            cr: <Self::PgType as pg_crud::PgType>::Cr,
        ) -> pg_crud::NotEmptyUnqVec<<Self::PgType as pg_crud::PgType>::Wh> {
            <Self as pg_crud::PgJsonTestCases>::rd_ids_and_cr_into_vec_wh_eq_using_fields(
                rd_ids, cr,
            )
        }
        fn rd_ids_and_cr_into_opt_vec_wh_eq_to_json_field(
            rd_ids: <Self::PgType as pg_crud::PgType>::RdIds,
            cr: <Self::PgType as pg_crud::PgType>::Cr,
        ) -> Option<pg_crud::NotEmptyUnqVec<<Self::PgType as pg_crud::PgType>::Wh>> {
            Some(
                <Self as pg_crud::PgJsonTestCases>::rd_ids_and_cr_into_vec_wh_eq_to_json_field(
                    rd_ids, cr,
                ),
            )
        }
        fn rd_ids_and_cr_into_pg_json_opt_vec_wh_dim_one_eq(
            rd_ids: <Self::PgType as pg_crud::PgType>::RdIds,
            cr: <Self::PgType as pg_crud::PgType>::Cr,
        ) -> Option<pg_crud::NotEmptyUnqVec<<Self::PgType as pg_crud::PgType>::Wh>> {
            <Self as pg_crud::PgJsonTestCases>::rd_ids_and_cr_into_pg_json_opt_vec_wh_dim_one_eq(
                rd_ids, cr,
            )
        }
        fn rd_ids_and_cr_into_pg_json_opt_vec_wh_dim_two_eq(
            rd_ids: <Self::PgType as pg_crud::PgType>::RdIds,
            cr: <Self::PgType as pg_crud::PgType>::Cr,
        ) -> Option<pg_crud::NotEmptyUnqVec<<Self::PgType as pg_crud::PgType>::Wh>> {
            <Self as pg_crud::PgJsonTestCases>::rd_ids_and_cr_into_pg_json_opt_vec_wh_dim_two_eq(
                rd_ids, cr,
            )
        }
        fn rd_ids_and_cr_into_pg_json_opt_vec_wh_dim_three_eq(
            rd_ids: <Self::PgType as pg_crud::PgType>::RdIds,
            cr: <Self::PgType as pg_crud::PgType>::Cr,
        ) -> Option<pg_crud::NotEmptyUnqVec<<Self::PgType as pg_crud::PgType>::Wh>> {
            <Self as pg_crud::PgJsonTestCases>::rd_ids_and_cr_into_pg_json_opt_vec_wh_dim_three_eq(
                rd_ids, cr,
            )
        }
        fn rd_ids_and_cr_into_pg_json_opt_vec_wh_dim_four_eq(
            rd_ids: <Self::PgType as pg_crud::PgType>::RdIds,
            cr: <Self::PgType as pg_crud::PgType>::Cr,
        ) -> Option<pg_crud::NotEmptyUnqVec<<Self::PgType as pg_crud::PgType>::Wh>> {
            <Self as pg_crud::PgJsonTestCases>::rd_ids_and_cr_into_pg_json_opt_vec_wh_dim_four_eq(
                rd_ids, cr,
            )
        }
        fn cr_into_pg_json_opt_vec_wh_len_eq(
            cr: <Self::PgType as pg_crud::PgType>::Cr,
        ) -> Option<pg_crud::NotEmptyUnqVec<<Self::PgType as pg_crud::PgType>::Wh>> {
            <Self as pg_crud::PgJsonTestCases>::cr_into_pg_json_opt_vec_wh_len_eq(cr)
        }
        fn cr_into_pg_json_opt_vec_wh_len_greater_than(
            cr: <Self::PgType as pg_crud::PgType>::Cr,
        ) -> Option<pg_crud::NotEmptyUnqVec<<Self::PgType as pg_crud::PgType>::Wh>> {
            <Self as pg_crud::PgJsonTestCases>::cr_into_pg_json_opt_vec_wh_len_greater_than(cr)
        }
        fn rd_ids_and_cr_into_pg_json_opt_vec_wh_greater_than(
            rd_ids: <Self::PgType as pg_crud::PgType>::RdIds,
            cr: <Self::PgType as pg_crud::PgType>::Cr,
        ) -> Option<
            pg_crud::NotEmptyUnqVec<
                pg_crud::SingleOrMultiple<<Self::PgType as pg_crud::PgType>::Wh>,
            >,
        > {
            <Self as pg_crud::PgJsonTestCases>::rd_ids_and_cr_into_pg_json_opt_vec_wh_greater_than(
                rd_ids, cr,
            )
        }
        fn rd_ids_and_cr_into_pg_json_opt_vec_wh_btwn(
            rd_ids: <Self::PgType as pg_crud::PgType>::RdIds,
            cr: <Self::PgType as pg_crud::PgType>::Cr,
        ) -> Option<
            pg_crud::NotEmptyUnqVec<
                pg_crud::SingleOrMultiple<<Self::PgType as pg_crud::PgType>::Wh>,
            >,
        > {
            <Self as pg_crud::PgJsonTestCases>::rd_ids_and_cr_into_pg_json_opt_vec_wh_btwn(
                rd_ids, cr,
            )
        }
        fn rd_ids_and_cr_into_pg_json_opt_vec_wh_in(
            rd_ids: <Self::PgType as pg_crud::PgType>::RdIds,
            cr: <Self::PgType as pg_crud::PgType>::Cr,
        ) -> Option<
            pg_crud::NotEmptyUnqVec<
                pg_crud::SingleOrMultiple<<Self::PgType as pg_crud::PgType>::Wh>,
            >,
        > {
            <Self as pg_crud::PgJsonTestCases>::rd_ids_and_cr_into_pg_json_opt_vec_wh_in(rd_ids, cr)
        }
        fn rd_ids_and_cr_into_pg_json_opt_vec_wh_rgx(
            rd_ids: <Self::PgType as pg_crud::PgType>::RdIds,
            cr: <Self::PgType as pg_crud::PgType>::Cr,
        ) -> Option<
            pg_crud::NotEmptyUnqVec<
                pg_crud::SingleOrMultiple<<Self::PgType as pg_crud::PgType>::Wh>,
            >,
        > {
            <Self as pg_crud::PgJsonTestCases>::rd_ids_and_cr_into_pg_json_opt_vec_wh_rgx(
                rd_ids, cr,
            )
        }
        fn rd_ids_and_cr_into_pg_json_opt_vec_wh_contains_el_greater_than(
            rd_ids: <Self::PgType as pg_crud::PgType>::RdIds,
            cr: <Self::PgType as pg_crud::PgType>::Cr,
        ) -> Option<
            pg_crud::NotEmptyUnqVec<
                pg_crud::SingleOrMultiple<<Self::PgType as pg_crud::PgType>::Wh>,
            >,
        > {
            < Self as pg_crud :: PgJsonTestCases > :: rd_ids_and_cr_into_pg_json_opt_vec_wh_contains_el_greater_than (rd_ids , cr)
        }
        fn rd_ids_and_cr_into_pg_json_opt_vec_wh_contains_el_rgx(
            rd_ids: <Self::PgType as pg_crud::PgType>::RdIds,
            cr: <Self::PgType as pg_crud::PgType>::Cr,
        ) -> Option<
            pg_crud::NotEmptyUnqVec<
                pg_crud::SingleOrMultiple<<Self::PgType as pg_crud::PgType>::Wh>,
            >,
        > {
            < Self as pg_crud :: PgJsonTestCases > :: rd_ids_and_cr_into_pg_json_opt_vec_wh_contains_el_rgx (rd_ids , cr)
        }
    }
    #[allow(clippy::arbitrary_source_item_ordering)]
    impl pg_crud::PgTypeNotPk for OptVecOfObjExampleWithIdAsNlArrOfNnJsonbObjWithId {
        type PgType = Self;
        type Cr = OptVecOfObjExampleWithIdAsNlArrOfNnJsonbObjWithIdCr;
    }
}
pub use obj_example_gen_pg_json_obj_mod::*;
#[derive(Debug, Clone, Copy, optml :: Optml)]
# [pg_crud :: pg_json_obj_config { { "pg_tbl_cols_write_into_pg_tbl_cols_using_pg_json_objs" : "False" , "whole_write_into_gen_pg_json_obj" : "False" , "vrt" : { "is_nl" : "True" , "pattern" : "Arr" , "trait_gen" : "PgTypeAndPgJson" } } }]
pub struct ObjExample {
    pub field_0: pg_crud::I8AsNnJsonbNbr,
    pub field_1: pg_crud::OptI8AsNlJsonbNbr,
    pub field_2: pg_crud::VecOfI8AsNnArrOfNnJsonbNbr,
}
