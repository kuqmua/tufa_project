#[derive(Debug, PartialEq, Eq, Hash, error_occurence::PrimitiveErrorOccurencePartialEqEqHash)]//PartialEq, Eq, Hash, Clone, 
pub struct StdStringString(pub std::string::String);
// #[derive(Debug, error_occurence::PrimitiveErrorOccurence)]//PartialEq, Eq, Hash, Clone, Copy, 
// pub struct StdPrimitiveU8(pub std::primitive::u8);

