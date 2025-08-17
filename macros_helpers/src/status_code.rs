#[derive(Debug, strum_macros::Display, PartialEq, Eq, Clone, Copy, Hash, naming::AsRefStrEnumWithUnitFieldsToUpperCamelCaseStringified, naming::AsRefStrEnumWithUnitFieldsToSnakeCaseStringified)]
pub enum StatusCode {
    Continue100,
    SwitchingProtocols101,
    Processing102,
    Ok200,
    Created201,
    Accepted202,
    NonAuthoritativeInformation203,
    NoContent204,
    ResetContent205,
    PartialContent206,
    MultiStatus207,
    AlreadyReported208,
    ImUsed226,
    MultipleChoices300,
    MovedPermanently301,
    Found302,
    SeeOther303,
    NotModified304,
    UseProxy305,
    TemporaryRedirect307,
    PermanentRedirect308,
    BadRequest400,
    Unauthorized401,
    PaymentRequired402,
    Forbidden403,
    NotFound404,
    MethodNotAllowed405,
    NotAcceptable406,
    ProxyAuthenticationRequired407,
    RequestTimeout408,
    Conflict409,
    Gone410,
    LengthRequired411,
    PreconditionFailed412,
    PayloadTooLarge413,
    UriTooLong414,
    UnsupportedMediaType415,
    RangeNotSatisfiable416,
    ExpectationFailed417,
    ImATeapot418,
    MisdirectedRequest421,
    UnprocessableEntity422,
    Locked423,
    FailedDependency424,
    UpgradeRequired426,
    PreconditionRequired428,
    TooManyRequests429,
    RequestHeaderFieldsTooLarge431,
    UnavailableForLegalReasons451,
    InternalServerError500,
    NotImplemented501,
    BadGateway502,
    ServiceUnavailable503,
    GatewayTimeout504,
    HttpVersionNotSupported505,
    VariantAlsoNegotiates506,
    InsufficientStorage507,
    LoopDetected508,
    NotExtended510,
    NetworkAuthenticationRequired511,
}

