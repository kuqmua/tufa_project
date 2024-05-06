// todo is there is no point to add additional info to enum like this
// eo_error_occurence_field: {
//     eo_display_with_serialize_deserialize_field: value
// }
// https://github.com/kuqmua/tufa_project/blob/ebb9f680ea508fb5df5ee5d2791e96ca34610bc2/error_occurence_test/src/main.rs#L85 2024-05-06 09:17:23
// impl display like this this
// eo_error_occurence_field
// https://github.com/kuqmua/tufa_project/blob/ebb9f680ea508fb5df5ee5d2791e96ca34610bc2/error_occurence_test/src/main.rs#L85 2024-05-06 09:17:23

#[derive(
    Debug,
    thiserror::Error,
    error_occurence_lib::ErrorOccurenceTest
)]
pub enum ErrorNamedOne {
    //use ToStdStringString for hashmap keys instead of Display
    //todo even for std::string::String in serialize deserialize version of error must be using ToStdStringString impl instead of std::fmt::Display
    //todo test on using only code_occurence as pnly field in named variant
    Variant {
        #[eo_to_std_string_string]
        eo_display_field: DisplayStruct,//IN SERIALIZE DESERIALIZE std::string::String
        #[eo_to_std_string_string_serialize_deserialize]
        eo_serde: SerializeDeserializeStruct,
        #[eo_error_occurence]
        eo_error_occurence_field: ErrorNamedTwo,//IN SERIALIZE DESERIALIZE nested
        #[eo_vec_to_std_string_string]//todo remove wrapper under std::vec::Vec
        eo_vec_display_field: std::vec::Vec<DisplayStruct>,//IN SERIALIZE DESERIALIZE std::vec::Vec<std::string::String>
        #[eo_vec_to_std_string_string_serialize_deserialize]
        eo_vec_serde: std::vec::Vec<SerializeDeserializeStruct>,
        #[eo_vec_error_occurence]
        eo_vec_error_occurence_field: std::vec::Vec<ErrorUnnamedOne>,//IN SERIALIZE DESERIALIZE std::vec::Vec<nested>
        #[eo_hashmap_key_std_string_string_value_to_std_string_string]
        hashmap_string_string: std::collections::HashMap<std::string::String, DisplayStruct>,
        #[eo_hashmap_key_std_string_string_value_to_std_string_string_serialize_deserialize]
        hashmap_string_serde: std::collections::HashMap<std::string::String, SerializeDeserializeStruct>,
        #[eo_hashmap_key_std_string_string_value_error_occurence]
        hashmap_string_error_occurence: std::collections::HashMap<std::string::String, ErrorUnnamedOne>,

        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}

#[derive(
    Debug,
    thiserror::Error,
    error_occurence_lib::ErrorOccurenceTest
)]
pub enum ErrorNamedTwo {
    Variant {
        #[eo_to_std_string_string_serialize_deserialize]
        eo_display_with_serialize_deserialize_field: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    }
}

#[derive(
    Debug, 
    thiserror::Error, 
    error_occurence_lib::ErrorOccurenceTest
)]
pub enum ErrorUnnamedOne {
    Something(ErrorNamedTwo),
}

#[derive(Debug)]
pub struct DisplayStruct {
    pub display: std::string::String,
    pub something: std::primitive::bool,
}

