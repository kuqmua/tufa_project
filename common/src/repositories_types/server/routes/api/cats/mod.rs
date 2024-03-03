#[derive(
    Debug,
    postgresql_crud::GeneratePostgresqlCrud,
)]
#[postgresql_crud::create_many_additional_http_status_codes_error_variants{}]
#[postgresql_crud::create_one_additional_http_status_codes_error_variants{}]
#[postgresql_crud::read_one_additional_http_status_codes_error_variants{}]
#[postgresql_crud::read_many_additional_http_status_codes_error_variants{}]
#[postgresql_crud::update_one_additional_http_status_codes_error_variants{}]
#[postgresql_crud::update_many_additional_http_status_codes_error_variants{}]
#[postgresql_crud::delete_one_additional_http_status_codes_error_variants{}]
#[postgresql_crud::delete_many_additional_http_status_codes_error_variants{}]
#[postgresql_crud::additional_http_status_codes_error_variants{
    #[path(crate::server::extractors::commit_extractor::)]
    enum CommitExtractorCheckErrorNamed {
        #[tvfrr_400_bad_request]
        CommitExtractorNotEqual {
            #[eo_display_with_serialize_deserialize]
            commit_not_equal: std::string::String,
            #[eo_display_with_serialize_deserialize]
            commit_to_use: std::string::String,
            code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
        },
        #[tvfrr_400_bad_request]
        CommitExtractorToStrConversion {
            #[eo_display]
            commit_to_str_conversion: http::header::ToStrError,
            code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
        },
        #[tvfrr_400_bad_request]
        NoCommitExtractorHeader {
            #[eo_display_with_serialize_deserialize]
            no_commit_header: std::string::String,
            code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
        },
    }
    // ;
    // enum SomethingErrorNamed {
    //     #[tvfrr_400_bad_request]
    //     SomethingVariant {
    //         #[eo_display_with_serialize_deserialize]
    //         something_field: std::string::String,
    //         code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    //     },
    // }
}]
pub struct Dog {
    // #[generate_postgresql_crud_primary_key]
    pub id: postgresql_crud::SqlxTypesUuidUuidAsPostgresqlUuidNotNullPrimaryKey,//sqlx::types::Uuid //todo make it UuidWrapper todo - if using js JSON.parse() - must be two variants - for usage and deserialization - coz json number type capacity less than i64::MAX
    // #[StdStringStringAsPostgresqlVarchar]
    pub name: postgresql_crud::StdStringStringAsPostgresqlVarchar,//todo not null
    // #[StdStringStringAsPostgresqlVarchar]
    pub color: postgresql_crud::StdStringStringAsPostgresqlVarchar,//todo not null
}

//////////////

