#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)]
pub struct StdPrimitiveI8SJson(std::primitive::i8);

/////////////////////
pub trait GeneratePostgresqlQueryPart<T> {
    fn generate_postgresql_query_part(&self, column_name_and_maybe_field_getter: &std::primitive::str) -> Result<std::string::String, T>;
}
//todo generate wrapper type for all possible json type
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)] //user type must implement utoipa::ToSchema trait
pub struct Something {
    pub something: std::string::String,
    pub omega: std::vec::Vec<std::primitive::bool>,
    // #[json_field_name_stringified_reader] //todo for the future proc macro
    pub doggie: Doggie,
    pub cats: std::vec::Vec<Cat>,
}


// fn deserialize_i8
// fn deserialize_i16
// fn deserialize_i32
// fn deserialize_i64
// fn deserialize_i128
// fn deserialize_u8
// fn deserialize_u16
// fn deserialize_u32
// fn deserialize_u64
// fn deserialize_u128
// fn deserialize_f32
// fn deserialize_f64
// fn deserialize_bool
// fn deserialize_char
// fn deserialize_str
// fn deserialize_string



// pub enum Value {
//     Null,
//     Bool(bool),
//     Number(Number),
//     String(String),
//     Array(Vec<Value>),
//     Object(Map<String, Value>),
// }