//todo or maybe two different traits - display foreign type and convert into serializable and deserializable type
impl error_occurence_lib::ToStdStringString for DisplayStruct {
    fn to_std_string_string(&self) -> std::string::String {
        format!("{self:?}")
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct SerializeDeserializeStruct {
    pub one: std::string::String,
    pub two: std::primitive::bool,
    pub three: std::primitive::u32,
}

impl error_occurence_lib::ToStdStringString for SerializeDeserializeStruct {
    fn to_std_string_string(&self) -> std::string::String {
        format!("{self:?}")
    }
}

fn main() {
    let e_two = ErrorNamedTwo::Variant {
        eo_display_with_serialize_deserialize_field: std::string::String::from("value"),
        code_occurence: error_occurence_lib::code_occurence!(),
    };
    println!("{e_two}");
    println!("*****************************");
    let e = ErrorNamedOne::Variant {
        eo_display_field: DisplayStruct {
            display: std::string::String::from("value"),
            something: true,
        },
        eo_serde: SerializeDeserializeStruct {
            one: std::string::String::from("value"),
            two: true,
            three: 42,
        },
        eo_error_occurence_field: e_two,
        eo_vec_display_field: vec![
            DisplayStruct {
                display: std::string::String::from("value"),
                something: true,
            }
        ],
        eo_vec_serde: vec![
            SerializeDeserializeStruct {
                one: std::string::String::from("value"),
                two: true,
                three: 42,
            }
        ],
        eo_vec_error_occurence_field: vec![
            ErrorUnnamedOne::Something(
                ErrorNamedTwo::Variant {
                    eo_display_with_serialize_deserialize_field: std::string::String::from("value"),
                    code_occurence: error_occurence_lib::code_occurence!(),
                }
            )
        ],
        hashmap_string_string: std::collections::HashMap::from([
                (
                    std::string::String::from("key"),
                    DisplayStruct {
                        display: std::string::String::from("value"),
                        something: true,
                    }
                ),
            ]),
        hashmap_string_serde: std::collections::HashMap::from([
                (
                    std::string::String::from("key"),
                    SerializeDeserializeStruct {
                        one: std::string::String::from("value"),
                        two: true,
                        three: 42,
                    }
                ),
            ]),
        hashmap_string_error_occurence: std::collections::HashMap::from([
            (
                std::string::String::from("key"),
                ErrorUnnamedOne::Something(
                    ErrorNamedTwo::Variant {
                        eo_display_with_serialize_deserialize_field: std::string::String::from("value"),
                        code_occurence: error_occurence_lib::code_occurence!(),
                    }
                )
            ),
        ]),

        code_occurence: error_occurence_lib::code_occurence!(),
    };
    println!("{e}");
    let e_serialize_deserialize_version = e.into_serialize_deserialize_version();
    println!("--------------------------------------------------------------------------------------------------");
    println!("{e_serialize_deserialize_version}");
    let e_serialize_deserialize_version_json_string = serde_json::to_string(&e_serialize_deserialize_version).unwrap();
    println!("{e_serialize_deserialize_version_json_string}");
    println!("+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++");
    let e_serialize_deserialize_version_deserialized: ErrorNamedOneWithSerializeDeserialize = serde_json::from_str(&e_serialize_deserialize_version_json_string).unwrap();
    println!("{e_serialize_deserialize_version_deserialized}");
    println!("@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@");
}

///////////////////////////////
impl std::fmt::Display for ErrorNamedOne {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!
        (formatter, "{}", error_occurence_lib :: helpers ::
        source_and_code_occurence_formatter(error_occurence_lib ::
        source_to_string_without_config :: SourceToStringWithoutConfig ::
        source_to_string_without_config(self), error_occurence_lib ::
        code_occurence :: Get :: get(self),))
    }
}
impl error_occurence_lib::source_to_string_with_config::SourceToStringWithConfig<'_>
    for ErrorNamedOne
{
    fn source_to_string_with_config<
        ConfigGeneric: config_lib::GetSourcePlaceType + config_lib::GetTimezone + ?Sized,
    >(
        &self,
        config: &ConfigGeneric,
    ) -> std::string::String {
        match self {
            ErrorNamedOne::Variant {
                eo_display_field,
                eo_serde,
                eo_error_occurence_field,
                eo_vec_display_field,
                eo_vec_serde,
                eo_vec_error_occurence_field,
                hashmap_string_string,
                hashmap_string_serde,
                hashmap_string_error_occurence,
                ..
            } => {
                format!
                ("{{
{}{}{}{}{}{}{}{}{}}}", error_occurence_lib ::
                lines_space_backslash :: LinesSpaceBackslash ::
                lines_space_backslash(& format!
                ("eo_display_field: {}", error_occurence_lib ::
                ToStdStringString :: to_std_string_string(eo_display_field))),
                error_occurence_lib :: lines_space_backslash ::
                LinesSpaceBackslash ::
                lines_space_backslash(& format!
                ("eo_serde: {}", error_occurence_lib :: ToStdStringString ::
                to_std_string_string(eo_serde))), error_occurence_lib ::
                lines_space_backslash :: LinesSpaceBackslash ::
                lines_space_backslash(& format!
                ("eo_error_occurence_field: {}", eo_error_occurence_field)),
                error_occurence_lib :: lines_space_backslash ::
                LinesSpaceBackslash ::
                lines_space_backslash(& format!
                ("eo_vec_display_field: {}", error_occurence_lib :: helpers ::
                stringified_lines_error_vec(eo_vec_display_field.iter().fold(std
                :: string :: String :: from(""), | mut acc, element |
                {
                    acc.push_str(& error_occurence_lib :: helpers ::
                    lines_space_backslash_addition(& error_occurence_lib ::
                    ToStdStringString :: to_std_string_string(element))); acc
                },)))), error_occurence_lib :: lines_space_backslash ::
                LinesSpaceBackslash ::
                lines_space_backslash(& format!
                ("eo_vec_serde: {}", error_occurence_lib :: helpers ::
                stringified_lines_error_vec(eo_vec_serde.iter().fold(std ::
                string :: String :: from(""), | mut acc, element |
                {
                    acc.push_str(& error_occurence_lib :: helpers ::
                    lines_space_backslash_addition(& error_occurence_lib ::
                    ToStdStringString :: to_std_string_string(element))); acc
                },)))), error_occurence_lib :: lines_space_backslash ::
                LinesSpaceBackslash ::
                lines_space_backslash(& format!
                ("eo_vec_error_occurence_field: {}", error_occurence_lib ::
                helpers ::
                stringified_lines_error_vec(eo_vec_error_occurence_field.iter().fold(std
                :: string :: String :: from(""), | mut acc, element |
                {
                    acc.push_str(& error_occurence_lib :: helpers ::
                    lines_space_backslash_addition(& element)); acc
                },)))), error_occurence_lib :: lines_space_backslash ::
                LinesSpaceBackslash ::
                lines_space_backslash(& format!
                ("hashmap_string_string: {}", error_occurence_lib :: helpers
                ::
                error_occurence_hashmap_formatter(hashmap_string_string.iter().fold(std
                :: string :: String :: new(), | mut acc, (key, value) |
                {
                    acc.push_str(& error_occurence_lib :: helpers ::
                    stringified_lines_error_hashmap_element(& key, &
                    error_occurence_lib :: ToStdStringString ::
                    to_std_string_string(value),)); acc
                },)))), error_occurence_lib :: lines_space_backslash ::
                LinesSpaceBackslash ::
                lines_space_backslash(& format!
                ("hashmap_string_serde: {}", error_occurence_lib :: helpers ::
                error_occurence_hashmap_formatter(hashmap_string_serde.iter().fold(std
                :: string :: String :: new(), | mut acc, (key, value) |
                {
                    acc.push_str(& error_occurence_lib :: helpers ::
                    stringified_lines_error_hashmap_element(& key, &
                    error_occurence_lib :: ToStdStringString ::
                    to_std_string_string(value),)); acc
                },)))), error_occurence_lib :: lines_space_backslash ::
                LinesSpaceBackslash ::
                lines_space_backslash(& format!
                ("hashmap_string_error_occurence: {}", error_occurence_lib ::
                helpers ::
                error_occurence_hashmap_formatter(hashmap_string_error_occurence.iter().fold(std
                :: string :: String :: new(), | mut acc, (key, value) |
                {
                    acc.push_str(& error_occurence_lib :: helpers ::
                    stringified_lines_error_hashmap_element(& key, & value,));
                    acc
                },)))))
            }
        }
    }
}
impl error_occurence_lib::source_to_string_without_config::SourceToStringWithoutConfig<'_>
    for ErrorNamedOne
{
    fn source_to_string_without_config(&self) -> std::string::String {
        match self {
            ErrorNamedOne::Variant {
                eo_display_field,
                eo_serde,
                eo_error_occurence_field,
                eo_vec_display_field,
                eo_vec_serde,
                eo_vec_error_occurence_field,
                hashmap_string_string,
                hashmap_string_serde,
                hashmap_string_error_occurence,
                ..
            } => {
                format!
                ("{{
{}{}{}{}{}{}{}{}{}}}", error_occurence_lib ::
                lines_space_backslash :: LinesSpaceBackslash ::
                lines_space_backslash(& format!
                ("eo_display_field: {}", error_occurence_lib ::
                ToStdStringString :: to_std_string_string(eo_display_field))),
                error_occurence_lib :: lines_space_backslash ::
                LinesSpaceBackslash ::
                lines_space_backslash(& format!
                ("eo_serde: {}", error_occurence_lib :: ToStdStringString ::
                to_std_string_string(eo_serde))), error_occurence_lib ::
                lines_space_backslash :: LinesSpaceBackslash ::
                lines_space_backslash(& format!
                ("eo_error_occurence_field: {}", eo_error_occurence_field)),
                error_occurence_lib :: lines_space_backslash ::
                LinesSpaceBackslash ::
                lines_space_backslash(& format!
                ("eo_vec_display_field: {}", error_occurence_lib :: helpers ::
                stringified_lines_error_vec(eo_vec_display_field.iter().fold(std
                :: string :: String :: from(""), | mut acc, element |
                {
                    acc.push_str(& error_occurence_lib :: helpers ::
                    lines_space_backslash_addition(& error_occurence_lib ::
                    ToStdStringString :: to_std_string_string(element))); acc
                },)))), error_occurence_lib :: lines_space_backslash ::
                LinesSpaceBackslash ::
                lines_space_backslash(& format!
                ("eo_vec_serde: {}", error_occurence_lib :: helpers ::
                stringified_lines_error_vec(eo_vec_serde.iter().fold(std ::
                string :: String :: from(""), | mut acc, element |
                {
                    acc.push_str(& error_occurence_lib :: helpers ::
                    lines_space_backslash_addition(& error_occurence_lib ::
                    ToStdStringString :: to_std_string_string(element))); acc
                },)))), error_occurence_lib :: lines_space_backslash ::
                LinesSpaceBackslash ::
                lines_space_backslash(& format!
                ("eo_vec_error_occurence_field: {}", error_occurence_lib ::
                helpers ::
                stringified_lines_error_vec(eo_vec_error_occurence_field.iter().fold(std
                :: string :: String :: from(""), | mut acc, element |
                {
                    acc.push_str(& error_occurence_lib :: helpers ::
                    lines_space_backslash_addition(& element)); acc
                },)))), error_occurence_lib :: lines_space_backslash ::
                LinesSpaceBackslash ::
                lines_space_backslash(& format!
                ("hashmap_string_string: {}", error_occurence_lib :: helpers
                ::
                error_occurence_hashmap_formatter(hashmap_string_string.iter().fold(std
                :: string :: String :: new(), | mut acc, (key, value) |
                {
                    acc.push_str(& error_occurence_lib :: helpers ::
                    stringified_lines_error_hashmap_element(& key, &
                    error_occurence_lib :: ToStdStringString ::
                    to_std_string_string(value),)); acc
                },)))), error_occurence_lib :: lines_space_backslash ::
                LinesSpaceBackslash ::
                lines_space_backslash(& format!
                ("hashmap_string_serde: {}", error_occurence_lib :: helpers ::
                error_occurence_hashmap_formatter(hashmap_string_serde.iter().fold(std
                :: string :: String :: new(), | mut acc, (key, value) |
                {
                    acc.push_str(& error_occurence_lib :: helpers ::
                    stringified_lines_error_hashmap_element(& key, &
                    error_occurence_lib :: ToStdStringString ::
                    to_std_string_string(value),)); acc
                },)))), error_occurence_lib :: lines_space_backslash ::
                LinesSpaceBackslash ::
                lines_space_backslash(& format!
                ("hashmap_string_error_occurence: {}", error_occurence_lib ::
                helpers ::
                error_occurence_hashmap_formatter(hashmap_string_error_occurence.iter().fold(std
                :: string :: String :: new(), | mut acc, (key, value) |
                {
                    acc.push_str(& error_occurence_lib :: helpers ::
                    stringified_lines_error_hashmap_element(& key, & value,));
                    acc
                },)))))
            }
        }
    }
}
impl error_occurence_lib::code_occurence::Get for ErrorNamedOne {
    fn get(&self) -> &error_occurence_lib::code_occurence::CodeOccurence {
        match self {
            ErrorNamedOne::Variant { code_occurence, .. } => code_occurence,
        }
    }
}
impl ErrorNamedOne {
    pub fn into_serialize_deserialize_version(self) -> ErrorNamedOneWithSerializeDeserialize {
        match self {
            ErrorNamedOne::Variant {
                eo_display_field,
                eo_serde,
                eo_error_occurence_field,
                eo_vec_display_field,
                eo_vec_serde,
                eo_vec_error_occurence_field,
                hashmap_string_string,
                hashmap_string_serde,
                hashmap_string_error_occurence,
                code_occurence,
            } => ErrorNamedOneWithSerializeDeserialize::Variant {
                eo_display_field: {
                    error_occurence_lib::ToStdStringString::to_std_string_string(&eo_display_field)
                },
                eo_serde,
                eo_error_occurence_field: {
                    eo_error_occurence_field.into_serialize_deserialize_version()
                },
                eo_vec_display_field: {
                    eo_vec_display_field
                        .into_iter()
                        .map(|element| {
                            error_occurence_lib::ToStdStringString::to_std_string_string(&element)
                        })
                        .collect()
                },
                eo_vec_serde,
                eo_vec_error_occurence_field: {
                    eo_vec_error_occurence_field
                        .into_iter()
                        .map(|element| element.into_serialize_deserialize_version())
                        .collect()
                },
                hashmap_string_string: {
                    hashmap_string_string
                        .into_iter()
                        .map(|(key, value)| {
                            (
                                key,
                                error_occurence_lib::ToStdStringString::to_std_string_string(
                                    &value,
                                ),
                            )
                        })
                        .collect()
                },
                hashmap_string_serde,
                hashmap_string_error_occurence: {
                    hashmap_string_error_occurence
                        .into_iter()
                        .map(|(key, value)| (key, value.into_serialize_deserialize_version()))
                        .collect()
                },
                code_occurence: code_occurence,
            },
        }
    }
}
#[derive(Debug, thiserror :: Error, serde :: Serialize, serde :: Deserialize)]
pub enum ErrorNamedOneWithSerializeDeserialize {
    Variant {
        eo_display_field: std::string::String,
        eo_serde: SerializeDeserializeStruct,
        eo_error_occurence_field: ErrorNamedTwoWithSerializeDeserialize,
        eo_vec_display_field: std::vec::Vec<std::string::String>,
        eo_vec_serde: std::vec::Vec<SerializeDeserializeStruct>,
        eo_vec_error_occurence_field: std::vec::Vec<ErrorUnnamedOneWithSerializeDeserialize>,
        hashmap_string_string: std::collections::HashMap<std::string::String, std::string::String>,
        hashmap_string_serde:
            std::collections::HashMap<std::string::String, SerializeDeserializeStruct>,
        hashmap_string_error_occurence:
            std::collections::HashMap<std::string::String, ErrorUnnamedOneWithSerializeDeserialize>,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
impl std::fmt::Display for ErrorNamedOneWithSerializeDeserialize {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!
        (formatter, "{}", error_occurence_lib :: helpers ::
        source_and_code_occurence_formatter(error_occurence_lib ::
        source_to_string_without_config :: SourceToStringWithoutConfig ::
        source_to_string_without_config(self), error_occurence_lib ::
        code_occurence :: Get :: get(self),))
    }
}
impl error_occurence_lib::source_to_string_without_config::SourceToStringWithoutConfig<'_>
    for ErrorNamedOneWithSerializeDeserialize
{
    fn source_to_string_without_config(&self) -> std::string::String {
        match self {
            ErrorNamedOneWithSerializeDeserialize::Variant {
                eo_display_field,
                eo_serde,
                eo_error_occurence_field,
                eo_vec_display_field,
                eo_vec_serde,
                eo_vec_error_occurence_field,
                hashmap_string_string,
                hashmap_string_serde,
                hashmap_string_error_occurence,
                ..
            } => {
                format!
                ("{{
{}{}{}{}{}{}{}{}{}}}", error_occurence_lib ::
                lines_space_backslash :: LinesSpaceBackslash ::
                lines_space_backslash(& format!
                ("eo_display_field: {}", error_occurence_lib ::
                ToStdStringString :: to_std_string_string(eo_display_field))),
                error_occurence_lib :: lines_space_backslash ::
                LinesSpaceBackslash ::
                lines_space_backslash(& format!
                ("eo_serde: {}", error_occurence_lib :: ToStdStringString ::
                to_std_string_string(eo_serde))), error_occurence_lib ::
                lines_space_backslash :: LinesSpaceBackslash ::
                lines_space_backslash(& format!
                ("eo_error_occurence_field: {}", eo_error_occurence_field)),
                error_occurence_lib :: lines_space_backslash ::
                LinesSpaceBackslash ::
                lines_space_backslash(& format!
                ("eo_vec_display_field: {}", error_occurence_lib :: helpers ::
                stringified_lines_error_vec(eo_vec_display_field.iter().fold(std
                :: string :: String :: from(""), | mut acc, element |
                {
                    acc.push_str(& error_occurence_lib :: helpers ::
                    lines_space_backslash_addition(& error_occurence_lib ::
                    ToStdStringString :: to_std_string_string(element))); acc
                })))), error_occurence_lib :: lines_space_backslash ::
                LinesSpaceBackslash ::
                lines_space_backslash(& format!
                ("eo_vec_serde: {}", error_occurence_lib :: helpers ::
                stringified_lines_error_vec(eo_vec_serde.iter().fold(std ::
                string :: String :: from(""), | mut acc, element |
                {
                    acc.push_str(& error_occurence_lib :: helpers ::
                    lines_space_backslash_addition(& error_occurence_lib ::
                    ToStdStringString :: to_std_string_string(element))); acc
                })))), error_occurence_lib :: lines_space_backslash ::
                LinesSpaceBackslash ::
                lines_space_backslash(& format!
                ("eo_vec_error_occurence_field: {}", error_occurence_lib ::
                helpers ::
                stringified_lines_error_vec(eo_vec_error_occurence_field.iter().fold(std
                :: string :: String :: from(""), | mut acc, element |
                {
                    acc.push_str(& error_occurence_lib :: helpers ::
                    lines_space_backslash_addition(& element)); acc
                },)))), error_occurence_lib :: lines_space_backslash ::
                LinesSpaceBackslash ::
                lines_space_backslash(& format!
                ("hashmap_string_string: {}", error_occurence_lib :: helpers
                ::
                error_occurence_hashmap_formatter(hashmap_string_string.iter().fold(std
                :: string :: String :: new(), | mut acc, (key, value) |
                {
                    acc.push_str(& error_occurence_lib :: helpers ::
                    stringified_lines_error_hashmap_element(& key, &
                    error_occurence_lib :: ToStdStringString ::
                    to_std_string_string(value),)); acc
                },)))), error_occurence_lib :: lines_space_backslash ::
                LinesSpaceBackslash ::
                lines_space_backslash(& format!
                ("hashmap_string_serde: {}", error_occurence_lib :: helpers ::
                error_occurence_hashmap_formatter(hashmap_string_serde.iter().fold(std
                :: string :: String :: new(), | mut acc, (key, value) |
                {
                    acc.push_str(& error_occurence_lib :: helpers ::
                    stringified_lines_error_hashmap_element(& key, &
                    error_occurence_lib :: ToStdStringString ::
                    to_std_string_string(value),)); acc
                },)))), error_occurence_lib :: lines_space_backslash ::
                LinesSpaceBackslash ::
                lines_space_backslash(& format!
                ("hashmap_string_error_occurence: {}", error_occurence_lib ::
                helpers ::
                error_occurence_hashmap_formatter(hashmap_string_error_occurence.iter().fold(std
                :: string :: String :: new(), | mut acc, (key, value) |
                {
                    acc.push_str(& error_occurence_lib :: helpers ::
                    stringified_lines_error_hashmap_element(& key, & value,));
                    acc
                },)))))
            }
        }
    }
}
impl error_occurence_lib::code_occurence::Get for ErrorNamedOneWithSerializeDeserialize {
    fn get(&self) -> &error_occurence_lib::code_occurence::CodeOccurence {
        match self {
            ErrorNamedOneWithSerializeDeserialize::Variant { code_occurence, .. } => code_occurence,
        }
    }
}
impl std::fmt::Display for ErrorNamedTwo {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!
        (formatter, "{}", error_occurence_lib :: helpers ::
        source_and_code_occurence_formatter(error_occurence_lib ::
        source_to_string_without_config :: SourceToStringWithoutConfig ::
        source_to_string_without_config(self), error_occurence_lib ::
        code_occurence :: Get :: get(self),))
    }
}
impl error_occurence_lib::source_to_string_with_config::SourceToStringWithConfig<'_>
    for ErrorNamedTwo
{
    fn source_to_string_with_config<
        ConfigGeneric: config_lib::GetSourcePlaceType + config_lib::GetTimezone + ?Sized,
    >(
        &self,
        config: &ConfigGeneric,
    ) -> std::string::String {
        match self {
            ErrorNamedTwo::Variant {
                eo_display_with_serialize_deserialize_field,
                ..
            } => {
                format!
                ("{{
{}}}", error_occurence_lib :: lines_space_backslash ::
                LinesSpaceBackslash ::
                lines_space_backslash(& format!
                ("eo_display_with_serialize_deserialize_field: {}",
                error_occurence_lib :: ToStdStringString ::
                to_std_string_string(eo_display_with_serialize_deserialize_field))))
            }
        }
    }
}
impl error_occurence_lib::source_to_string_without_config::SourceToStringWithoutConfig<'_>
    for ErrorNamedTwo
{
    fn source_to_string_without_config(&self) -> std::string::String {
        match self {
            ErrorNamedTwo::Variant {
                eo_display_with_serialize_deserialize_field,
                ..
            } => {
                format!
                ("{{
{}}}", error_occurence_lib :: lines_space_backslash ::
                LinesSpaceBackslash ::
                lines_space_backslash(& format!
                ("eo_display_with_serialize_deserialize_field: {}",
                error_occurence_lib :: ToStdStringString ::
                to_std_string_string(eo_display_with_serialize_deserialize_field))))
            }
        }
    }
}
impl error_occurence_lib::code_occurence::Get for ErrorNamedTwo {
    fn get(&self) -> &error_occurence_lib::code_occurence::CodeOccurence {
        match self {
            ErrorNamedTwo::Variant { code_occurence, .. } => code_occurence,
        }
    }
}
impl ErrorNamedTwo {
    pub fn into_serialize_deserialize_version(self) -> ErrorNamedTwoWithSerializeDeserialize {
        match self {
            ErrorNamedTwo::Variant {
                eo_display_with_serialize_deserialize_field,
                code_occurence,
            } => ErrorNamedTwoWithSerializeDeserialize::Variant {
                eo_display_with_serialize_deserialize_field,
                code_occurence: code_occurence,
            },
        }
    }
}
#[derive(Debug, thiserror :: Error, serde :: Serialize, serde :: Deserialize)]
pub enum ErrorNamedTwoWithSerializeDeserialize {
    Variant {
        eo_display_with_serialize_deserialize_field: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
impl std::fmt::Display for ErrorNamedTwoWithSerializeDeserialize {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!
        (formatter, "{}", error_occurence_lib :: helpers ::
        source_and_code_occurence_formatter(error_occurence_lib ::
        source_to_string_without_config :: SourceToStringWithoutConfig ::
        source_to_string_without_config(self), error_occurence_lib ::
        code_occurence :: Get :: get(self),))
    }
}
impl error_occurence_lib::source_to_string_without_config::SourceToStringWithoutConfig<'_>
    for ErrorNamedTwoWithSerializeDeserialize
{
    fn source_to_string_without_config(&self) -> std::string::String {
        match self {
            ErrorNamedTwoWithSerializeDeserialize::Variant {
                eo_display_with_serialize_deserialize_field,
                ..
            } => {
                format!
                ("{{
{}}}", error_occurence_lib :: lines_space_backslash ::
                LinesSpaceBackslash ::
                lines_space_backslash(& format!
                ("eo_display_with_serialize_deserialize_field: {}",
                error_occurence_lib :: ToStdStringString ::
                to_std_string_string(eo_display_with_serialize_deserialize_field))))
            }
        }
    }
}
impl error_occurence_lib::code_occurence::Get for ErrorNamedTwoWithSerializeDeserialize {
    fn get(&self) -> &error_occurence_lib::code_occurence::CodeOccurence {
        match self {
            ErrorNamedTwoWithSerializeDeserialize::Variant { code_occurence, .. } => code_occurence,
        }
    }
}
impl std::fmt::Display for ErrorUnnamedOne {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!
        (formatter, "{}", match self
        {
            ErrorUnnamedOne :: Something(value) => error_occurence_lib ::
            helpers ::
            source_and_code_occurence_formatter(error_occurence_lib ::
            source_to_string_without_config :: SourceToStringWithoutConfig ::
            source_to_string_without_config(value), error_occurence_lib ::
            code_occurence :: Get :: get(value),)
        })
    }
}
impl error_occurence_lib::to_string_with_config::ToStringWithConfig<'_> for ErrorUnnamedOne {
    fn to_string_with_config<
        ConfigGeneric: config_lib::GetSourcePlaceType + config_lib::GetTimezone + ?Sized,
    >(
        &self,
        config: &ConfigGeneric,
    ) -> std::string::String {
        match self {
            ErrorUnnamedOne::Something(value) => value.to_string(),
        }
    }
}
impl ErrorUnnamedOne {
    pub fn into_serialize_deserialize_version(self) -> ErrorUnnamedOneWithSerializeDeserialize {
        match self {
            ErrorUnnamedOne::Something(value) => {
                ErrorUnnamedOneWithSerializeDeserialize::Something(
                    value.into_serialize_deserialize_version(),
                )
            }
        }
    }
}
#[derive(Debug, thiserror :: Error, serde :: Serialize, serde :: Deserialize)]
pub enum ErrorUnnamedOneWithSerializeDeserialize {
    Something(ErrorNamedTwoWithSerializeDeserialize),
}
impl std::fmt::Display for ErrorUnnamedOneWithSerializeDeserialize {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!
        (formatter, "{}", match self
        {
            ErrorUnnamedOneWithSerializeDeserialize :: Something(value) =>
            error_occurence_lib :: helpers ::
            source_and_code_occurence_formatter(error_occurence_lib ::
            source_to_string_without_config :: SourceToStringWithoutConfig ::
            source_to_string_without_config(value), error_occurence_lib ::
            code_occurence :: Get :: get(value),)
        })
    }
}
