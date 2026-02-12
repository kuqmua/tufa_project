use naming::{
    AsRefStrEnumWithUnitFieldsToScStringified, AsRefStrEnumWithUnitFieldsToUccStringified,
};
use quote::quote;
use strum_macros::Display;
#[allow(clippy::arbitrary_source_item_ordering)]
#[derive(
    Debug,
    Display,
    PartialEq,
    Eq,
    Clone,
    Copy,
    Hash,
    AsRefStrEnumWithUnitFieldsToUccStringified,
    AsRefStrEnumWithUnitFieldsToScStringified,
)]
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
    #[must_use]
    pub fn to_axum_http_status_code_ts(&self) -> proc_macro2::TokenStream {
        match *self {
            Self::Continue100 => quote! {http::StatusCode::CONTINUE},
            Self::SwitchingProtocols101 => {
                quote! {http::StatusCode::SWITCHING_PROTOCOLS}
            }
            Self::Processing102 => quote! {http::StatusCode::PROCESSING},
            Self::Ok200 => quote! {http::StatusCode::OK},
            Self::Created201 => quote! {http::StatusCode::CREATED},
            Self::Accepted202 => quote! {http::StatusCode::ACCEPTED},
            Self::NonAuthoritativeInformation203 => {
                quote! {http::StatusCode::NON_AUTHORITATIVE_INFORMATION}
            }
            Self::NoContent204 => quote! {http::StatusCode::NO_CONTENT},
            Self::ResetContent205 => {
                quote! {http::StatusCode::RESET_CONTENT}
            }
            Self::PartialContent206 => {
                quote! {http::StatusCode::PARTIAL_CONTENT}
            }
            Self::MultiStatus207 => quote! {http::StatusCode::MULTI_STATUS},
            Self::AlreadyReported208 => {
                quote! {http::StatusCode::ALREADY_REPORTED}
            }
            Self::ImUsed226 => quote! {http::StatusCode::IM_USED},
            Self::MultipleChoices300 => {
                quote! {http::StatusCode::MULTIPLE_CHOICES}
            }
            Self::MovedPermanently301 => {
                quote! {http::StatusCode::MOVED_PERMANENTLY}
            }
            Self::Found302 => quote! {http::StatusCode::FOUND},
            Self::SeeOther303 => quote! {http::StatusCode::SEE_OTHER},
            Self::NotModified304 => quote! {http::StatusCode::NOT_MODIFIED},
            Self::UseProxy305 => quote! {http::StatusCode::USE_PROXY},
            Self::TemporaryRedirect307 => {
                quote! {http::StatusCode::TEMPORARY_REDIRECT}
            }
            Self::PermanentRedirect308 => {
                quote! {http::StatusCode::PERMANENT_REDIRECT}
            }
            Self::BadRequest400 => quote! {http::StatusCode::BAD_REQUEST},
            Self::Unauthorized401 => {
                quote! {http::StatusCode::UNAUTHORIZED}
            }
            Self::PaymentRequired402 => {
                quote! {http::StatusCode::PAYMENT_REQUIRED}
            }
            Self::Forbidden403 => quote! {http::StatusCode::FORBIDDEN},
            Self::NotFound404 => quote! {http::StatusCode::NOT_FOUND},
            Self::MethodNotAllowed405 => {
                quote! {http::StatusCode::METHOD_NOT_ALLOWED}
            }
            Self::NotAcceptable406 => {
                quote! {http::StatusCode::NOT_ACCEPTABLE}
            }
            Self::ProxyAuthenticationRequired407 => {
                quote! {http::StatusCode::PROXY_AUTHENTICATION_REQUIRED}
            }
            Self::RequestTimeout408 => {
                quote! {http::StatusCode::REQUEST_TIMEOUT}
            }
            Self::Conflict409 => quote! {http::StatusCode::CONFLICT},
            Self::Gone410 => quote! {http::StatusCode::GONE},
            Self::LengthRequired411 => {
                quote! {http::StatusCode::LENGTH_REQUIRED}
            }
            Self::PreconditionFailed412 => {
                quote! {http::StatusCode::PRECONDITION_FAILED}
            }
            Self::PayloadTooLarge413 => {
                quote! {http::StatusCode::PAYLOAD_TOO_LARGE}
            }
            Self::UriTooLong414 => quote! {http::StatusCode::URI_TOO_LONG},
            Self::UnsupportedMediaType415 => {
                quote! {http::StatusCode::UNSUPPORTED_MEDIA_TYPE}
            }
            Self::RangeNotSatisfiable416 => {
                quote! {http::StatusCode::RANGE_NOT_SATISFIABLE}
            }
            Self::ExpectationFailed417 => {
                quote! {http::StatusCode::EXPECTATION_FAILED}
            }
            Self::ImATeapot418 => quote! {http::StatusCode::IM_A_TEAPOT},
            Self::MisdirectedRequest421 => {
                quote! {http::StatusCode::MISDIRECTED_REQUEST}
            }
            Self::UnprocessableEntity422 => {
                quote! {http::StatusCode::UNPROCESSABLE_ENTITY}
            }
            Self::Locked423 => quote! {http::StatusCode::LOCKED},
            Self::FailedDependency424 => {
                quote! {http::StatusCode::FAILED_DEPENDENCY}
            }
            Self::UpgradeRequired426 => {
                quote! {http::StatusCode::UPGRADE_REQUIRED}
            }
            Self::PreconditionRequired428 => {
                quote! {http::StatusCode::PRECONDITION_REQUIRED}
            }
            Self::TooManyRequests429 => {
                quote! {http::StatusCode::TOO_MANY_REQUESTS}
            }
            Self::RequestHeaderFieldsTooLarge431 => {
                quote! {http::StatusCode::REQUEST_HEADER_FIELDS_TOO_LARGE}
            }
            Self::UnavailableForLegalReasons451 => {
                quote! {http::StatusCode::UNAVAILABLE_FOR_LEGAL_REASONS}
            }
            Self::InternalServerError500 => {
                quote! {http::StatusCode::INTERNAL_SERVER_ERROR}
            }
            Self::NotImplemented501 => {
                quote! {http::StatusCode::NOT_IMPLEMENTED}
            }
            Self::BadGateway502 => quote! {http::StatusCode::BAD_GATEWAY},
            Self::ServiceUnavailable503 => {
                quote! {http::StatusCode::SERVICE_UNAVAILABLE}
            }
            Self::GatewayTimeout504 => {
                quote! {http::StatusCode::GATEWAY_TIMEOUT}
            }
            Self::HttpVersionNotSupported505 => {
                quote! {http::StatusCode::HTTP_VERSION_NOT_SUPPORTED}
            }
            Self::VariantAlsoNegotiates506 => {
                quote! {http::StatusCode::VARIANT_ALSO_NEGOTIATES}
            }
            Self::InsufficientStorage507 => {
                quote! {http::StatusCode::INSUFFICIENT_STORAGE}
            }
            Self::LoopDetected508 => {
                quote! {http::StatusCode::LOOP_DETECTED}
            }
            Self::NotExtended510 => quote! {http::StatusCode::NOT_EXTENDED},
            Self::NetworkAuthenticationRequired511 => {
                quote! {http::StatusCode::NETWORK_AUTHENTICATION_REQUIRED}
            }
        }
    }
    #[must_use]
    pub fn to_http_status_code_ts(&self) -> proc_macro2::TokenStream {
        match *self {
            Self::Continue100 => quote! {http::StatusCode::CONTINUE},
            Self::SwitchingProtocols101 => {
                quote! {http::StatusCode::SWITCHING_PROTOCOLS}
            }
            Self::Processing102 => quote! {http::StatusCode::PROCESSING},
            Self::Ok200 => quote! {http::StatusCode::OK},
            Self::Created201 => quote! {http::StatusCode::CREATED},
            Self::Accepted202 => quote! {http::StatusCode::ACCEPTED},
            Self::NonAuthoritativeInformation203 => {
                quote! {http::StatusCode::NON_AUTHORITATIVE_INFORMATION}
            }
            Self::NoContent204 => quote! {http::StatusCode::NO_CONTENT},
            Self::ResetContent205 => quote! {http::StatusCode::RESET_CONTENT},
            Self::PartialContent206 => {
                quote! {http::StatusCode::PARTIAL_CONTENT}
            }
            Self::MultiStatus207 => quote! {http::StatusCode::MULTI_STATUS},
            Self::AlreadyReported208 => {
                quote! {http::StatusCode::ALREADY_REPORTED}
            }
            Self::ImUsed226 => quote! {http::StatusCode::IM_USED},
            Self::MultipleChoices300 => {
                quote! {http::StatusCode::MULTIPLE_CHOICES}
            }
            Self::MovedPermanently301 => {
                quote! {http::StatusCode::MOVED_PERMANENTLY}
            }
            Self::Found302 => quote! {http::StatusCode::FOUND},
            Self::SeeOther303 => quote! {http::StatusCode::SEE_OTHER},
            Self::NotModified304 => quote! {http::StatusCode::NOT_MODIFIED},
            Self::UseProxy305 => quote! {http::StatusCode::USE_PROXY},
            Self::TemporaryRedirect307 => {
                quote! {http::StatusCode::TEMPORARY_REDIRECT}
            }
            Self::PermanentRedirect308 => {
                quote! {http::StatusCode::PERMANENT_REDIRECT}
            }
            Self::BadRequest400 => quote! {http::StatusCode::BAD_REQUEST},
            Self::Unauthorized401 => quote! {http::StatusCode::UNAUTHORIZED},
            Self::PaymentRequired402 => {
                quote! {http::StatusCode::PAYMENT_REQUIRED}
            }
            Self::Forbidden403 => quote! {http::StatusCode::FORBIDDEN},
            Self::NotFound404 => quote! {http::StatusCode::NOT_FOUND},
            Self::MethodNotAllowed405 => {
                quote! {http::StatusCode::METHOD_NOT_ALLOWED}
            }
            Self::NotAcceptable406 => quote! {http::StatusCode::NOT_ACCEPTABLE},
            Self::ProxyAuthenticationRequired407 => {
                quote! {http::StatusCode::PROXY_AUTHENTICATION_REQUIRED}
            }
            Self::RequestTimeout408 => {
                quote! {http::StatusCode::REQUEST_TIMEOUT}
            }
            Self::Conflict409 => quote! {http::StatusCode::CONFLICT},
            Self::Gone410 => quote! {http::StatusCode::GONE},
            Self::LengthRequired411 => {
                quote! {http::StatusCode::LENGTH_REQUIRED}
            }
            Self::PreconditionFailed412 => {
                quote! {http::StatusCode::PRECONDITION_FAILED}
            }
            Self::PayloadTooLarge413 => {
                quote! {http::StatusCode::PAYLOAD_TOO_LARGE}
            }
            Self::UriTooLong414 => quote! {http::StatusCode::URI_TOO_LONG},
            Self::UnsupportedMediaType415 => {
                quote! {http::StatusCode::UNSUPPORTED_MEDIA_TYPE}
            }
            Self::RangeNotSatisfiable416 => {
                quote! {http::StatusCode::RANGE_NOT_SATISFIABLE}
            }
            Self::ExpectationFailed417 => {
                quote! {http::StatusCode::EXPECTATION_FAILED}
            }
            Self::ImATeapot418 => quote! {http::StatusCode::IM_A_TEAPOT},
            Self::MisdirectedRequest421 => {
                quote! {http::StatusCode::MISDIRECTED_REQUEST}
            }
            Self::UnprocessableEntity422 => {
                quote! {http::StatusCode::UNPROCESSABLE_ENTITY}
            }
            Self::Locked423 => quote! {http::StatusCode::LOCKED},
            Self::FailedDependency424 => {
                quote! {http::StatusCode::FAILED_DEPENDENCY}
            }
            Self::UpgradeRequired426 => {
                quote! {http::StatusCode::UPGRADE_REQUIRED}
            }
            Self::PreconditionRequired428 => {
                quote! {http::StatusCode::PRECONDITION_REQUIRED}
            }
            Self::TooManyRequests429 => {
                quote! {http::StatusCode::TOO_MANY_REQUESTS}
            }
            Self::RequestHeaderFieldsTooLarge431 => {
                quote! {http::StatusCode::REQUEST_HEADER_FIELDS_TOO_LARGE}
            }
            Self::UnavailableForLegalReasons451 => {
                quote! {http::StatusCode::UNAVAILABLE_FOR_LEGAL_REASONS}
            }
            Self::InternalServerError500 => {
                quote! {http::StatusCode::INTERNAL_SERVER_ERROR}
            }
            Self::NotImplemented501 => {
                quote! {http::StatusCode::NOT_IMPLEMENTED}
            }
            Self::BadGateway502 => quote! {http::StatusCode::BAD_GATEWAY},
            Self::ServiceUnavailable503 => {
                quote! {http::StatusCode::SERVICE_UNAVAILABLE}
            }
            Self::GatewayTimeout504 => {
                quote! {http::StatusCode::GATEWAY_TIMEOUT}
            }
            Self::HttpVersionNotSupported505 => {
                quote! {http::StatusCode::HTTP_VERSION_NOT_SUPPORTED}
            }
            Self::VariantAlsoNegotiates506 => {
                quote! {http::StatusCode::VARIANT_ALSO_NEGOTIATES}
            }
            Self::InsufficientStorage507 => {
                quote! {http::StatusCode::INSUFFICIENT_STORAGE}
            }
            Self::LoopDetected508 => quote! {http::StatusCode::LOOP_DETECTED},
            Self::NotExtended510 => quote! {http::StatusCode::NOT_EXTENDED},
            Self::NetworkAuthenticationRequired511 => {
                quote! {http::StatusCode::NETWORK_AUTHENTICATION_REQUIRED}
            }
        }
    }
    #[must_use]
    pub fn to_proc_macro_attribute_view_ts(&self) -> proc_macro2::TokenStream {
        let value = format!("#[{self}]");
        value
            .parse::<proc_macro2::TokenStream>()
            .expect("48ab5b45-d715-4872-a5e3-aa17c1012898")
    }
    #[must_use]
    pub fn to_status_code_description_ts(&self) -> proc_macro2::TokenStream {
        match *self {
            Self::Continue100 => quote! {"continue"},
            Self::SwitchingProtocols101 => quote! {"switching protocols"},
            Self::Processing102 => quote! {"processing"},
            Self::Ok200 => quote! {"ok"},
            Self::Created201 => quote! {"created"},
            Self::Accepted202 => quote! {"accepted"},
            Self::NonAuthoritativeInformation203 => {
                quote! {"non authoritative information"}
            }
            Self::NoContent204 => quote! {"no content"},
            Self::ResetContent205 => quote! {"reset content"},
            Self::PartialContent206 => quote! {"partial content"},
            Self::MultiStatus207 => quote! {"multi status"},
            Self::AlreadyReported208 => quote! {"already reported"},
            Self::ImUsed226 => quote! {"im used"},
            Self::MultipleChoices300 => quote! {"multiple choices"},
            Self::MovedPermanently301 => quote! {"moved permanently"},
            Self::Found302 => quote! {"found"},
            Self::SeeOther303 => quote! {"see other"},
            Self::NotModified304 => quote! {"not modified"},
            Self::UseProxy305 => quote! {"use proxy"},
            Self::TemporaryRedirect307 => quote! {"temporary redirect"},
            Self::PermanentRedirect308 => quote! {"permanent redirect"},
            Self::BadRequest400 => quote! {"bad request"},
            Self::Unauthorized401 => quote! {"unauthorized"},
            Self::PaymentRequired402 => quote! {"payment required"},
            Self::Forbidden403 => quote! {"forbidden"},
            Self::NotFound404 => quote! {"not found"},
            Self::MethodNotAllowed405 => quote! {"method not allowed"},
            Self::NotAcceptable406 => quote! {"not acceptable"},
            Self::ProxyAuthenticationRequired407 => {
                quote! {"proxy authentication required"}
            }
            Self::RequestTimeout408 => quote! {"request timeout"},
            Self::Conflict409 => quote! {"conflict"},
            Self::Gone410 => quote! {"gone"},
            Self::LengthRequired411 => quote! {"length required"},
            Self::PreconditionFailed412 => quote! {"precondition failed"},
            Self::PayloadTooLarge413 => quote! {"payload too large"},
            Self::UriTooLong414 => quote! {"uri too long"},
            Self::UnsupportedMediaType415 => quote! {"unsupported media type"},
            Self::RangeNotSatisfiable416 => quote! {"range not satisfiable"},
            Self::ExpectationFailed417 => quote! {"expectation failed"},
            Self::ImATeapot418 => quote! {"im a teapot"},
            Self::MisdirectedRequest421 => quote! {"misdirected request"},
            Self::UnprocessableEntity422 => quote! {"unprocessable entity"},
            Self::Locked423 => quote! {"locked"},
            Self::FailedDependency424 => quote! {"failed dependency"},
            Self::UpgradeRequired426 => quote! {"upgrade required"},
            Self::PreconditionRequired428 => quote! {"precondition required"},
            Self::TooManyRequests429 => quote! {"too many requests"},
            Self::RequestHeaderFieldsTooLarge431 => {
                quote! {"request header fields too large"}
            }
            Self::UnavailableForLegalReasons451 => {
                quote! {"unavailable for legal reasons"}
            }
            Self::InternalServerError500 => quote! {"internal server error"},
            Self::NotImplemented501 => quote! {"not implemented"},
            Self::BadGateway502 => quote! {"bad gateway"},
            Self::ServiceUnavailable503 => quote! {"service unavailable"},
            Self::GatewayTimeout504 => quote! {"gateway timeout"},
            Self::HttpVersionNotSupported505 => {
                quote! {"http version not supported"}
            }
            Self::VariantAlsoNegotiates506 => quote! {"variant also negotiates"},
            Self::InsufficientStorage507 => quote! {"insufficient storage"},
            Self::LoopDetected508 => quote! {"loop detected"},
            Self::NotExtended510 => quote! {"not extended"},
            Self::NetworkAuthenticationRequired511 => {
                quote! {"network authentication required"}
            }
        }
    }
    #[must_use]
    pub fn to_status_code_ts(&self) -> proc_macro2::TokenStream {
        match *self {
            Self::Continue100 => quote! {100},
            Self::SwitchingProtocols101 => quote! {101},
            Self::Processing102 => quote! {102},
            Self::Ok200 => quote! {200},
            Self::Created201 => quote! {201},
            Self::Accepted202 => quote! {202},
            Self::NonAuthoritativeInformation203 => quote! {203},
            Self::NoContent204 => quote! {204},
            Self::ResetContent205 => quote! {205},
            Self::PartialContent206 => quote! {206},
            Self::MultiStatus207 => quote! {207},
            Self::AlreadyReported208 => quote! {208},
            Self::ImUsed226 => quote! {226},
            Self::MultipleChoices300 => quote! {300},
            Self::MovedPermanently301 => quote! {301},
            Self::Found302 => quote! {302},
            Self::SeeOther303 => quote! {303},
            Self::NotModified304 => quote! {304},
            Self::UseProxy305 => quote! {305},
            Self::TemporaryRedirect307 => quote! {307},
            Self::PermanentRedirect308 => quote! {308},
            Self::BadRequest400 => quote! {400},
            Self::Unauthorized401 => quote! {401},
            Self::PaymentRequired402 => quote! {402},
            Self::Forbidden403 => quote! {403},
            Self::NotFound404 => quote! {404},
            Self::MethodNotAllowed405 => quote! {405},
            Self::NotAcceptable406 => quote! {406},
            Self::ProxyAuthenticationRequired407 => quote! {407},
            Self::RequestTimeout408 => quote! {408},
            Self::Conflict409 => quote! {409},
            Self::Gone410 => quote! {410},
            Self::LengthRequired411 => quote! {411},
            Self::PreconditionFailed412 => quote! {412},
            Self::PayloadTooLarge413 => quote! {413},
            Self::UriTooLong414 => quote! {414},
            Self::UnsupportedMediaType415 => quote! {415},
            Self::RangeNotSatisfiable416 => quote! {416},
            Self::ExpectationFailed417 => quote! {417},
            Self::ImATeapot418 => quote! {418},
            Self::MisdirectedRequest421 => quote! {421},
            Self::UnprocessableEntity422 => quote! {422},
            Self::Locked423 => quote! {423},
            Self::FailedDependency424 => quote! {424},
            Self::UpgradeRequired426 => quote! {426},
            Self::PreconditionRequired428 => quote! {428},
            Self::TooManyRequests429 => quote! {429},
            Self::RequestHeaderFieldsTooLarge431 => quote! {431},
            Self::UnavailableForLegalReasons451 => quote! {451},
            Self::InternalServerError500 => quote! {500},
            Self::NotImplemented501 => quote! {501},
            Self::BadGateway502 => quote! {502},
            Self::ServiceUnavailable503 => quote! {503},
            Self::GatewayTimeout504 => quote! {504},
            Self::HttpVersionNotSupported505 => quote! {505},
            Self::VariantAlsoNegotiates506 => quote! {506},
            Self::InsufficientStorage507 => quote! {507},
            Self::LoopDetected508 => quote! {508},
            Self::NotExtended510 => quote! {510},
            Self::NetworkAuthenticationRequired511 => quote! {511},
        }
    }
}
impl TryFrom<&String> for StatusCode {
    type Error = ();
    fn try_from(value: &String) -> Result<Self, Self::Error> {
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

#[must_use]
pub fn get_only_one(variant: &syn::Variant) -> StatusCode {
    let mut option_self = None;
    variant.attrs.iter().for_each(|attr| {
        if attr.path().segments.len() == 1 {
            let value = attr
                .path()
                .segments
                .first()
                .expect("9deb71d1-ea77-4e80-9f23-b91f9ab70003");
            if let Ok(named_attribute) = StatusCode::try_from(&value.ident.to_string()) {
                if option_self.is_some() {
                    panic!("07286cf0-dfaf-43fa-9c1f-d1a1cc28894e");
                } else {
                    option_self = Some(named_attribute);
                }
            }
        }
    });
    option_self.expect("19fc6512-3877-4498-827d-0bfbc2fb2199")
}
