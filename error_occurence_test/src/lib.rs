#[derive(Debug, thiserror::Error, 
    // error_occurence_lib::ErrorOccurence
)]
pub enum Something {
    One {
        // #[eo_display_with_serialize_deserialize]
        one: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}

impl error_occurence_lib::source_to_string_with_config::SourceToStringWithConfig<'_> for Something {
    fn source_to_string_with_config<
        ConfigGeneric: config_lib::GetSourcePlaceType + config_lib::GetTimezone + ?Sized,
    >(
        &self,
        config: &ConfigGeneric,
    ) -> std::string::String {
        match self {
            Something::One {
                one,
                code_occurence: _unused_argument_1,
            } => {
                format!
                ("{{
{}}}",
                error_occurence_lib::lines_space_backslash::LinesSpaceBackslash
                :: lines_space_backslash(& format! ("one: {}", one)))
            }
        }
    }
}
impl error_occurence_lib::source_to_string_without_config::SourceToStringWithoutConfig<'_>
    for Something
{
    fn source_to_string_without_config(&self) -> std::string::String {
        match self {
            Something::One {
                one,
                code_occurence: _unused_argument_1,
            } => {
                format!
                ("{{
{}}}",
                error_occurence_lib::lines_space_backslash::LinesSpaceBackslash
                :: lines_space_backslash(& format! ("one: {}", one)))
            }
        }
    }
}
impl error_occurence_lib::code_occurence::Get for Something {
    fn get(&self) -> &error_occurence_lib::code_occurence::CodeOccurence {
        match self {
            Something::One {
                one: _unused_argument_0,
                code_occurence,
            } => code_occurence,
        }
    }
}
impl error_occurence_lib::source_to_string_without_config::SourceToStringWithoutConfig<'_>
    for SomethingWithSerializeDeserialize
{
    fn source_to_string_without_config(&self) -> std::string::String {
        match self {
            SomethingWithSerializeDeserialize::One {
                one,
                code_occurence: _unused_argument_1,
            } => {
                format!
                ("{{
{}}}",
                error_occurence_lib::lines_space_backslash::LinesSpaceBackslash
                :: lines_space_backslash(& format! ("one: {}", one)))
            }
        }
    }
}
impl error_occurence_lib::code_occurence::Get for SomethingWithSerializeDeserialize {
    fn get(&self) -> &error_occurence_lib::code_occurence::CodeOccurence {
        match self {
            SomethingWithSerializeDeserialize::One {
                one: _unused_argument_0,
                code_occurence,
            } => code_occurence,
        }
    }
}
impl Something {
    pub fn into_serialize_deserialize_version(self) -> SomethingWithSerializeDeserialize {
        match self {
            Something::One {
                one,
                code_occurence,
            } => SomethingWithSerializeDeserialize::One {
                one: { one },
                code_occurence: code_occurence,
            },
        }
    }
}
impl std::fmt::Display for Something {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!
        (formatter, "{}",
        error_occurence_lib::to_string_without_config::ToStringWithoutConfig
        :: to_string_without_config(self))
    }
}
impl std::fmt::Display for SomethingWithSerializeDeserialize {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!
        (formatter, "{}",
        error_occurence_lib::to_string_without_config::ToStringWithoutConfig
        :: to_string_without_config(self))
    }
}
impl error_occurence_lib::error_occurence_named::ErrorOccurenceNamed for Something {
    fn error_occurence_named(&self) {}
}
#[derive(Debug, thiserror :: Error, serde :: Serialize, serde :: Deserialize)]
pub enum SomethingWithSerializeDeserialize {
    One {
        one: std::string::String,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}