impl StatusCode {
    pub fn to_axum_http_status_code_token_stream(&self) -> proc_macro2::TokenStream {
        match self {
            Self::Continue100 => quote::quote! {axum::http::StatusCode::CONTINUE},
            Self::SwitchingProtocols101 => {
                quote::quote! {axum::http::StatusCode::SWITCHING_PROTOCOLS}
            }
            Self::Processing102 => quote::quote! {axum::http::StatusCode::PROCESSING},
            Self::Ok200 => quote::quote! {axum::http::StatusCode::OK},
            Self::Created201 => quote::quote! {axum::http::StatusCode::CREATED},
            Self::Accepted202 => quote::quote! {axum::http::StatusCode::ACCEPTED},
            Self::NonAuthoritativeInformation203 => {
                quote::quote! {axum::http::StatusCode::NON_AUTHORITATIVE_INFORMATION}
            }
            Self::NoContent204 => quote::quote! {axum::http::StatusCode::NO_CONTENT},
            Self::ResetContent205 => {
                quote::quote! {axum::http::StatusCode::RESET_CONTENT}
            }
            Self::PartialContent206 => {
                quote::quote! {axum::http::StatusCode::PARTIAL_CONTENT}
            }
            Self::MultiStatus207 => quote::quote! {axum::http::StatusCode::MULTI_STATUS},
            Self::AlreadyReported208 => {
                quote::quote! {axum::http::StatusCode::ALREADY_REPORTED}
            }
            Self::ImUsed226 => quote::quote! {axum::http::StatusCode::IM_USED},
            Self::MultipleChoices300 => {
                quote::quote! {axum::http::StatusCode::MULTIPLE_CHOICES}
            }
            Self::MovedPermanently301 => {
                quote::quote! {axum::http::StatusCode::MOVED_PERMANENTLY}
            }
            Self::Found302 => quote::quote! {axum::http::StatusCode::FOUND},
            Self::SeeOther303 => quote::quote! {axum::http::StatusCode::SEE_OTHER},
            Self::NotModified304 => quote::quote! {axum::http::StatusCode::NOT_MODIFIED},
            Self::UseProxy305 => quote::quote! {axum::http::StatusCode::USE_PROXY},
            Self::TemporaryRedirect307 => {
                quote::quote! {axum::http::StatusCode::TEMPORARY_REDIRECT}
            }
            Self::PermanentRedirect308 => {
                quote::quote! {axum::http::StatusCode::PERMANENT_REDIRECT}
            }
            Self::BadRequest400 => quote::quote! {axum::http::StatusCode::BAD_REQUEST},
            Self::Unauthorized401 => {
                quote::quote! {axum::http::StatusCode::UNAUTHORIZED}
            }
            Self::PaymentRequired402 => {
                quote::quote! {axum::http::StatusCode::PAYMENT_REQUIRED}
            }
            Self::Forbidden403 => quote::quote! {axum::http::StatusCode::FORBIDDEN},
            Self::NotFound404 => quote::quote! {axum::http::StatusCode::NOT_FOUND},
            Self::MethodNotAllowed405 => {
                quote::quote! {axum::http::StatusCode::METHOD_NOT_ALLOWED}
            }
            Self::NotAcceptable406 => {
                quote::quote! {axum::http::StatusCode::NOT_ACCEPTABLE}
            }
            Self::ProxyAuthenticationRequired407 => {
                quote::quote! {axum::http::StatusCode::PROXY_AUTHENTICATION_REQUIRED}
            }
            Self::RequestTimeout408 => {
                quote::quote! {axum::http::StatusCode::REQUEST_TIMEOUT}
            }
            Self::Conflict409 => quote::quote! {axum::http::StatusCode::CONFLICT},
            Self::Gone410 => quote::quote! {axum::http::StatusCode::GONE},
            Self::LengthRequired411 => {
                quote::quote! {axum::http::StatusCode::LENGTH_REQUIRED}
            }
            Self::PreconditionFailed412 => {
                quote::quote! {axum::http::StatusCode::PRECONDITION_FAILED}
            }
            Self::PayloadTooLarge413 => {
                quote::quote! {axum::http::StatusCode::PAYLOAD_TOO_LARGE}
            }
            Self::UriTooLong414 => quote::quote! {axum::http::StatusCode::URI_TOO_LONG},
            Self::UnsupportedMediaType415 => {
                quote::quote! {axum::http::StatusCode::UNSUPPORTED_MEDIA_TYPE}
            }
            Self::RangeNotSatisfiable416 => {
                quote::quote! {axum::http::StatusCode::RANGE_NOT_SATISFIABLE}
            }
            Self::ExpectationFailed417 => {
                quote::quote! {axum::http::StatusCode::EXPECTATION_FAILED}
            }
            Self::ImATeapot418 => quote::quote! {axum::http::StatusCode::IM_A_TEAPOT},
            Self::MisdirectedRequest421 => {
                quote::quote! {axum::http::StatusCode::MISDIRECTED_REQUEST}
            }
            Self::UnprocessableEntity422 => {
                quote::quote! {axum::http::StatusCode::UNPROCESSABLE_ENTITY}
            }
            Self::Locked423 => quote::quote! {axum::http::StatusCode::LOCKED},
            Self::FailedDependency424 => {
                quote::quote! {axum::http::StatusCode::FAILED_DEPENDENCY}
            }
            Self::UpgradeRequired426 => {
                quote::quote! {axum::http::StatusCode::UPGRADE_REQUIRED}
            }
            Self::PreconditionRequired428 => {
                quote::quote! {axum::http::StatusCode::PRECONDITION_REQUIRED}
            }
            Self::TooManyRequests429 => {
                quote::quote! {axum::http::StatusCode::TOO_MANY_REQUESTS}
            }
            Self::RequestHeaderFieldsTooLarge431 => {
                quote::quote! {axum::http::StatusCode::REQUEST_HEADER_FIELDS_TOO_LARGE}
            }
            Self::UnavailableForLegalReasons451 => {
                quote::quote! {axum::http::StatusCode::UNAVAILABLE_FOR_LEGAL_REASONS}
            }
            Self::InternalServerError500 => {
                quote::quote! {axum::http::StatusCode::INTERNAL_SERVER_ERROR}
            }
            Self::NotImplemented501 => {
                quote::quote! {axum::http::StatusCode::NOT_IMPLEMENTED}
            }
            Self::BadGateway502 => quote::quote! {axum::http::StatusCode::BAD_GATEWAY},
            Self::ServiceUnavailable503 => {
                quote::quote! {axum::http::StatusCode::SERVICE_UNAVAILABLE}
            }
            Self::GatewayTimeout504 => {
                quote::quote! {axum::http::StatusCode::GATEWAY_TIMEOUT}
            }
            Self::HttpVersionNotSupported505 => {
                quote::quote! {axum::http::StatusCode::HTTP_VERSION_NOT_SUPPORTED}
            }
            Self::VariantAlsoNegotiates506 => {
                quote::quote! {axum::http::StatusCode::VARIANT_ALSO_NEGOTIATES}
            }
            Self::InsufficientStorage507 => {
                quote::quote! {axum::http::StatusCode::INSUFFICIENT_STORAGE}
            }
            Self::LoopDetected508 => {
                quote::quote! {axum::http::StatusCode::LOOP_DETECTED}
            }
            Self::NotExtended510 => quote::quote! {axum::http::StatusCode::NOT_EXTENDED},
            Self::NetworkAuthenticationRequired511 => {
                quote::quote! {axum::http::StatusCode::NETWORK_AUTHENTICATION_REQUIRED}
            }
        }
    }
    pub fn to_http_status_code_token_stream(&self) -> proc_macro2::TokenStream {
        match self {
            Self::Continue100 => quote::quote! {http::StatusCode::CONTINUE},
            Self::SwitchingProtocols101 => {
                quote::quote! {http::StatusCode::SWITCHING_PROTOCOLS}
            }
            Self::Processing102 => quote::quote! {http::StatusCode::PROCESSING},
            Self::Ok200 => quote::quote! {http::StatusCode::OK},
            Self::Created201 => quote::quote! {http::StatusCode::CREATED},
            Self::Accepted202 => quote::quote! {http::StatusCode::ACCEPTED},
            Self::NonAuthoritativeInformation203 => {
                quote::quote! {http::StatusCode::NON_AUTHORITATIVE_INFORMATION}
            }
            Self::NoContent204 => quote::quote! {http::StatusCode::NO_CONTENT},
            Self::ResetContent205 => quote::quote! {http::StatusCode::RESET_CONTENT},
            Self::PartialContent206 => {
                quote::quote! {http::StatusCode::PARTIAL_CONTENT}
            }
            Self::MultiStatus207 => quote::quote! {http::StatusCode::MULTI_STATUS},
            Self::AlreadyReported208 => {
                quote::quote! {http::StatusCode::ALREADY_REPORTED}
            }
            Self::ImUsed226 => quote::quote! {http::StatusCode::IM_USED},
            Self::MultipleChoices300 => {
                quote::quote! {http::StatusCode::MULTIPLE_CHOICES}
            }
            Self::MovedPermanently301 => {
                quote::quote! {http::StatusCode::MOVED_PERMANENTLY}
            }
            Self::Found302 => quote::quote! {http::StatusCode::FOUND},
            Self::SeeOther303 => quote::quote! {http::StatusCode::SEE_OTHER},
            Self::NotModified304 => quote::quote! {http::StatusCode::NOT_MODIFIED},
            Self::UseProxy305 => quote::quote! {http::StatusCode::USE_PROXY},
            Self::TemporaryRedirect307 => {
                quote::quote! {http::StatusCode::TEMPORARY_REDIRECT}
            }
            Self::PermanentRedirect308 => {
                quote::quote! {http::StatusCode::PERMANENT_REDIRECT}
            }
            Self::BadRequest400 => quote::quote! {http::StatusCode::BAD_REQUEST},
            Self::Unauthorized401 => quote::quote! {http::StatusCode::UNAUTHORIZED},
            Self::PaymentRequired402 => {
                quote::quote! {http::StatusCode::PAYMENT_REQUIRED}
            }
            Self::Forbidden403 => quote::quote! {http::StatusCode::FORBIDDEN},
            Self::NotFound404 => quote::quote! {http::StatusCode::NOT_FOUND},
            Self::MethodNotAllowed405 => {
                quote::quote! {http::StatusCode::METHOD_NOT_ALLOWED}
            }
            Self::NotAcceptable406 => quote::quote! {http::StatusCode::NOT_ACCEPTABLE},
            Self::ProxyAuthenticationRequired407 => {
                quote::quote! {http::StatusCode::PROXY_AUTHENTICATION_REQUIRED}
            }
            Self::RequestTimeout408 => {
                quote::quote! {http::StatusCode::REQUEST_TIMEOUT}
            }
            Self::Conflict409 => quote::quote! {http::StatusCode::CONFLICT},
            Self::Gone410 => quote::quote! {http::StatusCode::GONE},
            Self::LengthRequired411 => {
                quote::quote! {http::StatusCode::LENGTH_REQUIRED}
            }
            Self::PreconditionFailed412 => {
                quote::quote! {http::StatusCode::PRECONDITION_FAILED}
            }
            Self::PayloadTooLarge413 => {
                quote::quote! {http::StatusCode::PAYLOAD_TOO_LARGE}
            }
            Self::UriTooLong414 => quote::quote! {http::StatusCode::URI_TOO_LONG},
            Self::UnsupportedMediaType415 => {
                quote::quote! {http::StatusCode::UNSUPPORTED_MEDIA_TYPE}
            }
            Self::RangeNotSatisfiable416 => {
                quote::quote! {http::StatusCode::RANGE_NOT_SATISFIABLE}
            }
            Self::ExpectationFailed417 => {
                quote::quote! {http::StatusCode::EXPECTATION_FAILED}
            }
            Self::ImATeapot418 => quote::quote! {http::StatusCode::IM_A_TEAPOT},
            Self::MisdirectedRequest421 => {
                quote::quote! {http::StatusCode::MISDIRECTED_REQUEST}
            }
            Self::UnprocessableEntity422 => {
                quote::quote! {http::StatusCode::UNPROCESSABLE_ENTITY}
            }
            Self::Locked423 => quote::quote! {http::StatusCode::LOCKED},
            Self::FailedDependency424 => {
                quote::quote! {http::StatusCode::FAILED_DEPENDENCY}
            }
            Self::UpgradeRequired426 => {
                quote::quote! {http::StatusCode::UPGRADE_REQUIRED}
            }
            Self::PreconditionRequired428 => {
                quote::quote! {http::StatusCode::PRECONDITION_REQUIRED}
            }
            Self::TooManyRequests429 => {
                quote::quote! {http::StatusCode::TOO_MANY_REQUESTS}
            }
            Self::RequestHeaderFieldsTooLarge431 => {
                quote::quote! {http::StatusCode::REQUEST_HEADER_FIELDS_TOO_LARGE}
            }
            Self::UnavailableForLegalReasons451 => {
                quote::quote! {http::StatusCode::UNAVAILABLE_FOR_LEGAL_REASONS}
            }
            Self::InternalServerError500 => {
                quote::quote! {http::StatusCode::INTERNAL_SERVER_ERROR}
            }
            Self::NotImplemented501 => {
                quote::quote! {http::StatusCode::NOT_IMPLEMENTED}
            }
            Self::BadGateway502 => quote::quote! {http::StatusCode::BAD_GATEWAY},
            Self::ServiceUnavailable503 => {
                quote::quote! {http::StatusCode::SERVICE_UNAVAILABLE}
            }
            Self::GatewayTimeout504 => {
                quote::quote! {http::StatusCode::GATEWAY_TIMEOUT}
            }
            Self::HttpVersionNotSupported505 => {
                quote::quote! {http::StatusCode::HTTP_VERSION_NOT_SUPPORTED}
            }
            Self::VariantAlsoNegotiates506 => {
                quote::quote! {http::StatusCode::VARIANT_ALSO_NEGOTIATES}
            }
            Self::InsufficientStorage507 => {
                quote::quote! {http::StatusCode::INSUFFICIENT_STORAGE}
            }
            Self::LoopDetected508 => quote::quote! {http::StatusCode::LOOP_DETECTED},
            Self::NotExtended510 => quote::quote! {http::StatusCode::NOT_EXTENDED},
            Self::NetworkAuthenticationRequired511 => {
                quote::quote! {http::StatusCode::NETWORK_AUTHENTICATION_REQUIRED}
            }
        }
    }
    pub fn to_status_code_token_stream(&self) -> proc_macro2::TokenStream {
        match self {
            Self::Continue100 => quote::quote! {100},
            Self::SwitchingProtocols101 => quote::quote! {101},
            Self::Processing102 => quote::quote! {102},
            Self::Ok200 => quote::quote! {200},
            Self::Created201 => quote::quote! {201},
            Self::Accepted202 => quote::quote! {202},
            Self::NonAuthoritativeInformation203 => quote::quote! {203},
            Self::NoContent204 => quote::quote! {204},
            Self::ResetContent205 => quote::quote! {205},
            Self::PartialContent206 => quote::quote! {206},
            Self::MultiStatus207 => quote::quote! {207},
            Self::AlreadyReported208 => quote::quote! {208},
            Self::ImUsed226 => quote::quote! {226},
            Self::MultipleChoices300 => quote::quote! {300},
            Self::MovedPermanently301 => quote::quote! {301},
            Self::Found302 => quote::quote! {302},
            Self::SeeOther303 => quote::quote! {303},
            Self::NotModified304 => quote::quote! {304},
            Self::UseProxy305 => quote::quote! {305},
            Self::TemporaryRedirect307 => quote::quote! {307},
            Self::PermanentRedirect308 => quote::quote! {308},
            Self::BadRequest400 => quote::quote! {400},
            Self::Unauthorized401 => quote::quote! {401},
            Self::PaymentRequired402 => quote::quote! {402},
            Self::Forbidden403 => quote::quote! {403},
            Self::NotFound404 => quote::quote! {404},
            Self::MethodNotAllowed405 => quote::quote! {405},
            Self::NotAcceptable406 => quote::quote! {406},
            Self::ProxyAuthenticationRequired407 => quote::quote! {407},
            Self::RequestTimeout408 => quote::quote! {408},
            Self::Conflict409 => quote::quote! {409},
            Self::Gone410 => quote::quote! {410},
            Self::LengthRequired411 => quote::quote! {411},
            Self::PreconditionFailed412 => quote::quote! {412},
            Self::PayloadTooLarge413 => quote::quote! {413},
            Self::UriTooLong414 => quote::quote! {414},
            Self::UnsupportedMediaType415 => quote::quote! {415},
            Self::RangeNotSatisfiable416 => quote::quote! {416},
            Self::ExpectationFailed417 => quote::quote! {417},
            Self::ImATeapot418 => quote::quote! {418},
            Self::MisdirectedRequest421 => quote::quote! {421},
            Self::UnprocessableEntity422 => quote::quote! {422},
            Self::Locked423 => quote::quote! {423},
            Self::FailedDependency424 => quote::quote! {424},
            Self::UpgradeRequired426 => quote::quote! {426},
            Self::PreconditionRequired428 => quote::quote! {428},
            Self::TooManyRequests429 => quote::quote! {429},
            Self::RequestHeaderFieldsTooLarge431 => quote::quote! {431},
            Self::UnavailableForLegalReasons451 => quote::quote! {451},
            Self::InternalServerError500 => quote::quote! {500},
            Self::NotImplemented501 => quote::quote! {501},
            Self::BadGateway502 => quote::quote! {502},
            Self::ServiceUnavailable503 => quote::quote! {503},
            Self::GatewayTimeout504 => quote::quote! {504},
            Self::HttpVersionNotSupported505 => quote::quote! {505},
            Self::VariantAlsoNegotiates506 => quote::quote! {506},
            Self::InsufficientStorage507 => quote::quote! {507},
            Self::LoopDetected508 => quote::quote! {508},
            Self::NotExtended510 => quote::quote! {510},
            Self::NetworkAuthenticationRequired511 => quote::quote! {511},
        }
    }
    pub fn to_status_code_description_token_stream(&self) -> proc_macro2::TokenStream {
        match self {
            Self::Continue100 => quote::quote! {"continue"},
            Self::SwitchingProtocols101 => quote::quote! {"switching protocols"},
            Self::Processing102 => quote::quote! {"processing"},
            Self::Ok200 => quote::quote! {"ok"},
            Self::Created201 => quote::quote! {"created"},
            Self::Accepted202 => quote::quote! {"accepted"},
            Self::NonAuthoritativeInformation203 => {
                quote::quote! {"non authoritative information"}
            }
            Self::NoContent204 => quote::quote! {"no content"},
            Self::ResetContent205 => quote::quote! {"reset content"},
            Self::PartialContent206 => quote::quote! {"partial content"},
            Self::MultiStatus207 => quote::quote! {"multi status"},
            Self::AlreadyReported208 => quote::quote! {"already reported"},
            Self::ImUsed226 => quote::quote! {"im used"},
            Self::MultipleChoices300 => quote::quote! {"multiple choices"},
            Self::MovedPermanently301 => quote::quote! {"moved permanently"},
            Self::Found302 => quote::quote! {"found"},
            Self::SeeOther303 => quote::quote! {"see other"},
            Self::NotModified304 => quote::quote! {"not modified"},
            Self::UseProxy305 => quote::quote! {"use proxy"},
            Self::TemporaryRedirect307 => quote::quote! {"temporary redirect"},
            Self::PermanentRedirect308 => quote::quote! {"permanent redirect"},
            Self::BadRequest400 => quote::quote! {"bad request"},
            Self::Unauthorized401 => quote::quote! {"unauthorized"},
            Self::PaymentRequired402 => quote::quote! {"payment required"},
            Self::Forbidden403 => quote::quote! {"forbidden"},
            Self::NotFound404 => quote::quote! {"not found"},
            Self::MethodNotAllowed405 => quote::quote! {"method not allowed"},
            Self::NotAcceptable406 => quote::quote! {"not acceptable"},
            Self::ProxyAuthenticationRequired407 => {
                quote::quote! {"proxy authentication required"}
            }
            Self::RequestTimeout408 => quote::quote! {"request timeout"},
            Self::Conflict409 => quote::quote! {"conflict"},
            Self::Gone410 => quote::quote! {"gone"},
            Self::LengthRequired411 => quote::quote! {"length required"},
            Self::PreconditionFailed412 => quote::quote! {"precondition failed"},
            Self::PayloadTooLarge413 => quote::quote! {"payload too large"},
            Self::UriTooLong414 => quote::quote! {"uri too long"},
            Self::UnsupportedMediaType415 => quote::quote! {"unsupported media type"},
            Self::RangeNotSatisfiable416 => quote::quote! {"range not satisfiable"},
            Self::ExpectationFailed417 => quote::quote! {"expectation failed"},
            Self::ImATeapot418 => quote::quote! {"im a teapot"},
            Self::MisdirectedRequest421 => quote::quote! {"misdirected request"},
            Self::UnprocessableEntity422 => quote::quote! {"unprocessable entity"},
            Self::Locked423 => quote::quote! {"locked"},
            Self::FailedDependency424 => quote::quote! {"failed dependency"},
            Self::UpgradeRequired426 => quote::quote! {"upgrade required"},
            Self::PreconditionRequired428 => quote::quote! {"precondition required"},
            Self::TooManyRequests429 => quote::quote! {"too many requests"},
            Self::RequestHeaderFieldsTooLarge431 => {
                quote::quote! {"request header fields too large"}
            }
            Self::UnavailableForLegalReasons451 => {
                quote::quote! {"unavailable for legal reasons"}
            }
            Self::InternalServerError500 => quote::quote! {"internal server error"},
            Self::NotImplemented501 => quote::quote! {"not implemented"},
            Self::BadGateway502 => quote::quote! {"bad gateway"},
            Self::ServiceUnavailable503 => quote::quote! {"service unavailable"},
            Self::GatewayTimeout504 => quote::quote! {"gateway timeout"},
            Self::HttpVersionNotSupported505 => {
                quote::quote! {"http version not supported"}
            }
            Self::VariantAlsoNegotiates506 => quote::quote! {"variant also negotiates"},
            Self::InsufficientStorage507 => quote::quote! {"insufficient storage"},
            Self::LoopDetected508 => quote::quote! {"loop detected"},
            Self::NotExtended510 => quote::quote! {"not extended"},
            Self::NetworkAuthenticationRequired511 => {
                quote::quote! {"network authentication required"}
            }
        }
    }
    pub fn to_proc_macro_attribute_view_token_stream(&self) -> proc_macro2::TokenStream {
        let value = format!("#[{self}]");
        value.parse::<proc_macro2::TokenStream>().unwrap_or_else(|_| panic!("{value} {}", constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    }
}

// impl TryFrom<&syn::Variant> for StatusCode {
//     type Error = std::string::String;
//     fn try_from(value: &syn::Variant) -> Result<Self, Self::Error> {
//         let mut option_self: Option<Self> = None;
//         for element in &value.attrs {
//             if element.path().segments.len() == 1 {
//                 match element.path().segments.first() {
//                     Some(segment) => {
//                         println!("1{}", &segment.ident.to_string());
//                         if let Ok(value) = Self::try_from(&segment.ident.to_string()) {
//                             match option_self {
//                                 Some(value) => {
//                                     return Err(format!("duplicated status_code attributes {value} are not supported"));
//                                 }
//                                 None => {
//                                     option_self = Some(value);
//                                 }
//                             }
//                         }
//                     }
//                     None => {
//                         return Err(std::string::String::from(
//                             "element.path().segments.first() is None",
//                         ));
//                     }
//                 }
//             }
//         }
//         option_self.map_or_else(|| Err(std::string::String::from("status_code attribute not found")), |value| Ok(value))
//     }
// }

// impl TryFrom<&&syn::Variant> for StatusCode {
//     type Error = std::string::String;
//     fn try_from(value: &&syn::Variant) -> Result<Self, Self::Error> {
//         let mut option_self: Option<Self> = None;
//         for element in &value.attrs {
//             if element.path().segments.len() == 1 {
//                 match element.path().segments.first() {
//                     Some(segment) => {
//                         println!("2{}", &segment.ident.to_string());
//                         if let Ok(value) = Self::try_from(&segment.ident.to_string()) {
//                             match option_self {
//                                 Some(value) => {
//                                     return Err(format!("duplicated status_code attributes {value} are not supported"));
//                                 }
//                                 None => {
//                                     option_self = Some(value);
//                                 }
//                             }
//                         }
//                     }
//                     None => {
//                         return Err(std::string::String::from(
//                             "element.path().segments.first() is None",
//                         ));
//                     }
//                 }
//             }
//         }
//         option_self.map_or_else(|| Err(std::string::String::from("status_code attribute not found")), |value| Ok(value))
//     }
// }

impl TryFrom<&std::string::String> for StatusCode {
    type Error = ();
    fn try_from(value: &std::string::String) -> Result<Self, Self::Error> {
        if value == "continue_100" {
            Ok(Self::Continue100)
        } else if value == "switching_protocols_101" {
            Ok(Self::SwitchingProtocols101)
        } else if value == "processing_102" {
            Ok(Self::Processing102)
        } else if value == "200_ok" {
            Ok(Self::Ok200)
        } else if value == "created_201" {
            Ok(Self::Created201)
        } else if value == "accepted_202" {
            Ok(Self::Accepted202)
        } else if value == "non_authoritative_information_203" {
            Ok(Self::NonAuthoritativeInformation203)
        } else if value == "no_content_204" {
            Ok(Self::NoContent204)
        } else if value == "reset_content_205" {
            Ok(Self::ResetContent205)
        } else if value == "partial_content_206" {
            Ok(Self::PartialContent206)
        } else if value == "multi_status_207" {
            Ok(Self::MultiStatus207)
        } else if value == "already_reported_208" {
            Ok(Self::AlreadyReported208)
        } else if value == "im_used_226" {
            Ok(Self::ImUsed226)
        } else if value == "multiple_choices_300" {
            Ok(Self::MultipleChoices300)
        } else if value == "moved_permanently_301" {
            Ok(Self::MovedPermanently301)
        } else if value == "found_302" {
            Ok(Self::Found302)
        } else if value == "see_other_303" {
            Ok(Self::SeeOther303)
        } else if value == "not_modified_304" {
            Ok(Self::NotModified304)
        } else if value == "use_proxy_305" {
            Ok(Self::UseProxy305)
        } else if value == "temporary_redirect_307" {
            Ok(Self::TemporaryRedirect307)
        } else if value == "permanent_redirect_308" {
            Ok(Self::PermanentRedirect308)
        } else if value == "bad_request_400" {
            Ok(Self::BadRequest400)
        } else if value == "unauthorized_401" {
            Ok(Self::Unauthorized401)
        } else if value == "payment_required_402" {
            Ok(Self::PaymentRequired402)
        } else if value == "forbidden_403" {
            Ok(Self::Forbidden403)
        } else if value == "not_found_404" {
            Ok(Self::NotFound404)
        } else if value == "method_not_allowed_405" {
            Ok(Self::MethodNotAllowed405)
        } else if value == "not_acceptable_406" {
            Ok(Self::NotAcceptable406)
        } else if value == "proxy_authentication_required_407" {
            Ok(Self::ProxyAuthenticationRequired407)
        } else if value == "request_timeout_408" {
            Ok(Self::RequestTimeout408)
        } else if value == "conflict_409" {
            Ok(Self::Conflict409)
        } else if value == "gone_410" {
            Ok(Self::Gone410)
        } else if value == "length_required_411" {
            Ok(Self::LengthRequired411)
        } else if value == "precondition_failed_412" {
            Ok(Self::PreconditionFailed412)
        } else if value == "payload_too_large_413" {
            Ok(Self::PayloadTooLarge413)
        } else if value == "uri_too_long_414" {
            Ok(Self::UriTooLong414)
        } else if value == "unsupported_media_type_415" {
            Ok(Self::UnsupportedMediaType415)
        } else if value == "range_not_satisfiable_416" {
            Ok(Self::RangeNotSatisfiable416)
        } else if value == "expectation_failed_417" {
            Ok(Self::ExpectationFailed417)
        } else if value == "im_a_teapot_418" {
            Ok(Self::ImATeapot418)
        } else if value == "misdirected_request_421" {
            Ok(Self::MisdirectedRequest421)
        } else if value == "unprocessable_entity_422" {
            Ok(Self::UnprocessableEntity422)
        } else if value == "locked_423" {
            Ok(Self::Locked423)
        } else if value == "failed_dependency_424" {
            Ok(Self::FailedDependency424)
        } else if value == "upgrade_required_426" {
            Ok(Self::UpgradeRequired426)
        } else if value == "precondition_required_428" {
            Ok(Self::PreconditionRequired428)
        } else if value == "too_many_requests_429" {
            Ok(Self::TooManyRequests429)
        } else if value == "request_header_fields_too_large_431" {
            Ok(Self::RequestHeaderFieldsTooLarge431)
        } else if value == "unavailable_for_legal_reasons_451" {
            Ok(Self::UnavailableForLegalReasons451)
        } else if value == "internal_server_error_500" {
            Ok(Self::InternalServerError500)
        } else if value == "not_implemented_501" {
            Ok(Self::NotImplemented501)
        } else if value == "bad_gateway_502" {
            Ok(Self::BadGateway502)
        } else if value == "service_unavailable_503" {
            Ok(Self::ServiceUnavailable503)
        } else if value == "gateway_timeout_504" {
            Ok(Self::GatewayTimeout504)
        } else if value == "http_version_not_supported_505" {
            Ok(Self::HttpVersionNotSupported505)
        } else if value == "variant_also_negotiates_506" {
            Ok(Self::VariantAlsoNegotiates506)
        } else if value == "insufficient_storage_507" {
            Ok(Self::InsufficientStorage507)
        } else if value == "loop_detected_508" {
            Ok(Self::LoopDetected508)
        } else if value == "not_extended_510" {
            Ok(Self::NotExtended510)
        } else if value == "network_authentication_required_511" {
            Ok(Self::NetworkAuthenticationRequired511)
        } else {
            Err(())
        }
    }
}

pub fn get_only_one(variant: &syn::Variant, proc_macro_name_ident_stringified: &std::string::String) -> StatusCode {
    let mut option_self = None;
    variant.attrs.iter().for_each(|attr| {
        if attr.path().segments.len() == 1 {
            let value = attr.path().segments.first().map_or_else(|| panic!("{proc_macro_name_ident_stringified} attr.path().segments.get(0) is None"), |value| value);
            if let Ok(named_attribute) = StatusCode::try_from(&value.ident.to_string()) {
                if option_self.is_some() {
                    panic!("{proc_macro_name_ident_stringified} duplicated status_code attributes are not supported");
                } else {
                    option_self = Some(named_attribute);
                }
            }
        }
    });
    option_self.map_or_else(
        || {
            panic!("{proc_macro_name_ident_stringified} not supported status_code attribute");
        },
        |attr| attr,
    )
}
