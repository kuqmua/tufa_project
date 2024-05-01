#[derive(Debug, PartialEq, Eq, Hash, error_occurence::PrimitiveErrorOccurencePartialEqEqHash)]
pub struct StdStringString(pub std::string::String);
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, error_occurence::PrimitiveErrorOccurencePartialEqEqHashCloneCopy)]
pub struct StdPrimitiveU8(pub std::primitive::u8);