pub const TABLE_NAME: &str = "dogs";
#[derive(Debug, serde :: Serialize, serde :: Deserialize)]
pub struct DogOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: std::option::Option<postgresql_crud::SqlxTypesUuidUuidTryParseWithSerializeDeserialize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: std::option::Option<postgresql_crud::StdStringString>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: std::option::Option<postgresql_crud::StdStringString>,
}
impl std::convert::From<Dog> for DogOptions {
    fn from(value: Dog) -> Self {
        Self {
            id: Some(postgresql_crud::SqlxTypesUuidUuidTryParseWithSerializeDeserialize::from(value.id.0)),
            name: Some(value.name.0),
            color: Some(value.color.0),
        }
    }
}
#[derive(Debug)]
pub struct DogId {
    pub id: postgresql_crud::SqlxTypesUuidUuidTryParseWithSerializeDeserialize,
}
#[derive(Debug)]
pub struct DogName {
    pub name: postgresql_crud::StdStringString,
}
#[derive(Debug)]
pub struct DogColor {
    pub color: postgresql_crud::StdStringString,
}
#[derive(Debug)]
pub struct DogIdName {
    pub id: postgresql_crud::SqlxTypesUuidUuidTryParseWithSerializeDeserialize,
    pub name: postgresql_crud::StdStringString,
}
#[derive(Debug)]
pub struct DogIdColor {
    pub id: postgresql_crud::SqlxTypesUuidUuidTryParseWithSerializeDeserialize,
    pub color: postgresql_crud::StdStringString,
}
#[derive(Debug)]
pub struct DogNameColor {
    pub name: postgresql_crud::StdStringString,
    pub color: postgresql_crud::StdStringString,
}
#[derive(Debug)]
pub struct DogIdNameColor {
    pub id: postgresql_crud::SqlxTypesUuidUuidTryParseWithSerializeDeserialize,
    pub name: postgresql_crud::StdStringString,
    pub color: postgresql_crud::StdStringString,
}
#[derive(Debug, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum DogIdTryFromDogOptionsErrorNamed {
    // UuidWrapperTryFromPossibleUuidWrapper {
    //     #[eo_error_occurence]
    //     uuid_wrapper_try_from_possible_uuid_wrapper:
    //         crate::server::postgres::uuid_wrapper::UuidWrapperTryFromPossibleUuidWrapperErrorNamed,
    //     code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    // },
    IdIsNone {
        #[eo_display_with_serialize_deserialize]
        id_is_none: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
impl std::convert::TryFrom<DogOptions> for DogId {
    type Error = DogIdTryFromDogOptionsErrorNamed;
    fn try_from(value: DogOptions) -> Result<Self, Self::Error> {
        let id =
            match value.id {
                Some(value) => {
                    value
                    // match crate::server::postgres::uuid_wrapper::UuidWrapper::try_from(value) {
                    //     Ok(value) => value.into_inner(),
                    //     Err(e) => {
                    //         return Err(Self :: Error :: UuidWrapperTryFromPossibleUuidWrapper
                    // {
                    //     uuid_wrapper_try_from_possible_uuid_wrapper : e,
                    //     code_occurence : error_occurence_lib :: code_occurence ::
                    //     CodeOccurence ::
                    //     new(crate :: global_variables :: compile_time ::
                    //     project_git_info :: PROJECT_GIT_INFO.commit.to_string(),
                    //     file! ().to_string(), line! (), column! (),
                    //     Some(error_occurence_lib :: code_occurence :: MacroOccurence
                    //     {
                    //         file : std :: string :: String ::
                    //         from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                    //         line : 619, column : 41,
                    //     })),
                    // }) ;
                    //     }
                    // }
                }
                None => {
                    return Err(Self :: Error :: IdIsNone
                {
                    id_is_none : std :: string :: String :: from("id is None"),
                    code_occurence : error_occurence_lib :: code_occurence ::
                    CodeOccurence ::
                    new(crate :: global_variables :: compile_time ::
                    project_git_info :: PROJECT_GIT_INFO.commit.to_string(),
                    file! ().to_string(), line! (), column! (),
                    Some(error_occurence_lib :: code_occurence :: MacroOccurence
                    {
                        file : std :: string :: String ::
                        from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                        line : 625, column : 41,
                    })),
                }) ;
                }
            };
        Ok(Self { id })
    }
}
#[derive(Debug, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum DogNameTryFromDogOptionsErrorNamed {
    NameIsNone {
        #[eo_display_with_serialize_deserialize]
        name_is_none: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
impl std::convert::TryFrom<DogOptions> for DogName {
    type Error = DogNameTryFromDogOptionsErrorNamed;
    fn try_from(value: DogOptions) -> Result<Self, Self::Error> {
        let name =
            match value.name {
                Some(value) => value,
                None => {
                    return
                Err(Self :: Error :: NameIsNone
                {
                    name_is_none : std :: string :: String ::
                    from("name is None"), code_occurence : error_occurence_lib
                    :: code_occurence :: CodeOccurence ::
                    new(crate :: global_variables :: compile_time ::
                    project_git_info :: PROJECT_GIT_INFO.commit.to_string(),
                    file! ().to_string(), line! (), column! (),
                    Some(error_occurence_lib :: code_occurence :: MacroOccurence
                    {
                        file : std :: string :: String ::
                        from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                        line : 684, column : 29,
                    })),
                }) ;
                }
            };
        Ok(Self { name })
    }
}
#[derive(Debug, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum DogColorTryFromDogOptionsErrorNamed {
    ColorIsNone {
        #[eo_display_with_serialize_deserialize]
        color_is_none: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
impl std::convert::TryFrom<DogOptions> for DogColor {
    type Error = DogColorTryFromDogOptionsErrorNamed;
    fn try_from(value: DogOptions) -> Result<Self, Self::Error> {
        let color =
            match value.color {
                Some(value) => value,
                None => {
                    return
                Err(Self :: Error :: ColorIsNone
                {
                    color_is_none : std :: string :: String ::
                    from("color is None"), code_occurence : error_occurence_lib
                    :: code_occurence :: CodeOccurence ::
                    new(crate :: global_variables :: compile_time ::
                    project_git_info :: PROJECT_GIT_INFO.commit.to_string(),
                    file! ().to_string(), line! (), column! (),
                    Some(error_occurence_lib :: code_occurence :: MacroOccurence
                    {
                        file : std :: string :: String ::
                        from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                        line : 684, column : 29,
                    })),
                }) ;
                }
            };
        Ok(Self { color })
    }
}
#[derive(Debug, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum DogIdNameTryFromDogOptionsErrorNamed {
    UuidWrapperTryFromPossibleUuidWrapper {
        #[eo_error_occurence]
        uuid_wrapper_try_from_possible_uuid_wrapper:
            crate::server::postgres::uuid_wrapper::UuidWrapperTryFromPossibleUuidWrapperErrorNamed,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    IdIsNone {
        #[eo_display_with_serialize_deserialize]
        id_is_none: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NameIsNone {
        #[eo_display_with_serialize_deserialize]
        name_is_none: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
impl std::convert::TryFrom<DogOptions> for DogIdName {
    type Error = DogIdNameTryFromDogOptionsErrorNamed;
    fn try_from(value: DogOptions) -> Result<Self, Self::Error> {
        let id =
            match value.id {
                Some(value) => {
                    // match crate::server::postgres::uuid_wrapper::UuidWrapper::try_from(value) {
                    //     Ok(value) => value.into_inner(),
                    //     Err(e) => {
                    //         return
                    // Err(Self :: Error :: UuidWrapperTryFromPossibleUuidWrapper
                    // {
                    //     uuid_wrapper_try_from_possible_uuid_wrapper : e,
                    //     code_occurence : error_occurence_lib :: code_occurence ::
                    //     CodeOccurence ::
                    //     new(crate :: global_variables :: compile_time ::
                    //     project_git_info :: PROJECT_GIT_INFO.commit.to_string(),
                    //     file! ().to_string(), line! (), column! (),
                    //     Some(error_occurence_lib :: code_occurence :: MacroOccurence
                    //     {
                    //         file : std :: string :: String ::
                    //         from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                    //         line : 619, column : 41,
                    //     })),
                    // }) ;
                    //     }
                    // }
                    value
                }
                None => {
                    return
                Err(Self :: Error :: IdIsNone
                {
                    id_is_none : std :: string :: String :: from("id is None"),
                    code_occurence : error_occurence_lib :: code_occurence ::
                    CodeOccurence ::
                    new(crate :: global_variables :: compile_time ::
                    project_git_info :: PROJECT_GIT_INFO.commit.to_string(),
                    file! ().to_string(), line! (), column! (),
                    Some(error_occurence_lib :: code_occurence :: MacroOccurence
                    {
                        file : std :: string :: String ::
                        from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                        line : 625, column : 41,
                    })),
                }) ;
                }
            };
        let name =
            match value.name {
                Some(value) => value,
                None => {
                    return
                Err(Self :: Error :: NameIsNone
                {
                    name_is_none : std :: string :: String ::
                    from("name is None"), code_occurence : error_occurence_lib
                    :: code_occurence :: CodeOccurence ::
                    new(crate :: global_variables :: compile_time ::
                    project_git_info :: PROJECT_GIT_INFO.commit.to_string(),
                    file! ().to_string(), line! (), column! (),
                    Some(error_occurence_lib :: code_occurence :: MacroOccurence
                    {
                        file : std :: string :: String ::
                        from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                        line : 684, column : 29,
                    })),
                }) ;
                }
            };
        Ok(Self { id, name })
    }
}
#[derive(Debug, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum DogIdColorTryFromDogOptionsErrorNamed {
    UuidWrapperTryFromPossibleUuidWrapper {
        #[eo_error_occurence]
        uuid_wrapper_try_from_possible_uuid_wrapper:
            crate::server::postgres::uuid_wrapper::UuidWrapperTryFromPossibleUuidWrapperErrorNamed,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    IdIsNone {
        #[eo_display_with_serialize_deserialize]
        id_is_none: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    ColorIsNone {
        #[eo_display_with_serialize_deserialize]
        color_is_none: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
impl std::convert::TryFrom<DogOptions> for DogIdColor {
    type Error = DogIdColorTryFromDogOptionsErrorNamed;
    fn try_from(value: DogOptions) -> Result<Self, Self::Error> {
        let id =
            match value.id {
                Some(value) => {
                    // match crate::server::postgres::uuid_wrapper::UuidWrapper::try_from(value) {
                    //     Ok(value) => value.into_inner(),
                    //     Err(e) => {
                    //         return
                    // Err(Self :: Error :: UuidWrapperTryFromPossibleUuidWrapper
                    // {
                    //     uuid_wrapper_try_from_possible_uuid_wrapper : e,
                    //     code_occurence : error_occurence_lib :: code_occurence ::
                    //     CodeOccurence ::
                    //     new(crate :: global_variables :: compile_time ::
                    //     project_git_info :: PROJECT_GIT_INFO.commit.to_string(),
                    //     file! ().to_string(), line! (), column! (),
                    //     Some(error_occurence_lib :: code_occurence :: MacroOccurence
                    //     {
                    //         file : std :: string :: String ::
                    //         from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                    //         line : 619, column : 41,
                    //     })),
                    // }) ;
                    //     }
                    // }
                    value
                }
                None => {
                    return
                Err(Self :: Error :: IdIsNone
                {
                    id_is_none : std :: string :: String :: from("id is None"),
                    code_occurence : error_occurence_lib :: code_occurence ::
                    CodeOccurence ::
                    new(crate :: global_variables :: compile_time ::
                    project_git_info :: PROJECT_GIT_INFO.commit.to_string(),
                    file! ().to_string(), line! (), column! (),
                    Some(error_occurence_lib :: code_occurence :: MacroOccurence
                    {
                        file : std :: string :: String ::
                        from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                        line : 625, column : 41,
                    })),
                }) ;
                }
            };
        let color =
            match value.color {
                Some(value) => value,
                None => {
                    return
                Err(Self :: Error :: ColorIsNone
                {
                    color_is_none : std :: string :: String ::
                    from("color is None"), code_occurence : error_occurence_lib
                    :: code_occurence :: CodeOccurence ::
                    new(crate :: global_variables :: compile_time ::
                    project_git_info :: PROJECT_GIT_INFO.commit.to_string(),
                    file! ().to_string(), line! (), column! (),
                    Some(error_occurence_lib :: code_occurence :: MacroOccurence
                    {
                        file : std :: string :: String ::
                        from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                        line : 684, column : 29,
                    })),
                }) ;
                }
            };
        Ok(Self { id, color })
    }
}
#[derive(Debug, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum DogNameColorTryFromDogOptionsErrorNamed {
    NameIsNone {
        #[eo_display_with_serialize_deserialize]
        name_is_none: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    ColorIsNone {
        #[eo_display_with_serialize_deserialize]
        color_is_none: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
impl std::convert::TryFrom<DogOptions> for DogNameColor {
    type Error = DogNameColorTryFromDogOptionsErrorNamed;
    fn try_from(value: DogOptions) -> Result<Self, Self::Error> {
        let name =
            match value.name {
                Some(value) => value,
                None => {
                    return
                Err(Self :: Error :: NameIsNone
                {
                    name_is_none : std :: string :: String ::
                    from("name is None"), code_occurence : error_occurence_lib
                    :: code_occurence :: CodeOccurence ::
                    new(crate :: global_variables :: compile_time ::
                    project_git_info :: PROJECT_GIT_INFO.commit.to_string(),
                    file! ().to_string(), line! (), column! (),
                    Some(error_occurence_lib :: code_occurence :: MacroOccurence
                    {
                        file : std :: string :: String ::
                        from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                        line : 684, column : 29,
                    })),
                }) ;
                }
            };
        let color =
            match value.color {
                Some(value) => value,
                None => {
                    return
                Err(Self :: Error :: ColorIsNone
                {
                    color_is_none : std :: string :: String ::
                    from("color is None"), code_occurence : error_occurence_lib
                    :: code_occurence :: CodeOccurence ::
                    new(crate :: global_variables :: compile_time ::
                    project_git_info :: PROJECT_GIT_INFO.commit.to_string(),
                    file! ().to_string(), line! (), column! (),
                    Some(error_occurence_lib :: code_occurence :: MacroOccurence
                    {
                        file : std :: string :: String ::
                        from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                        line : 684, column : 29,
                    })),
                }) ;
                }
            };
        Ok(Self { name, color })
    }
}
#[derive(Debug, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum DogIdNameColorTryFromDogOptionsErrorNamed {
    UuidWrapperTryFromPossibleUuidWrapper {
        #[eo_error_occurence]
        uuid_wrapper_try_from_possible_uuid_wrapper:
            crate::server::postgres::uuid_wrapper::UuidWrapperTryFromPossibleUuidWrapperErrorNamed,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    IdIsNone {
        #[eo_display_with_serialize_deserialize]
        id_is_none: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NameIsNone {
        #[eo_display_with_serialize_deserialize]
        name_is_none: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    ColorIsNone {
        #[eo_display_with_serialize_deserialize]
        color_is_none: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
impl std::convert::TryFrom<DogOptions> for DogIdNameColor {
    type Error = DogIdNameColorTryFromDogOptionsErrorNamed;
    fn try_from(value: DogOptions) -> Result<Self, Self::Error> {
        let id =
            match value.id {
                Some(value) => {
                    // match crate::server::postgres::uuid_wrapper::UuidWrapper::try_from(value) {
                    //     Ok(value) => value.into_inner(),
                    //     Err(e) => {
                    //         return
                    // Err(Self :: Error :: UuidWrapperTryFromPossibleUuidWrapper
                    // {
                    //     uuid_wrapper_try_from_possible_uuid_wrapper : e,
                    //     code_occurence : error_occurence_lib :: code_occurence ::
                    //     CodeOccurence ::
                    //     new(crate :: global_variables :: compile_time ::
                    //     project_git_info :: PROJECT_GIT_INFO.commit.to_string(),
                    //     file! ().to_string(), line! (), column! (),
                    //     Some(error_occurence_lib :: code_occurence :: MacroOccurence
                    //     {
                    //         file : std :: string :: String ::
                    //         from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                    //         line : 619, column : 41,
                    //     })),
                    // }) ;
                    //     }
                    // }
                    value
                }
                None => {
                    return
                Err(Self :: Error :: IdIsNone
                {
                    id_is_none : std :: string :: String :: from("id is None"),
                    code_occurence : error_occurence_lib :: code_occurence ::
                    CodeOccurence ::
                    new(crate :: global_variables :: compile_time ::
                    project_git_info :: PROJECT_GIT_INFO.commit.to_string(),
                    file! ().to_string(), line! (), column! (),
                    Some(error_occurence_lib :: code_occurence :: MacroOccurence
                    {
                        file : std :: string :: String ::
                        from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                        line : 625, column : 41,
                    })),
                }) ;
                }
            };
        let name =
            match value.name {
                Some(value) => value,
                None => {
                    return
                Err(Self :: Error :: NameIsNone
                {
                    name_is_none : std :: string :: String ::
                    from("name is None"), code_occurence : error_occurence_lib
                    :: code_occurence :: CodeOccurence ::
                    new(crate :: global_variables :: compile_time ::
                    project_git_info :: PROJECT_GIT_INFO.commit.to_string(),
                    file! ().to_string(), line! (), column! (),
                    Some(error_occurence_lib :: code_occurence :: MacroOccurence
                    {
                        file : std :: string :: String ::
                        from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                        line : 684, column : 29,
                    })),
                }) ;
                }
            };
        let color =
            match value.color {
                Some(value) => value,
                None => {
                    return
                Err(Self :: Error :: ColorIsNone
                {
                    color_is_none : std :: string :: String ::
                    from("color is None"), code_occurence : error_occurence_lib
                    :: code_occurence :: CodeOccurence ::
                    new(crate :: global_variables :: compile_time ::
                    project_git_info :: PROJECT_GIT_INFO.commit.to_string(),
                    file! ().to_string(), line! (), column! (),
                    Some(error_occurence_lib :: code_occurence :: MacroOccurence
                    {
                        file : std :: string :: String ::
                        from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                        line : 684, column : 29,
                    })),
                }) ;
                }
            };
        Ok(Self { id, name, color })
    }
}
#[derive(
    Debug,
    serde :: Serialize,
    serde :: Deserialize,
    enum_extension ::
EnumExtension,
    strum_macros :: EnumIter,
    PartialEq,
    Eq,
    from_str :: FromStr,
)]
pub enum DogColumn {
    #[serde(rename(serialize = "id", deserialize = "id"))]
    Id,
    #[serde(rename(serialize = "name", deserialize = "name"))]
    Name,
    #[serde(rename(serialize = "color", deserialize = "color"))]
    Color,
}
impl std::fmt::Display for DogColumn {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", Self::to_snake_case(self))
    }
}
#[derive(
    Debug,
    serde :: Serialize,
    serde :: Deserialize,
    Clone,
    strum_macros
:: Display,
)]
pub enum DogColumnSelect {
    Id,
    Name,
    Color,
    IdName,
    IdColor,
    NameColor,
    IdNameColor,
}
impl crate::server::postgres::generate_query::GenerateQuery for DogColumnSelect {
    fn generate_query(&self) -> std::string::String {
        match self {
            Self::Id => std::string::String::from("id"),
            Self::Name => std::string::String::from("name"),
            Self::Color => std::string::String::from("color"),
            Self::IdName => std::string::String::from("id,name"),
            Self::IdColor => std::string::String::from("id,color"),
            Self::NameColor => std::string::String::from("name,color"),
            Self::IdNameColor => std::string::String::from("id,name,color"),
        }
    }
}
impl std::default::Default for DogColumnSelect {
    fn default() -> Self {
        Self::IdNameColor
    }
}
impl std::convert::From<std::option::Option<Self>> for DogColumnSelect {
    fn from(option_value: std::option::Option<Self>) -> Self {
        match option_value {
            Some(value) => value,
            None => Self::default(),
        }
    }
}
#[derive(Debug, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum DogColumnSelectFromStrErrorNamed {
    NotCorrect {
        #[eo_display_with_serialize_deserialize]
        not_correct_value: std::string::String,
        #[eo_display_with_serialize_deserialize]
        supported_values: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
impl std::str::FromStr for DogColumnSelect {
    type Err = DogColumnSelectFromStrErrorNamed;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value
        {
            "Id" => Ok(Self :: Id), "Name" => Ok(Self :: Name), "Color" =>
            Ok(Self :: Color), "IdName" => Ok(Self :: IdName), "IdColor" =>
            Ok(Self :: IdColor), "NameColor" => Ok(Self :: NameColor),
            "IdNameColor" => Ok(Self :: IdNameColor), _ =>
            Err(Self :: Err :: NotCorrect
            {
                not_correct_value : std :: string :: String :: from(value),
                supported_values : std :: string :: String ::
                from("\"Id\",\"Name\",\"Color\",\"IdName\",\"IdColor\",\"NameColor\",\"IdNameColor\""),
                code_occurence : error_occurence_lib :: code_occurence ::
                CodeOccurence ::
                new(crate :: global_variables :: compile_time ::
                project_git_info :: PROJECT_GIT_INFO.commit.to_string(), file!
                ().to_string(), line! (), column! (),
                Some(error_occurence_lib :: code_occurence :: MacroOccurence
                {
                    file : std :: string :: String ::
                    from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                    line : 977, column : 17,
                })),
            }),
        }
    }
}
impl crate::common::serde_urlencoded::SerdeUrlencodedParameter for DogColumnSelect {
    fn serde_urlencoded_parameter(self) -> std::string::String {
        self.to_string()
    }
}
impl DogColumnSelect {
    fn options_try_from_sqlx_row<'a, R: sqlx::Row>(&self, row: &'a R) -> sqlx::Result<DogOptions>
    where
        &'a std::primitive::str: sqlx::ColumnIndex<R>,
        std::option::Option<sqlx::types::Uuid>: sqlx::decode::Decode<'a, R::Database>,
        std::option::Option<sqlx::types::Uuid>: sqlx::types::Type<R::Database>,
        std::option::Option<std::string::String>:
            sqlx::decode::Decode<'a, R::Database>,
        std::option::Option<std::string::String>:
            sqlx::types::Type<R::Database>,
        std::option::Option<std::string::String>:
            sqlx::decode::Decode<'a, R::Database>,
        std::option::Option<std::string::String>:
            sqlx::types::Type<R::Database>,
    {
        let mut id: std::option::Option<postgresql_crud::SqlxTypesUuidUuidTryParseWithSerializeDeserialize> = None;
        //postgresql_crud::StdStringStringAsPostgresqlVarchar
        let mut name: std::option::Option<postgresql_crud::StdStringString> =
            None;
        let mut color: std::option::Option<postgresql_crud::StdStringString> =
            None;
        match self {
            Self::Id => {
                let primary_key_try_get_result: Result<
                    std::option::Option<sqlx::types::Uuid>,
                    sqlx::Error,
                > = row.try_get("id");
                id = match primary_key_try_get_result {
                    Ok(option_primary_key) => {
                        option_primary_key.map(|value| {
                            postgresql_crud::SqlxTypesUuidUuidTryParseWithSerializeDeserialize::from(postgresql_crud::SqlxTypesUuidUuid(value))
                        })
                    },
                    Err(e) => {
                        return Err(e);
                    }
                };
            }
            Self::Name => {
                let primary_key_try_get_result: Result<
                    std::option::Option<sqlx::types::Uuid>,
                    sqlx::Error,
                > = row.try_get("id");
                id = match primary_key_try_get_result {
                    Ok(option_primary_key) => {
                        option_primary_key.map(|value| {
                            postgresql_crud::SqlxTypesUuidUuidTryParseWithSerializeDeserialize::from(postgresql_crud::SqlxTypesUuidUuid(value))
                        })
                    },
                    Err(e) => {
                        return Err(e);
                    }
                };
                name = {
                    let value: std::option::Option<std::string::String> = row.try_get("name")?;
                    value.map(|value|postgresql_crud::StdStringString(value))
                };
            }
            Self::Color => {
                let primary_key_try_get_result: Result<
                    std::option::Option<sqlx::types::Uuid>,
                    sqlx::Error,
                > = row.try_get("id");
                id = match primary_key_try_get_result {
                    Ok(option_primary_key) => {
                        option_primary_key.map(|value| {
                            postgresql_crud::SqlxTypesUuidUuidTryParseWithSerializeDeserialize::from(postgresql_crud::SqlxTypesUuidUuid(value))
                        })
                    },
                    Err(e) => {
                        return Err(e);
                    }
                };
                color = {
                    let value: std::option::Option<std::string::String> = row.try_get("color")?;
                    value.map(|value|postgresql_crud::StdStringString(value))
                }
            }
            Self::IdName => {
                let primary_key_try_get_result: Result<
                    std::option::Option<sqlx::types::Uuid>,
                    sqlx::Error,
                > = row.try_get("id");
                id = match primary_key_try_get_result {
                    Ok(option_primary_key) => {
                        option_primary_key.map(|value| {
                            postgresql_crud::SqlxTypesUuidUuidTryParseWithSerializeDeserialize::from(postgresql_crud::SqlxTypesUuidUuid(value))
                        })
                    },
                    Err(e) => {
                        return Err(e);
                    }
                };
                name = {
                    let value: std::option::Option<std::string::String> = row.try_get("name")?;
                    value.map(|value|postgresql_crud::StdStringString(value))
                };
            }
            Self::IdColor => {
                let primary_key_try_get_result: Result<
                    std::option::Option<sqlx::types::Uuid>,
                    sqlx::Error,
                > = row.try_get("id");
                id = match primary_key_try_get_result {
                    Ok(option_primary_key) => {
                        option_primary_key.map(|value| {
                            postgresql_crud::SqlxTypesUuidUuidTryParseWithSerializeDeserialize::from(postgresql_crud::SqlxTypesUuidUuid(value))
                        })
                    },
                    Err(e) => {
                        return Err(e);
                    }
                };
                color = {
                    let value: std::option::Option<std::string::String> = row.try_get("color")?;
                    value.map(|value|postgresql_crud::StdStringString(value))
                };
            }
            Self::NameColor => {
                let primary_key_try_get_result: Result<
                    std::option::Option<sqlx::types::Uuid>,
                    sqlx::Error,
                > = row.try_get("id");
                id = match primary_key_try_get_result {
                    Ok(option_primary_key) => {
                        option_primary_key.map(|value| {
                            postgresql_crud::SqlxTypesUuidUuidTryParseWithSerializeDeserialize::from(postgresql_crud::SqlxTypesUuidUuid(value))
                        })
                    },
                    Err(e) => {
                        return Err(e);
                    }
                };
                name = {
                    let value: std::option::Option<std::string::String> = row.try_get("name")?;
                    value.map(|value|postgresql_crud::StdStringString(value))
                };
                color = {
                    let value: std::option::Option<std::string::String> = row.try_get("color")?;
                    value.map(|value|postgresql_crud::StdStringString(value))
                };
            }
            Self::IdNameColor => {
                let primary_key_try_get_result: Result<
                    std::option::Option<sqlx::types::Uuid>,
                    sqlx::Error,
                > = row.try_get("id");
                id = match primary_key_try_get_result {
                    Ok(option_primary_key) => {
                        option_primary_key.map(|value| {
                            postgresql_crud::SqlxTypesUuidUuidTryParseWithSerializeDeserialize::from(postgresql_crud::SqlxTypesUuidUuid(value))
                        })
                    },
                    Err(e) => {
                        return Err(e);
                    }
                };
                name = {
                    let value: std::option::Option<std::string::String> = row.try_get("name")?;
                    value.map(|value|postgresql_crud::StdStringString(value))
                };
                color = {
                    let value: std::option::Option<std::string::String> = row.try_get("color")?;
                    value.map(|value|postgresql_crud::StdStringString(value))
                };
            }
        }
        Ok(DogOptions { id, name, color })
    }
}
fn primary_key_uuid_wrapper_try_from_sqlx_row<'a, R: sqlx::Row>(
    row: &'a R,
) -> sqlx::Result<crate::server::postgres::uuid_wrapper::UuidWrapper>
where
    &'a std::primitive::str: sqlx::ColumnIndex<R>,
    sqlx::types::Uuid: sqlx::decode::Decode<'a, R::Database>,
    sqlx::types::Uuid: sqlx::types::Type<R::Database>,
{
    let primary_key: sqlx::types::Uuid = row.try_get("id")?;
    Ok(crate::server::postgres::uuid_wrapper::UuidWrapper::from(
        primary_key,
    ))
}
fn deserialize_dog_order_by<'de, D>(
    deserializer: D,
) -> Result<crate::server::postgres::order_by::OrderBy<DogColumn>, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    let string_deserialized = {
        use serde::Deserialize;
        std::string::String::deserialize(deserializer)?
    };
    let split_inner_url_parameters_symbol = ',';
    let default_message = format!("Invalid DogOrderBy:");
    let column_equal_str = "column=";
    let order_equal_str = "order=";
    let column = match string_deserialized.find(column_equal_str) {
        Some(index) => match index.checked_add(column_equal_str.len()) {
            Some(offset) => match string_deserialized.get(offset..) {
                Some(offset_slice) => match offset_slice.find(split_inner_url_parameters_symbol) {
                    Some(offset_slice_next_comma_index) => {
                        match offset_slice.get(0..offset_slice_next_comma_index) {
                            Some(possible_column) => match {
                                use std::str::FromStr;
                                DogColumn::from_str(possible_column)
                            } {
                                Ok(column) => column,
                                Err(e) => {
                                    return Err(serde::de::Error::custom(&format!(
                                        "{default_message} {column_equal_str} {}",
                                        e
                                    )));
                                }
                            },
                            None => {
                                return
                                Err(serde :: de :: Error ::
                                custom(& format!
                                ("{default_message} {column_equal_str} failed to offset_slice.get(0..offset_slice_next_comma_index)")))
                                ;
                            }
                        }
                    }
                    None => match offset_slice.get(0..) {
                        Some(possible_column) => match {
                            use std::str::FromStr;
                            DogColumn::from_str(possible_column)
                        } {
                            Ok(column) => column,
                            Err(e) => {
                                return Err(serde::de::Error::custom(&format!(
                                    "{default_message} {column_equal_str} {}",
                                    e
                                )));
                            }
                        },
                        None => {
                            return
                            Err(serde :: de :: Error ::
                            custom(& format!
                            ("{default_message} {column_equal_str} failed to offset_slice.get(0..)")))
                            ;
                        }
                    },
                },
                None => {
                    return
                    Err(serde :: de :: Error ::
                    custom(& format!
                    ("{default_message} {column_equal_str} failed to string_deserialized.get(offset..)")))
                    ;
                }
            },
            None => {
                return Err(serde::de::Error::custom(&format!(
                    "{default_message} {column_equal_str} index overflow"
                )));
            }
        },
        None => {
            return Err(serde::de::Error::custom(&format!(
                "{default_message} {column_equal_str} not found"
            )));
        }
    };
    let order = match string_deserialized.find(order_equal_str) {
        Some(index) => match index.checked_add(order_equal_str.len()) {
            Some(offset) => match string_deserialized.get(offset..) {
                Some(offset_slice) => match offset_slice.find(split_inner_url_parameters_symbol) {
                    Some(offset_slice_next_comma_index) => {
                        match offset_slice.get(0..offset_slice_next_comma_index) {
                            Some(possible_order) => match {
                                use std::str::FromStr;
                                crate::server::postgres::order::Order::from_str(possible_order)
                            } {
                                Ok(order) => Some(order),
                                Err(e) => {
                                    return Err(serde::de::Error::custom(&format!(
                                        "{default_message} {order_equal_str} {}",
                                        e
                                    )));
                                }
                            },
                            None => {
                                return
                                Err(serde :: de :: Error ::
                                custom(& format!
                                ("{default_message} {order_equal_str} failed to offset_slice.get(0..offset_slice_next_comma_index)")))
                                ;
                            }
                        }
                    }
                    None => match offset_slice.get(0..) {
                        Some(possible_order) => match {
                            use std::str::FromStr;
                            crate::server::postgres::order::Order::from_str(possible_order)
                        } {
                            Ok(order) => Some(order),
                            Err(e) => {
                                return Err(serde::de::Error::custom(&format!(
                                    "{default_message} {order_equal_str} {}",
                                    e
                                )));
                            }
                        },
                        None => {
                            return Err(serde::de::Error::custom(
                                &format!
                            ("{default_message} {order_equal_str} failed to offset_slice.get(0..)"),
                            ));
                        }
                    },
                },
                None => {
                    return
                    Err(serde :: de :: Error ::
                    custom(& format!
                    ("{default_message} {order_equal_str} failed to string_deserialized.get(offset..)")))
                    ;
                }
            },
            None => {
                return Err(serde::de::Error::custom(&format!(
                    "{default_message} {order_equal_str} index overflow"
                )));
            }
        },
        None => None,
    };
    Ok(crate::server::postgres::order_by::OrderBy { column, order })
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize)]
pub struct DogOrderByWrapper(
    #[serde(deserialize_with = "deserialize_dog_order_by")]
    pub  crate::server::postgres::order_by::OrderBy<DogColumn>,
);
impl crate::common::serde_urlencoded::SerdeUrlencodedParameter for DogOrderByWrapper {
    fn serde_urlencoded_parameter(self) -> std::string::String {
        let column = &self.0.column;
        let order = self.0.order.unwrap_or_default();
        format!("column={column},order={order}")
    }
}
#[derive(Debug, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum DogOrderByWrapperFromStrErrorNamed {
    ColumnFromStr {
        #[eo_display_with_serialize_deserialize]
        column_from_str: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    ColumnNoOffsetValue {
        #[eo_display_with_serialize_deserialize]
        column_no_offset_value: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    ColumnOffsetSliceGet {
        #[eo_display_with_serialize_deserialize]
        column_offset_slice_get: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    ColumnStringDeserializedGet {
        #[eo_display_with_serialize_deserialize]
        column_string_deserialized_get: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    ColumnIndexCheckedAdd {
        #[eo_display_with_serialize_deserialize]
        column_index_checked_add: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    ColumnStringDeserializedFind {
        #[eo_display_with_serialize_deserialize]
        column_string_deserialized_find: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    OrderFromStr {
        #[eo_display_with_serialize_deserialize]
        order_from_str: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    OrderOffsetSliceGetNone {
        #[eo_display_with_serialize_deserialize]
        order_offset_slice_get_none: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    OrderStringDeserializedGetNone {
        #[eo_display_with_serialize_deserialize]
        order_string_deserialized_get_none: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    OrderIndexCheckedAdd {
        #[eo_display_with_serialize_deserialize]
        order_index_checked_add: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
impl std::str::FromStr for DogOrderByWrapper {
    type Err = DogOrderByWrapperFromStrErrorNamed;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let string_deserialized = value.to_string();
        let split_inner_url_parameters_symbol = ',';
        let default_message = format!("Invalid DogOrderBy:");
        let column_equal_str = "column=";
        let order_equal_str = "order=";
        let column = match string_deserialized.find(column_equal_str) {
            Some(index) => match index.checked_add(column_equal_str.len()) {
                Some(offset) => match string_deserialized.get(offset..) {
                    Some(offset_slice) => {
                        match offset_slice.find(split_inner_url_parameters_symbol) {
                            Some(offset_slice_next_comma_index) => {
                                match offset_slice.get(0..offset_slice_next_comma_index) {
                                    Some(possible_column) => {
                                        match DogColumn::from_str(possible_column) {
                                            Ok(column) => column,
                                            Err(e) => {
                                                return
                                        Err(Self :: Err :: ColumnFromStr
                                        {
                                            column_from_str : e, code_occurence : error_occurence_lib ::
                                            code_occurence :: CodeOccurence ::
                                            new(crate :: global_variables :: compile_time ::
                                            project_git_info :: PROJECT_GIT_INFO.commit.to_string(),
                                            file! ().to_string(), line! (), column! (),
                                            Some(error_occurence_lib :: code_occurence :: MacroOccurence
                                            {
                                                file : std :: string :: String ::
                                                from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                                                line : 1403, column : 17,
                                            })),
                                        }) ;
                                            }
                                        }
                                    }
                                    None => {
                                        return
                                    Err(Self :: Err :: ColumnNoOffsetValue
                                    {
                                        column_no_offset_value : std :: string :: String ::
                                        from("no offset value"), code_occurence :
                                        error_occurence_lib :: code_occurence :: CodeOccurence ::
                                        new(crate :: global_variables :: compile_time ::
                                        project_git_info :: PROJECT_GIT_INFO.commit.to_string(),
                                        file! ().to_string(), line! (), column! (),
                                        Some(error_occurence_lib :: code_occurence :: MacroOccurence
                                        {
                                            file : std :: string :: String ::
                                            from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                                            line : 1409, column : 17,
                                        })),
                                    }) ;
                                    }
                                }
                            }
                            None => match offset_slice.get(0..) {
                                Some(possible_column) => match DogColumn::from_str(possible_column)
                                {
                                    Ok(column) => column,
                                    Err(e) => {
                                        return
                                    Err(Self :: Err :: ColumnFromStr
                                    {
                                        column_from_str : e, code_occurence : error_occurence_lib ::
                                        code_occurence :: CodeOccurence ::
                                        new(crate :: global_variables :: compile_time ::
                                        project_git_info :: PROJECT_GIT_INFO.commit.to_string(),
                                        file! ().to_string(), line! (), column! (),
                                        Some(error_occurence_lib :: code_occurence :: MacroOccurence
                                        {
                                            file : std :: string :: String ::
                                            from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                                            line : 1415, column : 17,
                                        })),
                                    }) ;
                                    }
                                },
                                None => {
                                    return
                                Err(Self :: Err :: ColumnOffsetSliceGet
                                {
                                    column_offset_slice_get : std :: string :: String ::
                                    from("offset_slice_get"), code_occurence :
                                    error_occurence_lib :: code_occurence :: CodeOccurence ::
                                    new(crate :: global_variables :: compile_time ::
                                    project_git_info :: PROJECT_GIT_INFO.commit.to_string(),
                                    file! ().to_string(), line! (), column! (),
                                    Some(error_occurence_lib :: code_occurence :: MacroOccurence
                                    {
                                        file : std :: string :: String ::
                                        from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                                        line : 1421, column : 17,
                                    })),
                                }) ;
                                }
                            },
                        }
                    }
                    None => {
                        return
                        Err(Self :: Err :: ColumnStringDeserializedGet
                        {
                            column_string_deserialized_get : std :: string :: String ::
                            from("string_deserialized_get"), code_occurence :
                            error_occurence_lib :: code_occurence :: CodeOccurence ::
                            new(crate :: global_variables :: compile_time ::
                            project_git_info :: PROJECT_GIT_INFO.commit.to_string(),
                            file! ().to_string(), line! (), column! (),
                            Some(error_occurence_lib :: code_occurence :: MacroOccurence
                            {
                                file : std :: string :: String ::
                                from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                                line : 1427, column : 17,
                            })),
                        }) ;
                    }
                },
                None => {
                    return
                    Err(Self :: Err :: ColumnIndexCheckedAdd
                    {
                        column_index_checked_add : std :: string :: String ::
                        from("index_checked_add"), code_occurence :
                        error_occurence_lib :: code_occurence :: CodeOccurence ::
                        new(crate :: global_variables :: compile_time ::
                        project_git_info :: PROJECT_GIT_INFO.commit.to_string(),
                        file! ().to_string(), line! (), column! (),
                        Some(error_occurence_lib :: code_occurence :: MacroOccurence
                        {
                            file : std :: string :: String ::
                            from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                            line : 1433, column : 17,
                        })),
                    }) ;
                }
            },
            None => {
                return Err(Self::Err::ColumnStringDeserializedFind {
                    column_string_deserialized_find: std::string::String::from(
                        "string_deserialized_find",
                    ),
                    code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                        crate::global_variables::compile_time::project_git_info::PROJECT_GIT_INFO
                            .commit
                            .to_string(),
                        file!().to_string(),
                        line!(),
                        column!(),
                        Some(error_occurence_lib::code_occurence::MacroOccurence {
                            file: std::string::String::from(
                                "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                            ),
                            line: 1439,
                            column: 17,
                        }),
                    ),
                });
            }
        };
        let order = match string_deserialized.find(order_equal_str) {
            Some(index) => match index.checked_add(order_equal_str.len()) {
                Some(offset) => match string_deserialized.get(offset..) {
                    Some(offset_slice) => {
                        match offset_slice.find(split_inner_url_parameters_symbol) {
                            Some(offset_slice_next_comma_index) => {
                                match offset_slice.get(0..offset_slice_next_comma_index) {
                                    Some(possible_order) => {
                                        match crate::server::postgres::order::Order::from_str(
                                            possible_order,
                                        ) {
                                            Ok(order) => Some(order),
                                            Err(e) => {
                                                return
                                        Err(Self :: Err :: OrderFromStr
                                        {
                                            order_from_str : e, code_occurence : error_occurence_lib ::
                                            code_occurence :: CodeOccurence ::
                                            new(crate :: global_variables :: compile_time ::
                                            project_git_info :: PROJECT_GIT_INFO.commit.to_string(),
                                            file! ().to_string(), line! (), column! (),
                                            Some(error_occurence_lib :: code_occurence :: MacroOccurence
                                            {
                                                file : std :: string :: String ::
                                                from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                                                line : 1445, column : 17,
                                            })),
                                        }) ;
                                            }
                                        }
                                    }
                                    None => {
                                        return
                                    Err(Self :: Err :: OrderOffsetSliceGetNone
                                    {
                                        order_offset_slice_get_none : std :: string :: String ::
                                        from("order_offset_slice_get_none"), code_occurence :
                                        error_occurence_lib :: code_occurence :: CodeOccurence ::
                                        new(crate :: global_variables :: compile_time ::
                                        project_git_info :: PROJECT_GIT_INFO.commit.to_string(),
                                        file! ().to_string(), line! (), column! (),
                                        Some(error_occurence_lib :: code_occurence :: MacroOccurence
                                        {
                                            file : std :: string :: String ::
                                            from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                                            line : 1451, column : 17,
                                        })),
                                    }) ;
                                    }
                                }
                            }
                            None => match offset_slice.get(0..) {
                                Some(possible_order) => {
                                    match crate::server::postgres::order::Order::from_str(
                                        possible_order,
                                    ) {
                                        Ok(order) => Some(order),
                                        Err(e) => {
                                            return
                                    Err(Self :: Err :: OrderFromStr
                                    {
                                        order_from_str : e, code_occurence : error_occurence_lib ::
                                        code_occurence :: CodeOccurence ::
                                        new(crate :: global_variables :: compile_time ::
                                        project_git_info :: PROJECT_GIT_INFO.commit.to_string(),
                                        file! ().to_string(), line! (), column! (),
                                        Some(error_occurence_lib :: code_occurence :: MacroOccurence
                                        {
                                            file : std :: string :: String ::
                                            from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                                            line : 1457, column : 17,
                                        }))
                                    }) ;
                                        }
                                    }
                                }
                                None => {
                                    return
                                Err(Self :: Err :: OrderOffsetSliceGetNone
                                {
                                    order_offset_slice_get_none : std :: string :: String ::
                                    from("order_offset_slice_get_none"), code_occurence :
                                    error_occurence_lib :: code_occurence :: CodeOccurence ::
                                    new(crate :: global_variables :: compile_time ::
                                    project_git_info :: PROJECT_GIT_INFO.commit.to_string(),
                                    file! ().to_string(), line! (), column! (),
                                    Some(error_occurence_lib :: code_occurence :: MacroOccurence
                                    {
                                        file : std :: string :: String ::
                                        from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                                        line : 1463, column : 17,
                                    })),
                                }) ;
                                }
                            },
                        }
                    }
                    None => {
                        return
                        Err(Self :: Err :: OrderStringDeserializedGetNone
                        {
                            order_string_deserialized_get_none : std :: string :: String
                            :: from("string_deserialized_get_none"), code_occurence :
                            error_occurence_lib :: code_occurence :: CodeOccurence ::
                            new(crate :: global_variables :: compile_time ::
                            project_git_info :: PROJECT_GIT_INFO.commit.to_string(),
                            file! ().to_string(), line! (), column! (),
                            Some(error_occurence_lib :: code_occurence :: MacroOccurence
                            {
                                file : std :: string :: String ::
                                from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                                line : 1469, column : 17,
                            })),
                        }) ;
                    }
                },
                None => {
                    return
                    Err(Self :: Err :: OrderIndexCheckedAdd
                    {
                        order_index_checked_add : std :: string :: String ::
                        from("order_index_checked_add"), code_occurence :
                        error_occurence_lib :: code_occurence :: CodeOccurence ::
                        new(crate :: global_variables :: compile_time ::
                        project_git_info :: PROJECT_GIT_INFO.commit.to_string(),
                        file! ().to_string(), line! (), column! (),
                        Some(error_occurence_lib :: code_occurence :: MacroOccurence
                        {
                            file : std :: string :: String ::
                            from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                            line : 1475, column : 17,
                        })),
                    }) ;
                }
            },
            None => None,
        };
        Ok(Self(crate::server::postgres::order_by::OrderBy {
            column,
            order,
        }))
    }
}
pub const ALLOW_METHODS: [http::Method; 4] = [
    http::Method::GET,
    http::Method::POST,
    http::Method::PATCH,
    http::Method::DELETE,
];
pub struct DogColumnReadPermission {
    id: bool,
    name: bool,
    color: bool,
}
#[test]
fn dog_emulate_crud_api_usage_test() {
    async fn find_out_if_it_works() {
        let api_location = std::string::String::from("http://127.0.0.1:8080");
        let limit = 1000;
        let offset = 0;
        println!("-------trycreate_many start-------");
        let primary_keys = match try_create_many(
            &api_location,
            CreateManyParameters {
                payload: CreateManyPayload(vec![CreateManyPayloadElement {
                    name: postgresql_crud::StdStringStringAsPostgresqlVarchar::default(),
                    color: postgresql_crud::StdStringStringAsPostgresqlVarchar::default(),
                }]),
            },
        )
        .await
        {
            Ok(value) => {
                println!("{value:#?}");
                value
            }
            Err(e) => panic!("{}", e),
        };
        println!("-------trycreate_many end-------");
        println!("-------tryread_many start-------");
        match try_read_many(
            &api_location,
            ReadManyParameters {
                payload: ReadManyPayload {
                    id: Some(primary_keys.clone()),
                    name: None,
                    color: None,
                    select: DogColumnSelect::IdNameColor,
                    order_by: crate::server::postgres::order_by::OrderBy {
                        column: DogColumn::Name,
                        order: Some(crate::server::postgres::order::Order::Desc),
                    },
                    limit: crate::server::postgres::postgres_bigint::PostgresBigint(limit),
                    offset: crate::server::postgres::postgres_bigint::PostgresBigint(offset),
                },
            },
        )
        .await
        {
            Ok(value) => {
                println!("{value:#?}");
                value
            }
            Err(e) => panic!("{}", e),
        };
        println!("-------tryread_many end-------");
        println!("-------tryupdate_many start-------");
        match try_update_many(
            &api_location,
            UpdateManyParameters {
                payload: UpdateManyPayload(
                    primary_keys
                        .clone()
                        .into_iter()
                        .map(|element| UpdateManyPayloadElement {
                            id: element,
                            name: postgresql_crud::StdStringStringAsPostgresqlVarchar::default(),
                            color: postgresql_crud::StdStringStringAsPostgresqlVarchar::default(),
                        })
                        .collect(),
                ),
            },
        )
        .await
        {
            Ok(value) => println!("{value:#?}"),
            Err(e) => panic!("{}", e),
        }
        println!("-------tryupdate_many end-------");
        println!("-------tryread_many start-------");
        match try_read_many(
            &api_location,
            ReadManyParameters {
                payload: ReadManyPayload {
                    id: Some(primary_keys.clone()),
                    name: None,
                    color: None,
                    select: DogColumnSelect::IdNameColor,
                    order_by: crate::server::postgres::order_by::OrderBy {
                        column: DogColumn::Name,
                        order: Some(crate::server::postgres::order::Order::Desc),
                    },
                    limit: crate::server::postgres::postgres_bigint::PostgresBigint(limit),
                    offset: crate::server::postgres::postgres_bigint::PostgresBigint(offset),
                },
            },
        )
        .await
        {
            Ok(value) => {
                println!("{value:#?}");
                value
            }
            Err(e) => panic!("{}", e),
        };
        println!("-------tryread_many end-------");
        println!("-------trydelete_many start-------");
        match try_delete_many(
            &api_location,
            DeleteManyParameters {
                payload: DeleteManyPayload {
                    id: Some(primary_keys.clone()),
                    name: None,
                    color: None,
                },
            },
        )
        .await
        {
            Ok(value) => println!("{value:#?}"),
            Err(e) => panic!("{}", e),
        }
        println!("-------trydelete_many end-------");
        println!("-------tryread_many start-------");
        match try_read_many(
            &api_location,
            ReadManyParameters {
                payload: ReadManyPayload {
                    id: Some(primary_keys.clone()),
                    name: None,
                    color: None,
                    select: DogColumnSelect::IdNameColor,
                    order_by: crate::server::postgres::order_by::OrderBy {
                        column: DogColumn::Name,
                        order: Some(crate::server::postgres::order::Order::Desc),
                    },
                    limit: crate::server::postgres::postgres_bigint::PostgresBigint(limit),
                    offset: crate::server::postgres::postgres_bigint::PostgresBigint(offset),
                },
            },
        )
        .await
        {
            Ok(value) => {
                println!("{value:#?}");
                value
            }
            Err(e) => panic!("{}", e),
        };
        println!("-------tryread_many end-------");
        println!("-------trycreate_one start-------");
        let primary_key = match try_create_one(
            &api_location,
            CreateOneParameters {
                payload: CreateOnePayload {
                    name: postgresql_crud::StdStringStringAsPostgresqlVarchar::default(),
                    color: postgresql_crud::StdStringStringAsPostgresqlVarchar::default(),
                },
            },
        )
        .await
        {
            Ok(value) => {
                println!("{value:#?}");
                value
            }
            Err(e) => panic!("{}", e),
        };
        println!("-------trycreate_one end-------");
        println!("-------tryread_one start-------");
        match try_read_one(
            &api_location,
            ReadOneParameters {
                payload: ReadOnePayload {
                    id: primary_key.clone(),
                    select: DogColumnSelect::IdNameColor,
                },
            },
        )
        .await
        {
            Ok(value) => println!("{value:#?}"),
            Err(e) => panic!("{}", e),
        };
        println!("-------tryread_one end-------");
        println!("-------tryupdate_one start-------");
        let primary_key = match try_update_one(
            &api_location,
            UpdateOneParameters {
                payload: UpdateOnePayload {
                    id: primary_key.clone(),
                    name: Some(postgresql_crud::StdStringStringAsPostgresqlVarchar::default()),
                    color: Some(postgresql_crud::StdStringStringAsPostgresqlVarchar::default()),
                },
            },
        )
        .await
        {
            Ok(value) => {
                println!("{value:#?}");
                value
            }
            Err(e) => panic!("{}", e),
        };
        println!("-------tryupdate_one end-------");
        println!("-------tryread_one start-------");
        match try_read_one(
            &api_location,
            ReadOneParameters {
                payload: ReadOnePayload {
                    id: primary_key.clone(),
                    select: DogColumnSelect::IdNameColor,
                },
            },
        )
        .await
        {
            Ok(value) => println!("{value:#?}"),
            Err(e) => panic!("{}", e),
        };
        println!("-------tryread_one end-------");
        println!("-------trydelete_one start-------");
        match try_delete_one(
            &api_location,
            DeleteOneParameters {
                payload: DeleteOnePayload {
                    id: primary_key.clone(),
                },
            },
        )
        .await
        {
            Ok(value) => println!("{value:#?}"),
            Err(e) => panic!("{}", e),
        }
        println!("-------trydelete_one end-------");
        println!("-------tryread_one start-------");
        match try_read_one(
            &api_location,
            ReadOneParameters {
                payload: ReadOnePayload {
                    id: primary_key.clone(),
                    select: DogColumnSelect::IdNameColor,
                },
            },
        )
        .await
        {
            Ok(value) => panic!("{value:#?}"),
            Err(e) => println!("{}", e),
        };
        println!("-------tryread_one end-------");
    }
    match tokio::runtime::Builder::new_multi_thread()
        .worker_threads(num_cpus::get())
        .enable_all()
        .build()
    {
        Err(e) => {
            panic!
            ("tokio::runtime::Builder::new_multi_thread().worker_threads(num_cpus::get()).enable_all().build() failed, error: {:#?}",
            e)
        }
        Ok(runtime) => {
            runtime.block_on(find_out_if_it_works());
        }
    }
}
// #[derive(Debug, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema)]
pub struct CreateManyPayloadElement {
    pub name: postgresql_crud::StdStringString,//without serialize deserialize
    pub color: postgresql_crud::StdStringString,
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema)]
pub struct CreateManyPayload(pub std::vec::Vec<CreateManyPayloadElement>);
#[derive(Debug, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema)]
pub struct CreateManyPayloadElementWithSerializeDeserialize {
    pub name: postgresql_crud::StdStringString,//with serialize deserialize
    pub color: postgresql_crud::StdStringString,//with serialize deserialize
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize)]
pub struct CreateManyPayloadWithSerializeDeserialize(
    std::vec::Vec<CreateManyPayloadElementWithSerializeDeserialize>,
);
#[derive(Debug, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum CreateManyPayloadElementTryFromCreateManyPayloadElementWithSerializeDeserializeErrorNamed {
    NotUuid {
        #[eo_error_occurence]
        not_uuid:
            crate::server::postgres::uuid_wrapper::UuidWrapperTryFromPossibleUuidWrapperErrorNamed,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
impl std::convert::TryFrom<CreateManyPayloadElementWithSerializeDeserialize>
    for CreateManyPayloadElement
{
    type Error =
        CreateManyPayloadElementTryFromCreateManyPayloadElementWithSerializeDeserializeErrorNamed;
    fn try_from(
        value: CreateManyPayloadElementWithSerializeDeserialize,
    ) -> Result<Self, Self::Error> {
        let name = value.name;
        let color = value.color;
        Ok(Self { name, color })
    }
}
#[derive(Debug, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum CreateManyPayloadTryFromCreateManyPayloadWithSerializeDeserializeErrorNamed {
    NotUuid {
        #[eo_error_occurence]
        not_uuid:
            crate::server::postgres::uuid_wrapper::UuidWrapperTryFromPossibleUuidWrapperErrorNamed,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
impl std::convert::TryFrom<CreateManyPayloadWithSerializeDeserialize> for CreateManyPayload {
    type Error = CreateManyPayloadTryFromCreateManyPayloadWithSerializeDeserializeErrorNamed;
    fn try_from(value: CreateManyPayloadWithSerializeDeserialize) -> Result<Self, Self::Error> {
        let mut elements = std::vec::Vec::with_capacity(value.0.len());
        for element in value.0 {
            match CreateManyPayloadElement::try_from(element) {
                Ok(value) => {
                    elements.push(value);
                }
                Err(e) => todo!(),
            }
        }
        Ok(Self(elements))
    }
}
impl std::convert::From<CreateManyPayloadElement>
    for CreateManyPayloadElementWithSerializeDeserialize
{
    fn from(value: CreateManyPayloadElement) -> Self {
        let name = value.name;
        let color = value.color;
        Self { name, color }
    }
}
impl std::convert::From<CreateManyPayload> for CreateManyPayloadWithSerializeDeserialize {
    fn from(value: CreateManyPayload) -> Self {
        Self(
            value
                .0
                .into_iter()
                .map(|element| CreateManyPayloadElementWithSerializeDeserialize::from(element))
                .collect::<std::vec::Vec<CreateManyPayloadElementWithSerializeDeserialize>>(),
        )
    }
}
#[derive(Debug)]
pub struct CreateManyParameters {
    pub payload: CreateManyPayload,
}
#[derive(Debug, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum OperationDoneButCannotConvertUuidWrapperFromPossibleUuidWrapperInClientErrorUnnamed {
    OperationDoneButCannotConvertUuidWrapperFromPossibleUuidWrapperInClient(
        crate::server::postgres::uuid_wrapper::UuidWrapperTryFromPossibleUuidWrapperErrorNamed,
    ),
}
#[derive(
    Debug,
    thiserror :: Error,
    error_occurence_lib :: ErrorOccurence,
    from_sqlx_postgres_error :: FromSqlxPostgresError,
)]
pub enum TryCreateMany {
    Configuration {
        #[eo_display_with_serialize_deserialize]
        configuration: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Database {
        #[eo_display_with_serialize_deserialize]
        database: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Io {
        #[eo_display]
        io: std::io::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Tls {
        #[eo_display_with_serialize_deserialize]
        tls: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Protocol {
        #[eo_display_with_serialize_deserialize]
        protocol: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    RowNotFound {
        #[eo_display_with_serialize_deserialize]
        row_not_found: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    TypeNotFound {
        #[eo_display_with_serialize_deserialize]
        type_not_found: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    ColumnIndexOutOfBounds {
        #[eo_display_with_serialize_deserialize]
        column_index_out_of_bounds: usize,
        #[eo_display_with_serialize_deserialize]
        len: usize,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    ColumnNotFound {
        #[eo_display_with_serialize_deserialize]
        column_not_found: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    ColumnDecode {
        #[eo_display_with_serialize_deserialize]
        column_decode_index: std::string::String,
        #[eo_display_with_serialize_deserialize]
        source_handle: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Decode {
        #[eo_display_with_serialize_deserialize]
        decode: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    PoolTimedOut {
        #[eo_display_with_serialize_deserialize]
        pool_timed_out: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    PoolClosed {
        #[eo_display_with_serialize_deserialize]
        pool_closed: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    WorkerCrashed {
        #[eo_display_with_serialize_deserialize]
        worker_crashed: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Migrate {
        #[eo_display]
        migrate: sqlx::migrate::MigrateError,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    JsonDataError {
        #[eo_display]
        json_data_error: axum::extract::rejection::JsonDataError,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    JsonSyntaxError {
        #[eo_display]
        json_syntax_error: axum::extract::rejection::JsonSyntaxError,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    MissingJsonContentType {
        #[eo_display_with_serialize_deserialize]
        missing_json_content_type: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    BytesRejection {
        #[eo_display_with_serialize_deserialize]
        bytes_rejection: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    UnexpectedCase {
        #[eo_display_with_serialize_deserialize]
        unexpected_case: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    BindQuery {
        #[eo_error_occurence]
        bind_query: crate::server::postgres::bind_query::TryGenerateBindIncrementsErrorNamed,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    OperationDoneButCannotConvertUuidWrapperFromPossibleUuidWrapperInServer {
        #[eo_display]
        operation_done_but_cannot_convert_uuid_wrapper_from_possible_uuid_wrapper_in_server:
            sqlx::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    CreateManyPayloadTryFromCreateManyPayloadWithSerializeDeserialize {
        #[eo_error_occurence]
        create_many_payload_try_from_create_many_payload_with_serialize_deserialize:
            CreateManyPayloadTryFromCreateManyPayloadWithSerializeDeserializeErrorNamed,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    CommitExtractorNotEqual {
        #[eo_display_with_serialize_deserialize]
        commit_not_equal: std::string::String,
        #[eo_display_with_serialize_deserialize]
        commit_to_use: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    CommitExtractorToStrConversion {
        #[eo_display]
        commit_to_str_conversion: http::header::ToStrError,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NoCommitExtractorHeader {
        #[eo_display_with_serialize_deserialize]
        no_commit_header: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize)]
pub enum TryCreateManyResponseVariants {
    Desirable(std :: vec :: Vec :: < crate :: server :: postgres ::
    uuid_wrapper :: PossibleUuidWrapper >), Configuration
    {
        configuration : std::string::String<>, code_occurence :
        error_occurence_lib::code_occurence::CodeOccurence
    }, Database
    {
        database : std::string::String<>, code_occurence :
        error_occurence_lib::code_occurence::CodeOccurence
    }, Io
    {
        io : std :: string :: String, code_occurence :
        error_occurence_lib::code_occurence::CodeOccurence
    }, Tls
    {
        tls : std::string::String<>, code_occurence :
        error_occurence_lib::code_occurence::CodeOccurence
    }, Protocol
    {
        protocol : std::string::String<>, code_occurence :
        error_occurence_lib::code_occurence::CodeOccurence
    }, RowNotFound
    {
        row_not_found : std::string::String<>, code_occurence :
        error_occurence_lib::code_occurence::CodeOccurence
    }, TypeNotFound
    {
        type_not_found : std::string::String<>, code_occurence :
        error_occurence_lib::code_occurence::CodeOccurence
    }, ColumnIndexOutOfBounds
    {
        column_index_out_of_bounds : usize<>, len : usize<>, code_occurence :
        error_occurence_lib::code_occurence::CodeOccurence
    }, ColumnNotFound
    {
        column_not_found : std::string::String<>, code_occurence :
        error_occurence_lib::code_occurence::CodeOccurence
    }, ColumnDecode
    {
        column_decode_index : std::string::String<>, source_handle :
        std::string::String<>, code_occurence :
        error_occurence_lib::code_occurence::CodeOccurence
    }, Decode
    {
        decode : std::string::String<>, code_occurence :
        error_occurence_lib::code_occurence::CodeOccurence
    }, PoolTimedOut
    {
        pool_timed_out : std::string::String<>, code_occurence :
        error_occurence_lib::code_occurence::CodeOccurence
    }, PoolClosed
    {
        pool_closed : std::string::String<>, code_occurence :
        error_occurence_lib::code_occurence::CodeOccurence
    }, WorkerCrashed
    {
        worker_crashed : std::string::String<>, code_occurence :
        error_occurence_lib::code_occurence::CodeOccurence
    }, Migrate
    {
        migrate : std :: string :: String, code_occurence :
        error_occurence_lib::code_occurence::CodeOccurence
    }, JsonDataError
    {
        json_data_error : std :: string :: String, code_occurence :
        error_occurence_lib::code_occurence::CodeOccurence
    }, JsonSyntaxError
    {
        json_syntax_error : std :: string :: String, code_occurence :
        error_occurence_lib::code_occurence::CodeOccurence
    }, MissingJsonContentType
    {
        missing_json_content_type : std::string::String<>, code_occurence :
        error_occurence_lib::code_occurence::CodeOccurence
    }, BytesRejection
    {
        bytes_rejection : std::string::String<>, code_occurence :
        error_occurence_lib::code_occurence::CodeOccurence
    }, UnexpectedCase
    {
        unexpected_case : std::string::String<>, code_occurence :
        error_occurence_lib::code_occurence::CodeOccurence
    }, BindQuery
    {
        bind_query :
        crate::server::postgres::bind_query::TryGenerateBindIncrementsErrorNamedWithSerializeDeserialize,
        code_occurence : error_occurence_lib::code_occurence::CodeOccurence
    }, OperationDoneButCannotConvertUuidWrapperFromPossibleUuidWrapperInServer
    {
        operation_done_but_cannot_convert_uuid_wrapper_from_possible_uuid_wrapper_in_server
        : std :: string :: String, code_occurence :
        error_occurence_lib::code_occurence::CodeOccurence
    }, CreateManyPayloadTryFromCreateManyPayloadWithSerializeDeserialize
    {
        create_many_payload_try_from_create_many_payload_with_serialize_deserialize
        :
        CreateManyPayloadTryFromCreateManyPayloadWithSerializeDeserializeErrorNamedWithSerializeDeserialize,
        code_occurence : error_occurence_lib::code_occurence::CodeOccurence
    }, CommitExtractorNotEqual
    {
        commit_not_equal : std::string::String<>, commit_to_use :
        std::string::String<>, code_occurence :
        error_occurence_lib::code_occurence::CodeOccurence
    }, CommitExtractorToStrConversion
    {
        commit_to_str_conversion : std :: string :: String, code_occurence :
        error_occurence_lib::code_occurence::CodeOccurence
    }, NoCommitExtractorHeader
    {
        no_commit_header : std::string::String<>, code_occurence :
        error_occurence_lib::code_occurence::CodeOccurence
    }
}
impl std::convert::From<TryCreateMany> for TryCreateManyResponseVariants {
    fn from(value: TryCreateMany) -> Self {
        match value.into_serialize_deserialize_version()
        {
            TryCreateManyWithSerializeDeserialize :: Configuration
            { configuration, code_occurence } => Self :: Configuration
            { configuration, code_occurence },
            TryCreateManyWithSerializeDeserialize :: Database
            { database, code_occurence } => Self :: Database
            { database, code_occurence },
            TryCreateManyWithSerializeDeserialize :: Io { io, code_occurence }
            => Self :: Io { io, code_occurence },
            TryCreateManyWithSerializeDeserialize :: Tls
            { tls, code_occurence } => Self :: Tls { tls, code_occurence },
            TryCreateManyWithSerializeDeserialize :: Protocol
            { protocol, code_occurence } => Self :: Protocol
            { protocol, code_occurence },
            TryCreateManyWithSerializeDeserialize :: RowNotFound
            { row_not_found, code_occurence } => Self :: RowNotFound
            { row_not_found, code_occurence },
            TryCreateManyWithSerializeDeserialize :: TypeNotFound
            { type_not_found, code_occurence } => Self :: TypeNotFound
            { type_not_found, code_occurence },
            TryCreateManyWithSerializeDeserialize :: ColumnIndexOutOfBounds
            { column_index_out_of_bounds, len, code_occurence } => Self ::
            ColumnIndexOutOfBounds
            { column_index_out_of_bounds, len, code_occurence },
            TryCreateManyWithSerializeDeserialize :: ColumnNotFound
            { column_not_found, code_occurence } => Self :: ColumnNotFound
            { column_not_found, code_occurence },
            TryCreateManyWithSerializeDeserialize :: ColumnDecode
            { column_decode_index, source_handle, code_occurence } => Self ::
            ColumnDecode
            { column_decode_index, source_handle, code_occurence },
            TryCreateManyWithSerializeDeserialize :: Decode
            { decode, code_occurence } => Self :: Decode
            { decode, code_occurence }, TryCreateManyWithSerializeDeserialize
            :: PoolTimedOut { pool_timed_out, code_occurence } => Self ::
            PoolTimedOut { pool_timed_out, code_occurence },
            TryCreateManyWithSerializeDeserialize :: PoolClosed
            { pool_closed, code_occurence } => Self :: PoolClosed
            { pool_closed, code_occurence },
            TryCreateManyWithSerializeDeserialize :: WorkerCrashed
            { worker_crashed, code_occurence } => Self :: WorkerCrashed
            { worker_crashed, code_occurence },
            TryCreateManyWithSerializeDeserialize :: Migrate
            { migrate, code_occurence } => Self :: Migrate
            { migrate, code_occurence }, TryCreateManyWithSerializeDeserialize
            :: JsonDataError { json_data_error, code_occurence } => Self ::
            JsonDataError { json_data_error, code_occurence },
            TryCreateManyWithSerializeDeserialize :: JsonSyntaxError
            { json_syntax_error, code_occurence } => Self :: JsonSyntaxError
            { json_syntax_error, code_occurence },
            TryCreateManyWithSerializeDeserialize :: MissingJsonContentType
            { missing_json_content_type, code_occurence } => Self ::
            MissingJsonContentType
            { missing_json_content_type, code_occurence },
            TryCreateManyWithSerializeDeserialize :: BytesRejection
            { bytes_rejection, code_occurence } => Self :: BytesRejection
            { bytes_rejection, code_occurence },
            TryCreateManyWithSerializeDeserialize :: UnexpectedCase
            { unexpected_case, code_occurence } => Self :: UnexpectedCase
            { unexpected_case, code_occurence },
            TryCreateManyWithSerializeDeserialize :: BindQuery
            { bind_query, code_occurence } => Self :: BindQuery
            { bind_query, code_occurence },
            TryCreateManyWithSerializeDeserialize ::
            OperationDoneButCannotConvertUuidWrapperFromPossibleUuidWrapperInServer
            {
                operation_done_but_cannot_convert_uuid_wrapper_from_possible_uuid_wrapper_in_server,
                code_occurence
            } => Self ::
            OperationDoneButCannotConvertUuidWrapperFromPossibleUuidWrapperInServer
            {
                operation_done_but_cannot_convert_uuid_wrapper_from_possible_uuid_wrapper_in_server,
                code_occurence
            }, TryCreateManyWithSerializeDeserialize ::
            CreateManyPayloadTryFromCreateManyPayloadWithSerializeDeserialize
            {
                create_many_payload_try_from_create_many_payload_with_serialize_deserialize,
                code_occurence
            } => Self ::
            CreateManyPayloadTryFromCreateManyPayloadWithSerializeDeserialize
            {
                create_many_payload_try_from_create_many_payload_with_serialize_deserialize,
                code_occurence
            }, TryCreateManyWithSerializeDeserialize ::
            CommitExtractorNotEqual
            { commit_not_equal, commit_to_use, code_occurence } => Self ::
            CommitExtractorNotEqual
            { commit_not_equal, commit_to_use, code_occurence },
            TryCreateManyWithSerializeDeserialize ::
            CommitExtractorToStrConversion
            { commit_to_str_conversion, code_occurence } => Self ::
            CommitExtractorToStrConversion
            { commit_to_str_conversion, code_occurence },
            TryCreateManyWithSerializeDeserialize :: NoCommitExtractorHeader
            { no_commit_header, code_occurence } => Self ::
            NoCommitExtractorHeader { no_commit_header, code_occurence }
        }
    }
}
impl std::convert::From<&TryCreateManyResponseVariants> for axum::http::StatusCode {
    fn from(value: &TryCreateManyResponseVariants) -> Self {
        match value
        {
            TryCreateManyResponseVariants :: Desirable(_) => axum :: http ::
            StatusCode :: CREATED, TryCreateManyResponseVariants ::
            Configuration { configuration : _, code_occurence : _ } => axum ::
            http :: StatusCode :: CREATED, TryCreateManyResponseVariants ::
            Database { database : _, code_occurence : _ } => axum :: http ::
            StatusCode :: CREATED, TryCreateManyResponseVariants :: Io
            { io : _, code_occurence : _ } => axum :: http :: StatusCode ::
            CREATED, TryCreateManyResponseVariants :: Tls
            { tls : _, code_occurence : _ } => axum :: http :: StatusCode ::
            CREATED, TryCreateManyResponseVariants :: Protocol
            { protocol : _, code_occurence : _ } => axum :: http :: StatusCode
            :: CREATED, TryCreateManyResponseVariants :: RowNotFound
            { row_not_found : _, code_occurence : _ } => axum :: http ::
            StatusCode :: CREATED, TryCreateManyResponseVariants ::
            TypeNotFound { type_not_found : _, code_occurence : _ } => axum ::
            http :: StatusCode :: CREATED, TryCreateManyResponseVariants ::
            ColumnIndexOutOfBounds
            { column_index_out_of_bounds : _, len : _, code_occurence : _ } =>
            axum :: http :: StatusCode :: CREATED,
            TryCreateManyResponseVariants :: ColumnNotFound
            { column_not_found : _, code_occurence : _ } => axum :: http ::
            StatusCode :: CREATED, TryCreateManyResponseVariants ::
            ColumnDecode
            { column_decode_index : _, source_handle : _, code_occurence : _ }
            => axum :: http :: StatusCode :: CREATED,
            TryCreateManyResponseVariants :: Decode
            { decode : _, code_occurence : _ } => axum :: http :: StatusCode
            :: CREATED, TryCreateManyResponseVariants :: PoolTimedOut
            { pool_timed_out : _, code_occurence : _ } => axum :: http ::
            StatusCode :: CREATED, TryCreateManyResponseVariants :: PoolClosed
            { pool_closed : _, code_occurence : _ } => axum :: http ::
            StatusCode :: CREATED, TryCreateManyResponseVariants ::
            WorkerCrashed { worker_crashed : _, code_occurence : _ } => axum
            :: http :: StatusCode :: CREATED, TryCreateManyResponseVariants ::
            Migrate { migrate : _, code_occurence : _ } => axum :: http ::
            StatusCode :: CREATED, TryCreateManyResponseVariants ::
            JsonDataError { json_data_error : _, code_occurence : _ } => axum
            :: http :: StatusCode :: CREATED, TryCreateManyResponseVariants ::
            JsonSyntaxError { json_syntax_error : _, code_occurence : _ } =>
            axum :: http :: StatusCode :: CREATED,
            TryCreateManyResponseVariants :: MissingJsonContentType
            { missing_json_content_type : _, code_occurence : _ } => axum ::
            http :: StatusCode :: CREATED, TryCreateManyResponseVariants ::
            BytesRejection { bytes_rejection : _, code_occurence : _ } => axum
            :: http :: StatusCode :: CREATED, TryCreateManyResponseVariants ::
            UnexpectedCase { unexpected_case : _, code_occurence : _ } => axum
            :: http :: StatusCode :: CREATED, TryCreateManyResponseVariants ::
            BindQuery { bind_query : _, code_occurence : _ } => axum :: http
            :: StatusCode :: CREATED, TryCreateManyResponseVariants ::
            OperationDoneButCannotConvertUuidWrapperFromPossibleUuidWrapperInServer
            {
                operation_done_but_cannot_convert_uuid_wrapper_from_possible_uuid_wrapper_in_server
                : _, code_occurence : _
            } => axum :: http :: StatusCode :: CREATED,
            TryCreateManyResponseVariants ::
            CreateManyPayloadTryFromCreateManyPayloadWithSerializeDeserialize
            {
                create_many_payload_try_from_create_many_payload_with_serialize_deserialize
                : _, code_occurence : _
            } => axum :: http :: StatusCode :: CREATED,
            TryCreateManyResponseVariants :: CommitExtractorNotEqual
            { commit_not_equal : _, commit_to_use : _, code_occurence : _ } =>
            axum :: http :: StatusCode :: CREATED,
            TryCreateManyResponseVariants :: CommitExtractorToStrConversion
            { commit_to_str_conversion : _, code_occurence : _ } => axum ::
            http :: StatusCode :: CREATED, TryCreateManyResponseVariants ::
            NoCommitExtractorHeader
            { no_commit_header : _, code_occurence : _ } => axum :: http ::
            StatusCode :: CREATED
        }
    }
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema)]
pub enum TryCreateManyResponseVariantsTvfrr201Created {
    Desirable(std::vec::Vec<crate::server::postgres::uuid_wrapper::PossibleUuidWrapper>),
}
impl std::convert::From<TryCreateManyResponseVariantsTvfrr201Created>
    for TryCreateManyResponseVariants
{
    fn from(value: TryCreateManyResponseVariantsTvfrr201Created) -> Self {
        match value {
            TryCreateManyResponseVariantsTvfrr201Created::Desirable(i) => Self::Desirable(i),
        }
    }
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema)]
pub enum TryCreateManyResponseVariantsTvfrr404NotFound {
    RowNotFound {
        row_not_found: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
impl std::convert::From<TryCreateManyResponseVariantsTvfrr404NotFound>
    for TryCreateManyResponseVariants
{
    fn from(value: TryCreateManyResponseVariantsTvfrr404NotFound) -> Self {
        match value {
            TryCreateManyResponseVariantsTvfrr404NotFound::RowNotFound {
                row_not_found,
                code_occurence,
            } => Self::RowNotFound {
                row_not_found,
                code_occurence,
            },
        }
    }
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema)]
pub enum TryCreateManyResponseVariantsTvfrr400BadRequest {
    TypeNotFound
    {
        type_not_found : std::string::String<>, code_occurence :
        error_occurence_lib::code_occurence::CodeOccurence
    }, ColumnNotFound
    {
        column_not_found : std::string::String<>, code_occurence :
        error_occurence_lib::code_occurence::CodeOccurence
    }, JsonDataError
    {
        json_data_error : std :: string :: String, code_occurence :
        error_occurence_lib::code_occurence::CodeOccurence
    }, JsonSyntaxError
    {
        json_syntax_error : std :: string :: String, code_occurence :
        error_occurence_lib::code_occurence::CodeOccurence
    }, MissingJsonContentType
    {
        missing_json_content_type : std::string::String<>, code_occurence :
        error_occurence_lib::code_occurence::CodeOccurence
    }, CreateManyPayloadTryFromCreateManyPayloadWithSerializeDeserialize
    {
        create_many_payload_try_from_create_many_payload_with_serialize_deserialize
        :
        CreateManyPayloadTryFromCreateManyPayloadWithSerializeDeserializeErrorNamedWithSerializeDeserialize,
        code_occurence : error_occurence_lib::code_occurence::CodeOccurence
    }, CommitExtractorNotEqual
    {
        commit_not_equal : std::string::String<>, commit_to_use :
        std::string::String<>, code_occurence :
        error_occurence_lib::code_occurence::CodeOccurence
    }, CommitExtractorToStrConversion
    {
        commit_to_str_conversion : std :: string :: String, code_occurence :
        error_occurence_lib::code_occurence::CodeOccurence
    }, NoCommitExtractorHeader
    {
        no_commit_header : std::string::String<>, code_occurence :
        error_occurence_lib::code_occurence::CodeOccurence
    }
}
impl std::convert::From<TryCreateManyResponseVariantsTvfrr400BadRequest>
    for TryCreateManyResponseVariants
{
    fn from(value: TryCreateManyResponseVariantsTvfrr400BadRequest) -> Self {
        match value
        {
            TryCreateManyResponseVariantsTvfrr400BadRequest :: TypeNotFound
            { type_not_found, code_occurence } => Self :: TypeNotFound
            { type_not_found, code_occurence },
            TryCreateManyResponseVariantsTvfrr400BadRequest :: ColumnNotFound
            { column_not_found, code_occurence } => Self :: ColumnNotFound
            { column_not_found, code_occurence },
            TryCreateManyResponseVariantsTvfrr400BadRequest :: JsonDataError
            { json_data_error, code_occurence } => Self :: JsonDataError
            { json_data_error, code_occurence },
            TryCreateManyResponseVariantsTvfrr400BadRequest :: JsonSyntaxError
            { json_syntax_error, code_occurence } => Self :: JsonSyntaxError
            { json_syntax_error, code_occurence },
            TryCreateManyResponseVariantsTvfrr400BadRequest ::
            MissingJsonContentType
            { missing_json_content_type, code_occurence } => Self ::
            MissingJsonContentType
            { missing_json_content_type, code_occurence },
            TryCreateManyResponseVariantsTvfrr400BadRequest ::
            CreateManyPayloadTryFromCreateManyPayloadWithSerializeDeserialize
            {
                create_many_payload_try_from_create_many_payload_with_serialize_deserialize,
                code_occurence
            } => Self ::
            CreateManyPayloadTryFromCreateManyPayloadWithSerializeDeserialize
            {
                create_many_payload_try_from_create_many_payload_with_serialize_deserialize,
                code_occurence
            }, TryCreateManyResponseVariantsTvfrr400BadRequest ::
            CommitExtractorNotEqual
            { commit_not_equal, commit_to_use, code_occurence } => Self ::
            CommitExtractorNotEqual
            { commit_not_equal, commit_to_use, code_occurence },
            TryCreateManyResponseVariantsTvfrr400BadRequest ::
            CommitExtractorToStrConversion
            { commit_to_str_conversion, code_occurence } => Self ::
            CommitExtractorToStrConversion
            { commit_to_str_conversion, code_occurence },
            TryCreateManyResponseVariantsTvfrr400BadRequest ::
            NoCommitExtractorHeader { no_commit_header, code_occurence } =>
            Self :: NoCommitExtractorHeader
            { no_commit_header, code_occurence }
        }
    }
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema)]
pub enum TryCreateManyResponseVariantsTvfrr408RequestTimeout {
    PoolTimedOut {
        pool_timed_out: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
impl std::convert::From<TryCreateManyResponseVariantsTvfrr408RequestTimeout>
    for TryCreateManyResponseVariants
{
    fn from(value: TryCreateManyResponseVariantsTvfrr408RequestTimeout) -> Self {
        match value {
            TryCreateManyResponseVariantsTvfrr408RequestTimeout::PoolTimedOut {
                pool_timed_out,
                code_occurence,
            } => Self::PoolTimedOut {
                pool_timed_out,
                code_occurence,
            },
        }
    }
}
#[derive(Debug, serde :: Serialize, serde :: Deserialize, utoipa :: ToSchema)]
pub enum TryCreateManyResponseVariantsTvfrr500InternalServerError {
    Configuration
    {
        configuration : std::string::String<>, code_occurence :
        error_occurence_lib::code_occurence::CodeOccurence
    }, Database
    {
        database : std::string::String<>, code_occurence :
        error_occurence_lib::code_occurence::CodeOccurence
    }, Io
    {
        io : std :: string :: String, code_occurence :
        error_occurence_lib::code_occurence::CodeOccurence
    }, Tls
    {
        tls : std::string::String<>, code_occurence :
        error_occurence_lib::code_occurence::CodeOccurence
    }, Protocol
    {
        protocol : std::string::String<>, code_occurence :
        error_occurence_lib::code_occurence::CodeOccurence
    }, ColumnIndexOutOfBounds
    {
        column_index_out_of_bounds : usize<>, len : usize<>, code_occurence :
        error_occurence_lib::code_occurence::CodeOccurence
    }, ColumnDecode
    {
        column_decode_index : std::string::String<>, source_handle :
        std::string::String<>, code_occurence :
        error_occurence_lib::code_occurence::CodeOccurence
    }, Decode
    {
        decode : std::string::String<>, code_occurence :
        error_occurence_lib::code_occurence::CodeOccurence
    }, PoolClosed
    {
        pool_closed : std::string::String<>, code_occurence :
        error_occurence_lib::code_occurence::CodeOccurence
    }, WorkerCrashed
    {
        worker_crashed : std::string::String<>, code_occurence :
        error_occurence_lib::code_occurence::CodeOccurence
    }, Migrate
    {
        migrate : std :: string :: String, code_occurence :
        error_occurence_lib::code_occurence::CodeOccurence
    }, BytesRejection
    {
        bytes_rejection : std::string::String<>, code_occurence :
        error_occurence_lib::code_occurence::CodeOccurence
    }, UnexpectedCase
    {
        unexpected_case : std::string::String<>, code_occurence :
        error_occurence_lib::code_occurence::CodeOccurence
    }, BindQuery
    {
        bind_query :
        crate::server::postgres::bind_query::TryGenerateBindIncrementsErrorNamedWithSerializeDeserialize,
        code_occurence : error_occurence_lib::code_occurence::CodeOccurence
    }, OperationDoneButCannotConvertUuidWrapperFromPossibleUuidWrapperInServer
    {
        operation_done_but_cannot_convert_uuid_wrapper_from_possible_uuid_wrapper_in_server
        : std :: string :: String, code_occurence :
        error_occurence_lib::code_occurence::CodeOccurence
    }
}
impl std::convert::From<TryCreateManyResponseVariantsTvfrr500InternalServerError>
    for TryCreateManyResponseVariants
{
    fn from(value: TryCreateManyResponseVariantsTvfrr500InternalServerError) -> Self {
        match value
        {
            TryCreateManyResponseVariantsTvfrr500InternalServerError ::
            Configuration { configuration, code_occurence } => Self ::
            Configuration { configuration, code_occurence },
            TryCreateManyResponseVariantsTvfrr500InternalServerError ::
            Database { database, code_occurence } => Self :: Database
            { database, code_occurence },
            TryCreateManyResponseVariantsTvfrr500InternalServerError :: Io
            { io, code_occurence } => Self :: Io { io, code_occurence },
            TryCreateManyResponseVariantsTvfrr500InternalServerError :: Tls
            { tls, code_occurence } => Self :: Tls { tls, code_occurence },
            TryCreateManyResponseVariantsTvfrr500InternalServerError ::
            Protocol { protocol, code_occurence } => Self :: Protocol
            { protocol, code_occurence },
            TryCreateManyResponseVariantsTvfrr500InternalServerError ::
            ColumnIndexOutOfBounds
            { column_index_out_of_bounds, len, code_occurence } => Self ::
            ColumnIndexOutOfBounds
            { column_index_out_of_bounds, len, code_occurence },
            TryCreateManyResponseVariantsTvfrr500InternalServerError ::
            ColumnDecode
            { column_decode_index, source_handle, code_occurence } => Self ::
            ColumnDecode
            { column_decode_index, source_handle, code_occurence },
            TryCreateManyResponseVariantsTvfrr500InternalServerError :: Decode
            { decode, code_occurence } => Self :: Decode
            { decode, code_occurence },
            TryCreateManyResponseVariantsTvfrr500InternalServerError ::
            PoolClosed { pool_closed, code_occurence } => Self :: PoolClosed
            { pool_closed, code_occurence },
            TryCreateManyResponseVariantsTvfrr500InternalServerError ::
            WorkerCrashed { worker_crashed, code_occurence } => Self ::
            WorkerCrashed { worker_crashed, code_occurence },
            TryCreateManyResponseVariantsTvfrr500InternalServerError ::
            Migrate { migrate, code_occurence } => Self :: Migrate
            { migrate, code_occurence },
            TryCreateManyResponseVariantsTvfrr500InternalServerError ::
            BytesRejection { bytes_rejection, code_occurence } => Self ::
            BytesRejection { bytes_rejection, code_occurence },
            TryCreateManyResponseVariantsTvfrr500InternalServerError ::
            UnexpectedCase { unexpected_case, code_occurence } => Self ::
            UnexpectedCase { unexpected_case, code_occurence },
            TryCreateManyResponseVariantsTvfrr500InternalServerError ::
            BindQuery { bind_query, code_occurence } => Self :: BindQuery
            { bind_query, code_occurence },
            TryCreateManyResponseVariantsTvfrr500InternalServerError ::
            OperationDoneButCannotConvertUuidWrapperFromPossibleUuidWrapperInServer
            {
                operation_done_but_cannot_convert_uuid_wrapper_from_possible_uuid_wrapper_in_server,
                code_occurence
            } => Self ::
            OperationDoneButCannotConvertUuidWrapperFromPossibleUuidWrapperInServer
            {
                operation_done_but_cannot_convert_uuid_wrapper_from_possible_uuid_wrapper_in_server,
                code_occurence
            }
        }
    }
}
impl TryFrom<TryCreateManyResponseVariants>
    for std::vec::Vec<crate::server::postgres::uuid_wrapper::PossibleUuidWrapper>
{
    type Error = TryCreateManyWithSerializeDeserialize;
    fn try_from(value: TryCreateManyResponseVariants) -> Result<Self, Self::Error> {
        match value
        {
            TryCreateManyResponseVariants :: Desirable(i) => Ok(i),
            TryCreateManyResponseVariants :: Configuration
            { configuration, code_occurence } =>
            Err(TryCreateManyWithSerializeDeserialize :: Configuration
            { configuration, code_occurence }), TryCreateManyResponseVariants
            :: Database { database, code_occurence } =>
            Err(TryCreateManyWithSerializeDeserialize :: Database
            { database, code_occurence }), TryCreateManyResponseVariants :: Io
            { io, code_occurence } =>
            Err(TryCreateManyWithSerializeDeserialize :: Io
            { io, code_occurence }), TryCreateManyResponseVariants :: Tls
            { tls, code_occurence } =>
            Err(TryCreateManyWithSerializeDeserialize :: Tls
            { tls, code_occurence }), TryCreateManyResponseVariants ::
            Protocol { protocol, code_occurence } =>
            Err(TryCreateManyWithSerializeDeserialize :: Protocol
            { protocol, code_occurence }), TryCreateManyResponseVariants ::
            RowNotFound { row_not_found, code_occurence } =>
            Err(TryCreateManyWithSerializeDeserialize :: RowNotFound
            { row_not_found, code_occurence }), TryCreateManyResponseVariants
            :: TypeNotFound { type_not_found, code_occurence } =>
            Err(TryCreateManyWithSerializeDeserialize :: TypeNotFound
            { type_not_found, code_occurence }), TryCreateManyResponseVariants
            :: ColumnIndexOutOfBounds
            { column_index_out_of_bounds, len, code_occurence } =>
            Err(TryCreateManyWithSerializeDeserialize ::
            ColumnIndexOutOfBounds
            { column_index_out_of_bounds, len, code_occurence }),
            TryCreateManyResponseVariants :: ColumnNotFound
            { column_not_found, code_occurence } =>
            Err(TryCreateManyWithSerializeDeserialize :: ColumnNotFound
            { column_not_found, code_occurence }),
            TryCreateManyResponseVariants :: ColumnDecode
            { column_decode_index, source_handle, code_occurence } =>
            Err(TryCreateManyWithSerializeDeserialize :: ColumnDecode
            { column_decode_index, source_handle, code_occurence }),
            TryCreateManyResponseVariants :: Decode { decode, code_occurence }
            =>
            Err(TryCreateManyWithSerializeDeserialize :: Decode
            { decode, code_occurence }), TryCreateManyResponseVariants ::
            PoolTimedOut { pool_timed_out, code_occurence } =>
            Err(TryCreateManyWithSerializeDeserialize :: PoolTimedOut
            { pool_timed_out, code_occurence }), TryCreateManyResponseVariants
            :: PoolClosed { pool_closed, code_occurence } =>
            Err(TryCreateManyWithSerializeDeserialize :: PoolClosed
            { pool_closed, code_occurence }), TryCreateManyResponseVariants ::
            WorkerCrashed { worker_crashed, code_occurence } =>
            Err(TryCreateManyWithSerializeDeserialize :: WorkerCrashed
            { worker_crashed, code_occurence }), TryCreateManyResponseVariants
            :: Migrate { migrate, code_occurence } =>
            Err(TryCreateManyWithSerializeDeserialize :: Migrate
            { migrate, code_occurence }), TryCreateManyResponseVariants ::
            JsonDataError { json_data_error, code_occurence } =>
            Err(TryCreateManyWithSerializeDeserialize :: JsonDataError
            { json_data_error, code_occurence }),
            TryCreateManyResponseVariants :: JsonSyntaxError
            { json_syntax_error, code_occurence } =>
            Err(TryCreateManyWithSerializeDeserialize :: JsonSyntaxError
            { json_syntax_error, code_occurence }),
            TryCreateManyResponseVariants :: MissingJsonContentType
            { missing_json_content_type, code_occurence } =>
            Err(TryCreateManyWithSerializeDeserialize ::
            MissingJsonContentType
            { missing_json_content_type, code_occurence }),
            TryCreateManyResponseVariants :: BytesRejection
            { bytes_rejection, code_occurence } =>
            Err(TryCreateManyWithSerializeDeserialize :: BytesRejection
            { bytes_rejection, code_occurence }),
            TryCreateManyResponseVariants :: UnexpectedCase
            { unexpected_case, code_occurence } =>
            Err(TryCreateManyWithSerializeDeserialize :: UnexpectedCase
            { unexpected_case, code_occurence }),
            TryCreateManyResponseVariants :: BindQuery
            { bind_query, code_occurence } =>
            Err(TryCreateManyWithSerializeDeserialize :: BindQuery
            { bind_query, code_occurence }), TryCreateManyResponseVariants ::
            OperationDoneButCannotConvertUuidWrapperFromPossibleUuidWrapperInServer
            {
                operation_done_but_cannot_convert_uuid_wrapper_from_possible_uuid_wrapper_in_server,
                code_occurence
            } =>
            Err(TryCreateManyWithSerializeDeserialize ::
            OperationDoneButCannotConvertUuidWrapperFromPossibleUuidWrapperInServer
            {
                operation_done_but_cannot_convert_uuid_wrapper_from_possible_uuid_wrapper_in_server,
                code_occurence
            }), TryCreateManyResponseVariants ::
            CreateManyPayloadTryFromCreateManyPayloadWithSerializeDeserialize
            {
                create_many_payload_try_from_create_many_payload_with_serialize_deserialize,
                code_occurence
            } =>
            Err(TryCreateManyWithSerializeDeserialize ::
            CreateManyPayloadTryFromCreateManyPayloadWithSerializeDeserialize
            {
                create_many_payload_try_from_create_many_payload_with_serialize_deserialize,
                code_occurence
            }), TryCreateManyResponseVariants :: CommitExtractorNotEqual
            { commit_not_equal, commit_to_use, code_occurence } =>
            Err(TryCreateManyWithSerializeDeserialize ::
            CommitExtractorNotEqual
            { commit_not_equal, commit_to_use, code_occurence }),
            TryCreateManyResponseVariants :: CommitExtractorToStrConversion
            { commit_to_str_conversion, code_occurence } =>
            Err(TryCreateManyWithSerializeDeserialize ::
            CommitExtractorToStrConversion
            { commit_to_str_conversion, code_occurence }),
            TryCreateManyResponseVariants :: NoCommitExtractorHeader
            { no_commit_header, code_occurence } =>
            Err(TryCreateManyWithSerializeDeserialize ::
            NoCommitExtractorHeader { no_commit_header, code_occurence })
        }
    }
}
#[derive(Debug, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum TryCreateManyRequestError {
    ExpectedType {
        #[eo_display_with_serialize_deserialize]
        expected_type: TryCreateManyWithSerializeDeserialize,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    UnexpectedStatusCode {
        #[eo_display]
        status_code: http::StatusCode,
        #[eo_display_foreign_type]
        headers: reqwest::header::HeaderMap,
        #[eo_display_foreign_type]
        response_text_result: crate::common::api_request_unexpected_error::ResponseTextResult,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    FailedToGetResponseText {
        #[eo_display_foreign_type]
        reqwest: reqwest::Error,
        #[eo_display]
        status_code: http::StatusCode,
        #[eo_display_foreign_type]
        headers: reqwest::header::HeaderMap,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    DeserializeResponse {
        #[eo_display]
        serde: serde_json::Error,
        #[eo_display]
        status_code: http::StatusCode,
        #[eo_display_foreign_type]
        headers: reqwest::header::HeaderMap,
        #[eo_display_with_serialize_deserialize]
        response_text: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Reqwest {
        #[eo_display_foreign_type]
        reqwest: reqwest::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
pub enum TryCreateManyStatusCodesChecker {
    ConfigurationTvfrr500InternalServerError,
    DatabaseTvfrr500InternalServerError,
    IoTvfrr500InternalServerError,
    TlsTvfrr500InternalServerError,
    ProtocolTvfrr500InternalServerError,
    RowNotFoundTvfrr404NotFound,
    TypeNotFoundTvfrr400BadRequest,
    ColumnIndexOutOfBoundsTvfrr500InternalServerError,
    ColumnNotFoundTvfrr400BadRequest,
    ColumnDecodeTvfrr500InternalServerError,
    DecodeTvfrr500InternalServerError,
    PoolTimedOutTvfrr408RequestTimeout,
    PoolClosedTvfrr500InternalServerError,
    WorkerCrashedTvfrr500InternalServerError,
    MigrateTvfrr500InternalServerError,
    JsonDataErrorTvfrr400BadRequest,
    JsonSyntaxErrorTvfrr400BadRequest,
    MissingJsonContentTypeTvfrr400BadRequest,
    BytesRejectionTvfrr500InternalServerError,
    UnexpectedCaseTvfrr500InternalServerError,
    BindQueryTvfrr500InternalServerError,
    OperationDoneButCannotConvertUuidWrapperFromPossibleUuidWrapperInServerTvfrr500InternalServerError,
    CreateManyPayloadTryFromCreateManyPayloadWithSerializeDeserializeTvfrr400BadRequest,
    CommitExtractorNotEqualTvfrr400BadRequest,
    CommitExtractorToStrConversionTvfrr400BadRequest,
    NoCommitExtractorHeaderTvfrr400BadRequest,
}
impl axum::response::IntoResponse for TryCreateManyResponseVariants {
    fn into_response(self) -> axum::response::Response {
        match & self
        {
            TryCreateManyResponseVariants :: Desirable(_) =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: CREATED ; res
            } TryCreateManyResponseVariants :: Configuration
            { configuration : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: CREATED ; res
            }, TryCreateManyResponseVariants :: Database
            { database : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: CREATED ; res
            }, TryCreateManyResponseVariants :: Io
            { io : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: CREATED ; res
            }, TryCreateManyResponseVariants :: Tls
            { tls : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: CREATED ; res
            }, TryCreateManyResponseVariants :: Protocol
            { protocol : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: CREATED ; res
            }, TryCreateManyResponseVariants :: RowNotFound
            { row_not_found : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: CREATED ; res
            }, TryCreateManyResponseVariants :: TypeNotFound
            { type_not_found : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: CREATED ; res
            }, TryCreateManyResponseVariants :: ColumnIndexOutOfBounds
            { column_index_out_of_bounds : _, len : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: CREATED ; res
            }, TryCreateManyResponseVariants :: ColumnNotFound
            { column_not_found : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: CREATED ; res
            }, TryCreateManyResponseVariants :: ColumnDecode
            { column_decode_index : _, source_handle : _, code_occurence : _ }
            =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: CREATED ; res
            }, TryCreateManyResponseVariants :: Decode
            { decode : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: CREATED ; res
            }, TryCreateManyResponseVariants :: PoolTimedOut
            { pool_timed_out : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: CREATED ; res
            }, TryCreateManyResponseVariants :: PoolClosed
            { pool_closed : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: CREATED ; res
            }, TryCreateManyResponseVariants :: WorkerCrashed
            { worker_crashed : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: CREATED ; res
            }, TryCreateManyResponseVariants :: Migrate
            { migrate : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: CREATED ; res
            }, TryCreateManyResponseVariants :: JsonDataError
            { json_data_error : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: CREATED ; res
            }, TryCreateManyResponseVariants :: JsonSyntaxError
            { json_syntax_error : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: CREATED ; res
            }, TryCreateManyResponseVariants :: MissingJsonContentType
            { missing_json_content_type : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: CREATED ; res
            }, TryCreateManyResponseVariants :: BytesRejection
            { bytes_rejection : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: CREATED ; res
            }, TryCreateManyResponseVariants :: UnexpectedCase
            { unexpected_case : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: CREATED ; res
            }, TryCreateManyResponseVariants :: BindQuery
            { bind_query : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: CREATED ; res
            }, TryCreateManyResponseVariants ::
            OperationDoneButCannotConvertUuidWrapperFromPossibleUuidWrapperInServer
            {
                operation_done_but_cannot_convert_uuid_wrapper_from_possible_uuid_wrapper_in_server
                : _, code_occurence : _
            } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: CREATED ; res
            }, TryCreateManyResponseVariants ::
            CreateManyPayloadTryFromCreateManyPayloadWithSerializeDeserialize
            {
                create_many_payload_try_from_create_many_payload_with_serialize_deserialize
                : _, code_occurence : _
            } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: CREATED ; res
            }, TryCreateManyResponseVariants :: CommitExtractorNotEqual
            { commit_not_equal : _, commit_to_use : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: CREATED ; res
            }, TryCreateManyResponseVariants :: CommitExtractorToStrConversion
            { commit_to_str_conversion : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: CREATED ; res
            }, TryCreateManyResponseVariants :: NoCommitExtractorHeader
            { no_commit_header : _, code_occurence : _ } =>
            {
                let mut res = axum :: Json(self).into_response() ; *
                res.status_mut() = axum :: http :: StatusCode :: CREATED ; res
            }
        }
    }
}
#[derive(Debug, thiserror :: Error, error_occurence_lib :: ErrorOccurence)]
pub enum TryCreateManyErrorNamed {
    SerdeJsonToString {
        #[eo_display]
        serde_json_to_string: serde_json::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    OperationDoneButCannotConvertUuidWrapperFromPossibleUuidWrapperInClient {
        #[eo_vec_error_occurence]
        operation_done_but_cannot_convert_uuid_wrapper_from_possible_uuid_wrapper_in_client:
            std::vec::Vec<
                OperationDoneButCannotConvertUuidWrapperFromPossibleUuidWrapperInClientErrorUnnamed,
            >,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    ExpectedType {
        #[eo_display_with_serialize_deserialize]
        expected_type: TryCreateManyWithSerializeDeserialize,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    UnexpectedStatusCode {
        #[eo_display]
        status_code: http::StatusCode,
        #[eo_display_foreign_type]
        headers: reqwest::header::HeaderMap,
        #[eo_display_foreign_type]
        response_text_result: crate::common::api_request_unexpected_error::ResponseTextResult,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    FailedToGetResponseText {
        #[eo_display_foreign_type]
        reqwest: reqwest::Error,
        #[eo_display]
        status_code: http::StatusCode,
        #[eo_display_foreign_type]
        headers: reqwest::header::HeaderMap,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    DeserializeResponse {
        #[eo_display]
        serde: serde_json::Error,
        #[eo_display]
        status_code: http::StatusCode,
        #[eo_display_foreign_type]
        headers: reqwest::header::HeaderMap,
        #[eo_display_with_serialize_deserialize]
        response_text: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    Reqwest {
        #[eo_display_foreign_type]
        reqwest: reqwest::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
pub async fn try_create_many<'a>(
    server_location: &str,
    parameters: CreateManyParameters,
) -> Result<
    std::vec::Vec<crate::server::postgres::uuid_wrapper::UuidWrapper>,
    TryCreateManyErrorNamed,
> {
    let payload = match serde_json::to_string(&CreateManyPayloadWithSerializeDeserialize::from(
        parameters.payload,
    )) {
        Ok(value) => value,
        Err(e) => {
            return Err(TryCreateManyErrorNamed::SerdeJsonToString {
                serde_json_to_string: e,
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    crate::global_variables::compile_time::project_git_info::PROJECT_GIT_INFO
                        .commit
                        .to_string(),
                    file!().to_string(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: std::string::String::from(
                            "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                        ),
                        line: 1697,
                        column: 13,
                    }),
                ),
            });
        }
    };
    let url = format!("{}/dogs/create_many", server_location,);
    let future = reqwest::Client::new()
        .post(&url)
        .header(
            postgresql_crud::COMMIT,
            crate::global_variables::compile_time::project_git_info::PROJECT_GIT_INFO.commit,
        )
        .header(reqwest::header::CONTENT_TYPE, "application/json")
        .body(payload)
        .send();
    let response = match future.await {
        Ok(response) => response,
        Err(e) => {
            return Err(TryCreateManyErrorNamed::Reqwest {
                reqwest: e,
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    crate::global_variables::compile_time::project_git_info::PROJECT_GIT_INFO
                        .commit
                        .to_string(),
                    file!().to_string(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: std::string::String::from(
                            "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                        ),
                        line: 2339,
                        column: 13,
                    }),
                ),
            });
        }
    };
    let status_code = response.status();
    let headers = response.headers().clone();
    let response_text = match response.text().await {
        Ok(response_text) => response_text,
        Err(e) => {
            return Err(TryCreateManyErrorNamed::FailedToGetResponseText {
                reqwest: e,
                status_code,
                headers,
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    crate::global_variables::compile_time::project_git_info::PROJECT_GIT_INFO
                        .commit
                        .to_string(),
                    file!().to_string(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: std::string::String::from(
                            "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                        ),
                        line: 2275,
                        column: 13,
                    }),
                ),
            });
        }
    };
    let variants =
        if status_code == http::StatusCode::CREATED {
            match serde_json::from_str::<TryCreateManyResponseVariantsTvfrr201Created>(
                &response_text,
            ) {
                Ok(value) => TryCreateManyResponseVariants::from(value),
                Err(e) => {
                    return
                Err(TryCreateManyErrorNamed :: DeserializeResponse
                {
                    serde : e, status_code, headers, response_text,
                    code_occurence : error_occurence_lib :: code_occurence ::
                    CodeOccurence ::
                    new(crate :: global_variables :: compile_time ::
                    project_git_info :: PROJECT_GIT_INFO.commit.to_string(),
                    file! ().to_string(), line! (), column! (),
                    Some(error_occurence_lib :: code_occurence :: MacroOccurence
                    {
                        file : std :: string :: String ::
                        from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                        line : 2306, column : 13,
                    }))
                }) ;
                }
            }
        } else if status_code == http::StatusCode::REQUEST_TIMEOUT {
            match serde_json::from_str::<TryCreateManyResponseVariantsTvfrr408RequestTimeout>(
                &response_text,
            ) {
                Ok(value) => TryCreateManyResponseVariants::from(value),
                Err(e) => {
                    return
                Err(TryCreateManyErrorNamed :: DeserializeResponse
                {
                    serde : e, status_code, headers, response_text,
                    code_occurence : error_occurence_lib :: code_occurence ::
                    CodeOccurence ::
                    new(crate :: global_variables :: compile_time ::
                    project_git_info :: PROJECT_GIT_INFO.commit.to_string(),
                    file! ().to_string(), line! (), column! (),
                    Some(error_occurence_lib :: code_occurence :: MacroOccurence
                    {
                        file : std :: string :: String ::
                        from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                        line : 2306, column : 13,
                    }))
                }) ;
                }
            }
        } else if status_code == http::StatusCode::NOT_FOUND {
            match serde_json::from_str::<TryCreateManyResponseVariantsTvfrr404NotFound>(
                &response_text,
            ) {
                Ok(value) => TryCreateManyResponseVariants::from(value),
                Err(e) => {
                    return
                Err(TryCreateManyErrorNamed :: DeserializeResponse
                {
                    serde : e, status_code, headers, response_text,
                    code_occurence : error_occurence_lib :: code_occurence ::
                    CodeOccurence ::
                    new(crate :: global_variables :: compile_time ::
                    project_git_info :: PROJECT_GIT_INFO.commit.to_string(),
                    file! ().to_string(), line! (), column! (),
                    Some(error_occurence_lib :: code_occurence :: MacroOccurence
                    {
                        file : std :: string :: String ::
                        from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                        line : 2306, column : 13,
                    }))
                }) ;
                }
            }
        } else if status_code == http::StatusCode::BAD_REQUEST {
            match serde_json::from_str::<TryCreateManyResponseVariantsTvfrr400BadRequest>(
                &response_text,
            ) {
                Ok(value) => TryCreateManyResponseVariants::from(value),
                Err(e) => {
                    return
                Err(TryCreateManyErrorNamed :: DeserializeResponse
                {
                    serde : e, status_code, headers, response_text,
                    code_occurence : error_occurence_lib :: code_occurence ::
                    CodeOccurence ::
                    new(crate :: global_variables :: compile_time ::
                    project_git_info :: PROJECT_GIT_INFO.commit.to_string(),
                    file! ().to_string(), line! (), column! (),
                    Some(error_occurence_lib :: code_occurence :: MacroOccurence
                    {
                        file : std :: string :: String ::
                        from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                        line : 2306, column : 13,
                    }))
                }) ;
                }
            }
        } else {
            return Err(TryCreateManyErrorNamed::UnexpectedStatusCode {
                status_code,
                headers,
                response_text_result:
                    crate::common::api_request_unexpected_error::ResponseTextResult::ResponseText(
                        response_text,
                    ),
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    crate::global_variables::compile_time::project_git_info::PROJECT_GIT_INFO
                        .commit
                        .to_string(),
                    file!().to_string(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: std::string::String::from(
                            "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                        ),
                        line: 2244,
                        column: 13,
                    }),
                ),
            });
        };
    match std::vec::Vec::<crate::server::postgres::uuid_wrapper::PossibleUuidWrapper>::try_from(
        variants,
    ) {
        Ok(value) => {
            let mut vec_values = std::vec::Vec::with_capacity(value.len());
            let mut vec_errors = std::vec::Vec::with_capacity(value.len());
            for element in value {
                match crate::server::postgres::uuid_wrapper::UuidWrapper::try_from(element) {
                    Ok(value) => {
                        vec_values.push(value);
                    }
                    Err(e) => {
                        vec_errors.push(OperationDoneButCannotConvertUuidWrapperFromPossibleUuidWrapperInClientErrorUnnamed
                        ::
                        OperationDoneButCannotConvertUuidWrapperFromPossibleUuidWrapperInClient(e))
                        ;
                    }
                }
            }
            if let false = vec_errors.is_empty() {
                return
                Err(TryCreateManyErrorNamed ::
                OperationDoneButCannotConvertUuidWrapperFromPossibleUuidWrapperInClient
                {
                    operation_done_but_cannot_convert_uuid_wrapper_from_possible_uuid_wrapper_in_client
                    : vec_errors, code_occurence : error_occurence_lib ::
                    code_occurence :: CodeOccurence ::
                    new(crate :: global_variables :: compile_time ::
                    project_git_info :: PROJECT_GIT_INFO.commit.to_string(),
                    file! ().to_string(), line! (), column! (),
                    Some(error_occurence_lib :: code_occurence :: MacroOccurence
                    {
                        file : std :: string :: String ::
                        from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                        line : 2003, column : 13,
                    }))
                }) ;
            }
            Ok(vec_values)
        }
        Err(e) => {
            return Err(TryCreateManyErrorNamed::ExpectedType {
                expected_type: e,
                code_occurence: error_occurence_lib::code_occurence::CodeOccurence::new(
                    crate::global_variables::compile_time::project_git_info::PROJECT_GIT_INFO
                        .commit
                        .to_string(),
                    file!().to_string(),
                    line!(),
                    column!(),
                    Some(error_occurence_lib::code_occurence::MacroOccurence {
                        file: std::string::String::from(
                            "postgresql_crud/generate_postgresql_crud/src/lib.rs",
                        ),
                        line: 2213,
                        column: 13,
                    }),
                ),
            });
        }
    }
}
#[utoipa ::
path(post, path = "/dogs/create_many", operation_id = "/dogs/create_many", tag
= "dogs",
request_body(content = CreateManyPayload, description =
"dogs create_many payload", content_type = "application/json"),
responses((status = 201, description = "created", body =
TryCreateManyResponseVariantsTvfrr201Created, content_type =
"application/json"),
(status = 500, description = "internal server error", body =
TryCreateManyResponseVariantsTvfrr500InternalServerError, content_type =
"application/json"),
(status = 404, description = "not found", body =
TryCreateManyResponseVariantsTvfrr404NotFound, content_type =
"application/json"),
(status = 400, description = "bad request", body =
TryCreateManyResponseVariantsTvfrr400BadRequest, content_type =
"application/json"),
(status = 408, description = "request timeout", body =
TryCreateManyResponseVariantsTvfrr408RequestTimeout, content_type =
"application/json")),)]
pub async fn create_many(
    app_state: axum::extract::State<
        postgresql_crud::app_state::DynArcGetConfigGetPostgresPoolSendSync,
    >,
    payload_extraction_result: Result<
        axum::Json<CreateManyPayloadWithSerializeDeserialize>,
        axum::extract::rejection::JsonRejection,
    >,
) -> impl axum::response::IntoResponse {
    let parameters = CreateManyParameters {
        payload:
            match crate::server::routes::helpers::json_extractor_error::JsonValueResultExtractor::<
                CreateManyPayloadWithSerializeDeserialize,
                TryCreateManyResponseVariants,
            >::try_extract_value(payload_extraction_result, &app_state)
            {
                Ok(value) => match CreateManyPayload::try_from(value) {
                    Ok(value) => value,
                    Err(e) => {
                        let e = TryCreateMany ::
                    CreateManyPayloadTryFromCreateManyPayloadWithSerializeDeserialize
                    {
                        create_many_payload_try_from_create_many_payload_with_serialize_deserialize
                        : e, code_occurence : error_occurence_lib :: code_occurence
                        :: CodeOccurence ::
                        new(crate :: global_variables :: compile_time ::
                        project_git_info :: PROJECT_GIT_INFO.commit.to_string(),
                        file! ().to_string(), line! (), column! (),
                        Some(error_occurence_lib :: code_occurence :: MacroOccurence
                        {
                            file : std :: string :: String ::
                            from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                            line : 2953, column : 17,
                        })),
                    } ;
                        error_occurence_lib::error_log::ErrorLog::error_log(&e, app_state.as_ref());
                        return TryCreateManyResponseVariants::from(e);
                    }
                },
                Err(e) => {
                    return e;
                }
            },
    };
    println!("{:#?}", parameters);
    {
        let query_string = {
            "insert into dogs (name, color) select name, color from unnest($1, $2) as a(name, color) returning id"
        };
        println!("{}", query_string);
        let binded_query = {
            let mut query = sqlx::query::<sqlx::Postgres>(&query_string);
            let current_vec_len = parameters.payload.0.len();
            let (name_vec, color_vec) = parameters.payload.0.into_iter().fold(
                (
                    std::vec::Vec::with_capacity(current_vec_len),
                    std::vec::Vec::with_capacity(current_vec_len),
                ),
                |mut acc, element| {
                    acc.0.push(element.name);
                    acc.1.push(element.color);
                    acc
                },
            );
            query = query.bind(
                name_vec.into_iter().map(|element|element.into_inner()).collect()
            );
            query = query.bind(
                color_vec.into_iter().map(|element|element.into_inner()).collect()
            );
            query
        };
        let mut pool_connection = match app_state.get_postgres_pool().acquire().await {
            Ok(value) => value,
            Err(e) => {
                let e = TryCreateMany::from(e);
                error_occurence_lib::error_log::ErrorLog::error_log(&e, app_state.as_ref());
                return TryCreateManyResponseVariants::from(e);
            }
        };
        let pg_connection = match sqlx::Acquire::acquire(&mut pool_connection).await {
            Ok(value) => value,
            Err(e) => {
                let e = TryCreateMany::from(e);
                error_occurence_lib::error_log::ErrorLog::error_log(&e, app_state.as_ref());
                return TryCreateManyResponseVariants::from(e);
            }
        };
        let mut rows = binded_query.fetch(pg_connection.as_mut());
        let mut vec_values = std::vec::Vec::new();
        while let Some(row) = {
            match {
                use futures::TryStreamExt;
                rows.try_next()
            }
            .await
            {
                Ok(value) => value,
                Err(e) => {
                    let e = TryCreateMany::from(e);
                    error_occurence_lib::error_log::ErrorLog::error_log(&e, app_state.as_ref());
                    return TryCreateManyResponseVariants::from(e);
                }
            }
        } {
            match {
                use sqlx::Row;
                row.try_get::<sqlx::types::Uuid, &str>("id")
            } {
                Ok(value) => {
                    vec_values.push(
                        crate::server::postgres::uuid_wrapper::PossibleUuidWrapper::from(value),
                    );
                }
                Err(e) => {
                    let e = TryCreateMany ::
                    OperationDoneButCannotConvertUuidWrapperFromPossibleUuidWrapperInServer
                    {
                        operation_done_but_cannot_convert_uuid_wrapper_from_possible_uuid_wrapper_in_server
                        : e, code_occurence : error_occurence_lib :: code_occurence
                        :: CodeOccurence ::
                        new(crate :: global_variables :: compile_time ::
                        project_git_info :: PROJECT_GIT_INFO.commit.to_string(),
                        file! ().to_string(), line! (), column! (),
                        Some(error_occurence_lib :: code_occurence :: MacroOccurence
                        {
                            file : std :: string :: String ::
                            from("postgresql_crud/generate_postgresql_crud/src/lib.rs"),
                            line : 1982, column : 13,
                        })),
                    } ;
                    error_occurence_lib::error_log::ErrorLog::error_log(&e, app_state.as_ref());
                    return TryCreateManyResponseVariants::from(e);
                }
            }
        }
        TryCreateManyResponseVariants::Desirable(vec_values)
    }
}
impl std::convert::From<crate::server::extractors::commit_extractor::CommitExtractorCheckErrorNamed>
    for TryCreateMany
{
    fn from(
        value: crate::server::extractors::commit_extractor::CommitExtractorCheckErrorNamed,
    ) -> Self {
        match value
        {
            crate::server::extractors::commit_extractor::CommitExtractorCheckErrorNamed
            :: CommitExtractorNotEqual
            { commit_not_equal, commit_to_use, code_occurence } => Self ::
            CommitExtractorNotEqual
            { commit_not_equal, commit_to_use, code_occurence },
            crate::server::extractors::commit_extractor::CommitExtractorCheckErrorNamed
            :: CommitExtractorToStrConversion
            { commit_to_str_conversion, code_occurence } => Self ::
            CommitExtractorToStrConversion
            { commit_to_str_conversion, code_occurence },
            crate::server::extractors::commit_extractor::CommitExtractorCheckErrorNamed
            :: NoCommitExtractorHeader { no_commit_header, code_occurence } =>
            Self :: NoCommitExtractorHeader
            { no_commit_header, code_occurence }
        }
    }
}