impl std::fmt::Display for Something {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{:?}", &self)
    }
}
impl std::convert::From<Something> for SomethingOptions {
    fn from(value: Something) -> Self {
        Self {
            something: Some(value.something),
            omega: Some(value.omega),
            doggie: Some(DoggieOptions::from(value.doggie)),
            cats: Some(value.cats.into_iter().map(|element|CatOptions::from(element)).collect::<std::vec::Vec<CatOptions>>()),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)]
pub enum SomethingReader {
    #[serde(rename(
        serialize = "something",
        deserialize = "something"
    ))]
    Something,
    #[serde(rename(
        serialize = "omega",
        deserialize = "omega"
    ))]
    Omega {
        limit: std::primitive::u64,
        offset: std::primitive::u64,
    },
    #[serde(rename(
        serialize = "doggie",
        deserialize = "doggie"
    ))]
    Doggie(std::vec::Vec<DoggieReader>),
    #[serde(rename(
        serialize = "cats",
        deserialize = "cats"
    ))]
    Cats {
        reader_vec: std::vec::Vec<CatReader>,
        limit: std::primitive::u64,
        offset: std::primitive::u64,
    }
}
impl error_occurence_lib::ToStdStringString for SomethingReader {
    fn to_std_string_string(&self) -> std::string::String {
        format!("{self:?}")
    }
}
#[derive(Debug, thiserror::Error, error_occurence_lib::ErrorOccurence)]
pub enum SomethingGeneratePostgresqlQueryPartErrorNamed {
    OffsetPlusLimitIsIntOverflow {
        #[eo_to_std_string_string_serialize_deserialize]
        limit: std::primitive::u64,
        #[eo_to_std_string_string_serialize_deserialize]
        offset: std::primitive::u64,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    FieldsFilterIsEmpty {
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NotUniqueDoggieFieldFilter {
        #[eo_to_std_string_string_serialize_deserialize]
        field: DoggieReader,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NotUniqueCatsFieldFilter {
        #[eo_to_std_string_string_serialize_deserialize]
        field: CatReader,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    }
}
impl GeneratePostgresqlQueryPart<SomethingGeneratePostgresqlQueryPartErrorNamed> for SomethingReader {
    //todo return result instead of std::string::String
    fn generate_postgresql_query_part(&self, column_name_and_maybe_field_getter: &std::primitive::str) -> Result<std::string::String, SomethingGeneratePostgresqlQueryPartErrorNamed> {
        match self {
            Self::Something => Ok(format!("'something',{column_name_and_maybe_field_getter}->'something'")),
            Self::Omega {
                limit,
                offset
            } => {
                let start = offset;
                let end = match offset.checked_add(*limit) {
                    Some(value) => value,
                    None => {
                        return Err(SomethingGeneratePostgresqlQueryPartErrorNamed::OffsetPlusLimitIsIntOverflow {
                            limit: *limit,
                            offset: *offset,
                            code_occurence: error_occurence_lib::code_occurence!(),
                        });
                    }
                };
                Ok(format!("'omega',(select json_agg(value) from json_array_elements((select {column_name_and_maybe_field_getter}->'omega')) with ordinality where ordinality between {start} and {end})"))
            },
            Self::Doggie(reader_vec) => Ok(format!(
                "'doggie',jsonb_build_object({})",
                {
                    if reader_vec.is_empty() {
                        return Err(SomethingGeneratePostgresqlQueryPartErrorNamed::FieldsFilterIsEmpty {
                            code_occurence: error_occurence_lib::code_occurence!(),
                        });
                    }
                    let mut unique_reader_vec = vec![];
                    for element in reader_vec {
                        if unique_reader_vec.contains(&element) {
                            return Err(SomethingGeneratePostgresqlQueryPartErrorNamed::NotUniqueDoggieFieldFilter {
                                field: *element,
                                code_occurence: error_occurence_lib::code_occurence!(),
                            });
                        }
                        else {
                            unique_reader_vec.push(&element);
                        }
                    }
                    let mut acc = reader_vec.iter().fold(std::string::String::default(), |mut acc, element| {
                        acc.push_str(&format!(
                            "{},",
                            element.generate_postgresql_query_part(&format!("{column_name_and_maybe_field_getter}->'doggie'")).unwrap()//todo return error
                        ));
                        acc
                    });
                    let _ = acc.pop();
                    acc
                }
            )),
            Self::Cats {
                reader_vec,
                limit,
                offset
            } => {
                if reader_vec.is_empty() {
                    return Err(SomethingGeneratePostgresqlQueryPartErrorNamed::FieldsFilterIsEmpty {
                        code_occurence: error_occurence_lib::code_occurence!(),
                    });
                }
                let mut unique_reader_vec = vec![];
                for element in reader_vec {
                    if unique_reader_vec.contains(&element) {
                        return Err(SomethingGeneratePostgresqlQueryPartErrorNamed::NotUniqueCatsFieldFilter {
                            field: *element,
                            code_occurence: error_occurence_lib::code_occurence!(),
                        });
                    }
                    else {
                        unique_reader_vec.push(&element);
                    }
                }
                let mut acc = reader_vec.iter().fold(std::string::String::default(), |mut acc, element| {
                    acc.push_str(&format!(
                        "{},",
                        element.generate_postgresql_query_part("value").unwrap()//todo return error//todo if it two inner[][] - is it correct to use value still?
                    ));
                    acc
                });
                let _ = acc.pop();
                let start = offset;
                let end = match offset.checked_add(*limit) {
                    Some(value) => value,
                    None => {
                        return Err(SomethingGeneratePostgresqlQueryPartErrorNamed::OffsetPlusLimitIsIntOverflow {
                            limit: *limit,
                            offset: *offset,
                            code_occurence: error_occurence_lib::code_occurence!(),
                        });
                    }
                };
                Ok(format!("'cats',(select json_agg(jsonb_build_object({acc})) from json_array_elements((select sqlx_types_json_t_as_postgresql_json_not_null->'cats')) with ordinality where ordinality between {start} AND {end})"))
            }
        }
    }
}

// SELECT 
//     jsonb_build_object(
//         'cats',
//         (SELECT json_agg(jsonb_build_object('meow', value->>'meow')) 
//          FROM json_array_elements(
//                  (SELECT sqlx_types_json_t_as_postgresql_json_not_null->'cats')
//              ) WITH ORDINALITY 
//              WHERE ordinality BETWEEN 0 AND 4)
//     ) AS sqlx_types_json_t_as_postgresql_json_not_null 
// FROM jsongeneric 
// WHERE std_primitive_i64_as_postgresql_big_serial_not_null_primary_key = 1;



#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize, utoipa::ToSchema)] //user type must implement utoipa::ToSchema trait
pub struct SomethingOptions {
    something: std::option::Option<std::string::String>,
    omega: std::option::Option<std::vec::Vec<bool>>,
    // #[json_field_name_stringified_reader] //todo for the future proc macro
    doggie: std::option::Option<DoggieOptions>,
    cats: std::option::Option<std::vec::Vec<CatOptions>>,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)] //user type must implement utoipa::ToSchema trait
pub struct Doggie {
    pub says: std::string::String,
}
impl std::convert::From<Doggie> for DoggieOptions {
    fn from(value: Doggie) -> Self {
        Self {
            says: Some(value.says)
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)]
pub enum DoggieReader {
    #[serde(rename(
        serialize = "says",
        deserialize = "says"
    ))]
    Says
}
impl error_occurence_lib::ToStdStringString for DoggieReader {
    fn to_std_string_string(&self) -> std::string::String {
        format!("{self:?}")
    }
}
#[derive(Debug, thiserror::Error, error_occurence_lib::ErrorOccurence)]
pub enum DoggieGeneratePostgresqlQueryPartErrorNamed {
    OffsetPlusLimitIsIntOverflow {
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    FieldsFilterIsEmpty {
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NotUniqueFieldFilter {
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    }
}
impl GeneratePostgresqlQueryPart<DoggieGeneratePostgresqlQueryPartErrorNamed> for DoggieReader {
    fn generate_postgresql_query_part(&self, column_name_and_maybe_field_getter: &std::primitive::str) -> Result<std::string::String, DoggieGeneratePostgresqlQueryPartErrorNamed> {
        match self {
            Self::Says => Ok(format!("'says',{column_name_and_maybe_field_getter}->'says'")),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)] //user type must implement utoipa::ToSchema trait
pub struct DoggieOptions {
    says: std::option::Option<std::string::String>,
}

// impl DoggieOptions {
//     fn s(value: sqlx::types::JsonValue) -> Self {
//         let f: Self = serde_json::from_value(value).unwrap();

//         f
//     }
// }
// let schema = schema_for!(Something);
// println!("{}", serde_json::to_string_pretty(&schema).unwrap());


#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)] //user type must implement utoipa::ToSchema trait
pub struct Cat {
    pub meow: std::string::String,
    pub one: std::string::String,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize, utoipa::ToSchema, schemars::JsonSchema)]
pub enum CatReader {
    #[serde(rename(
        serialize = "meow",
        deserialize = "meow"
    ))]
    Meow,
    #[serde(rename(
        serialize = "one",
        deserialize = "one"
    ))]
    One
}
impl error_occurence_lib::ToStdStringString for CatReader {
    fn to_std_string_string(&self) -> std::string::String {
        format!("{self:?}")
    }
}
#[derive(Debug, thiserror::Error, error_occurence_lib::ErrorOccurence)]
pub enum CatGeneratePostgresqlQueryPartErrorNamed {
    OffsetPlusLimitIsIntOverflow {
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    FieldsFilterIsEmpty {
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
    NotUniqueFieldFilter {
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    }
}
impl GeneratePostgresqlQueryPart<CatGeneratePostgresqlQueryPartErrorNamed> for CatReader {
    fn generate_postgresql_query_part(&self, column_name_and_maybe_field_getter: &std::primitive::str) -> Result<std::string::String, CatGeneratePostgresqlQueryPartErrorNamed> {
        match self {
            Self::Meow => Ok(format!("'meow',{column_name_and_maybe_field_getter}->'meow'")),
            Self::One => Ok(format!("'one',{column_name_and_maybe_field_getter}->'one'")),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize, utoipa::ToSchema)] //user type must implement utoipa::ToSchema trait
pub struct CatOptions {
    meow: std::option::Option<std::string::String>,
    one: std::option::Option<std::string::String>,
}
impl std::convert::From<Cat> for CatOptions {
    fn from(value: Cat) -> Self {
        Self {
            meow: Some(value.meow),
            one: Some(value.one),
        }
    }
}