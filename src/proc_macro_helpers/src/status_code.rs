#[derive(
    Debug,
    strum_macros::Display,
    PartialEq,
    Eq,
    Clone,
    Hash,
    proc_macro_assistants::ToUpperCamelCaseStringified,
    proc_macro_assistants::ToSnakeCaseStringified,
)]
pub enum StatusCode {
    Tvfrr100Continue,
    Tvfrr101SwitchingProtocols,
    Tvfrr102Processing,
    Tvfrr200Ok,
    Tvfrr201Created,
    Tvfrr202Accepted,
    Tvfrr203NonAuthoritativeInformation,
    Tvfrr204NoContent,
    Tvfrr205ResetContent,
    Tvfrr206PartialContent,
    Tvfrr207MultiStatus,
    Tvfrr208AlreadyReported,
    Tvfrr226ImUsed,
    Tvfrr300MultipleChoices,
    Tvfrr301MovedPermanently,
    Tvfrr302Found,
    Tvfrr303SeeOther,
    Tvfrr304NotModified,
    Tvfrr305UseProxy,
    Tvfrr307TemporaryRedirect,
    Tvfrr308PermanentRedirect,
    Tvfrr400BadRequest,
    Tvfrr401Unauthorized,
    Tvfrr402PaymentRequired,
    Tvfrr403Forbidden,
    Tvfrr404NotFound,
    Tvfrr405MethodNotAllowed,
    Tvfrr406NotAcceptable,
    Tvfrr407ProxyAuthenticationRequired,
    Tvfrr408RequestTimeout,
    Tvfrr409Conflict,
    Tvfrr410Gone,
    Tvfrr411LengthRequired,
    Tvfrr412PreconditionFailed,
    Tvfrr413PayloadTooLarge,
    Tvfrr414UriTooLong,
    Tvfrr415UnsupportedMediaType,
    Tvfrr416RangeNotSatisfiable,
    Tvfrr417ExpectationFailed,
    Tvfrr418ImATeapot,
    Tvfrr421MisdirectedRequest,
    Tvfrr422UnprocessableEntity,
    Tvfrr423Locked,
    Tvfrr424FailedDependency,
    Tvfrr426UpgradeRequired,
    Tvfrr428PreconditionRequired,
    Tvfrr429TooManyRequests,
    Tvfrr431RequestHeaderFieldsTooLarge,
    Tvfrr451UnavailableForLegalReasons,
    Tvfrr500InternalServerError,
    Tvfrr501NotImplemented,
    Tvfrr502BadGateway,
    Tvfrr503ServiceUnavailable,
    Tvfrr504GatewayTimeout,
    Tvfrr505HttpVersionNotSupported,
    Tvfrr506VariantAlsoNegotiates,
    Tvfrr507InsufficientStorage,
    Tvfrr508LoopDetected,
    Tvfrr510NotExtended,
    Tvfrr511NetworkAuthenticationRequired,
}

impl StatusCode {
    pub fn to_axum_http_status_code_token_stream(&self) -> proc_macro2::TokenStream {
        match self {
            StatusCode::Tvfrr100Continue => quote::quote! {axum::http::StatusCode::CONTINUE},
            StatusCode::Tvfrr101SwitchingProtocols => {
                quote::quote! {axum::http::StatusCode::SWITCHING_PROTOCOLS}
            }
            StatusCode::Tvfrr102Processing => quote::quote! {axum::http::StatusCode::PROCESSING},
            StatusCode::Tvfrr200Ok => quote::quote! {axum::http::StatusCode::OK},
            StatusCode::Tvfrr201Created => quote::quote! {axum::http::StatusCode::CREATED},
            StatusCode::Tvfrr202Accepted => quote::quote! {axum::http::StatusCode::ACCEPTED},
            StatusCode::Tvfrr203NonAuthoritativeInformation => {
                quote::quote! {axum::http::StatusCode::NON_AUTHORITATIVE_INFORMATION}
            }
            StatusCode::Tvfrr204NoContent => quote::quote! {axum::http::StatusCode::NO_CONTENT},
            StatusCode::Tvfrr205ResetContent => quote::quote! {axum::http::StatusCode::RESET_CONTENT},
            StatusCode::Tvfrr206PartialContent => {
                quote::quote! {axum::http::StatusCode::PARTIAL_CONTENT}
            }
            StatusCode::Tvfrr207MultiStatus => quote::quote! {axum::http::StatusCode::MULTI_STATUS},
            StatusCode::Tvfrr208AlreadyReported => {
                quote::quote! {axum::http::StatusCode::ALREADY_REPORTED}
            }
            StatusCode::Tvfrr226ImUsed => quote::quote! {axum::http::StatusCode::IM_USED},
            StatusCode::Tvfrr300MultipleChoices => {
                quote::quote! {axum::http::StatusCode::MULTIPLE_CHOICES}
            }
            StatusCode::Tvfrr301MovedPermanently => {
                quote::quote! {axum::http::StatusCode::MOVED_PERMANENTLY}
            }
            StatusCode::Tvfrr302Found => quote::quote! {axum::http::StatusCode::FOUND},
            StatusCode::Tvfrr303SeeOther => quote::quote! {axum::http::StatusCode::SEE_OTHER},
            StatusCode::Tvfrr304NotModified => quote::quote! {axum::http::StatusCode::NOT_MODIFIED},
            StatusCode::Tvfrr305UseProxy => quote::quote! {axum::http::StatusCode::USE_PROXY},
            StatusCode::Tvfrr307TemporaryRedirect => {
                quote::quote! {axum::http::StatusCode::TEMPORARY_REDIRECT}
            }
            StatusCode::Tvfrr308PermanentRedirect => {
                quote::quote! {axum::http::StatusCode::PERMANENT_REDIRECT}
            }
            StatusCode::Tvfrr400BadRequest => quote::quote! {axum::http::StatusCode::BAD_REQUEST},
            StatusCode::Tvfrr401Unauthorized => quote::quote! {axum::http::StatusCode::UNAUTHORIZED},
            StatusCode::Tvfrr402PaymentRequired => {
                quote::quote! {axum::http::StatusCode::PAYMENT_REQUIRED}
            }
            StatusCode::Tvfrr403Forbidden => quote::quote! {axum::http::StatusCode::FORBIDDEN},
            StatusCode::Tvfrr404NotFound => quote::quote! {axum::http::StatusCode::NOT_FOUND},
            StatusCode::Tvfrr405MethodNotAllowed => {
                quote::quote! {axum::http::StatusCode::METHOD_NOT_ALLOWED}
            }
            StatusCode::Tvfrr406NotAcceptable => quote::quote! {axum::http::StatusCode::NOT_ACCEPTABLE},
            StatusCode::Tvfrr407ProxyAuthenticationRequired => {
                quote::quote! {axum::http::StatusCode::PROXY_AUTHENTICATION_REQUIRED}
            }
            StatusCode::Tvfrr408RequestTimeout => {
                quote::quote! {axum::http::StatusCode::REQUEST_TIMEOUT}
            }
            StatusCode::Tvfrr409Conflict => quote::quote! {axum::http::StatusCode::CONFLICT},
            StatusCode::Tvfrr410Gone => quote::quote! {axum::http::StatusCode::GONE},
            StatusCode::Tvfrr411LengthRequired => {
                quote::quote! {axum::http::StatusCode::LENGTH_REQUIRED}
            }
            StatusCode::Tvfrr412PreconditionFailed => {
                quote::quote! {axum::http::StatusCode::PRECONDITION_FAILED}
            }
            StatusCode::Tvfrr413PayloadTooLarge => {
                quote::quote! {axum::http::StatusCode::PAYLOAD_TOO_LARGE}
            }
            StatusCode::Tvfrr414UriTooLong => quote::quote! {axum::http::StatusCode::URI_TOO_LONG},
            StatusCode::Tvfrr415UnsupportedMediaType => {
                quote::quote! {axum::http::StatusCode::UNSUPPORTED_MEDIA_TYPE}
            }
            StatusCode::Tvfrr416RangeNotSatisfiable => {
                quote::quote! {axum::http::StatusCode::RANGE_NOT_SATISFIABLE}
            }
            StatusCode::Tvfrr417ExpectationFailed => {
                quote::quote! {axum::http::StatusCode::EXPECTATION_FAILED}
            }
            StatusCode::Tvfrr418ImATeapot => quote::quote! {axum::http::StatusCode::IM_A_TEAPOT},
            StatusCode::Tvfrr421MisdirectedRequest => {
                quote::quote! {axum::http::StatusCode::MISDIRECTED_REQUEST}
            }
            StatusCode::Tvfrr422UnprocessableEntity => {
                quote::quote! {axum::http::StatusCode::UNPROCESSABLE_ENTITY}
            }
            StatusCode::Tvfrr423Locked => quote::quote! {axum::http::StatusCode::LOCKED},
            StatusCode::Tvfrr424FailedDependency => {
                quote::quote! {axum::http::StatusCode::FAILED_DEPENDENCY}
            }
            StatusCode::Tvfrr426UpgradeRequired => {
                quote::quote! {axum::http::StatusCode::UPGRADE_REQUIRED}
            }
            StatusCode::Tvfrr428PreconditionRequired => {
                quote::quote! {axum::http::StatusCode::PRECONDITION_REQUIRED}
            }
            StatusCode::Tvfrr429TooManyRequests => {
                quote::quote! {axum::http::StatusCode::TOO_MANY_REQUESTS}
            }
            StatusCode::Tvfrr431RequestHeaderFieldsTooLarge => {
                quote::quote! {axum::http::StatusCode::REQUEST_HEADER_FIELDS_TOO_LARGE}
            }
            StatusCode::Tvfrr451UnavailableForLegalReasons => {
                quote::quote! {axum::http::StatusCode::UNAVAILABLE_FOR_LEGAL_REASONS}
            }
            StatusCode::Tvfrr500InternalServerError => {
                quote::quote! {axum::http::StatusCode::INTERNAL_SERVER_ERROR}
            }
            StatusCode::Tvfrr501NotImplemented => {
                quote::quote! {axum::http::StatusCode::NOT_IMPLEMENTED}
            }
            StatusCode::Tvfrr502BadGateway => quote::quote! {axum::http::StatusCode::BAD_GATEWAY},
            StatusCode::Tvfrr503ServiceUnavailable => {
                quote::quote! {axum::http::StatusCode::SERVICE_UNAVAILABLE}
            }
            StatusCode::Tvfrr504GatewayTimeout => {
                quote::quote! {axum::http::StatusCode::GATEWAY_TIMEOUT}
            }
            StatusCode::Tvfrr505HttpVersionNotSupported => {
                quote::quote! {axum::http::StatusCode::HTTP_VERSION_NOT_SUPPORTED}
            }
            StatusCode::Tvfrr506VariantAlsoNegotiates => {
                quote::quote! {axum::http::StatusCode::VARIANT_ALSO_NEGOTIATES}
            }
            StatusCode::Tvfrr507InsufficientStorage => {
                quote::quote! {axum::http::StatusCode::INSUFFICIENT_STORAGE}
            }
            StatusCode::Tvfrr508LoopDetected => quote::quote! {axum::http::StatusCode::LOOP_DETECTED},
            StatusCode::Tvfrr510NotExtended => quote::quote! {axum::http::StatusCode::NOT_EXTENDED},
            StatusCode::Tvfrr511NetworkAuthenticationRequired => {
                quote::quote! {axum::http::StatusCode::NETWORK_AUTHENTICATION_REQUIRED}
            }
        }
    }
    pub fn to_http_status_code_token_stream(&self) -> proc_macro2::TokenStream {
        match self {
            StatusCode::Tvfrr100Continue => quote::quote! {http::StatusCode::CONTINUE},
            StatusCode::Tvfrr101SwitchingProtocols => {
                quote::quote! {http::StatusCode::SWITCHING_PROTOCOLS}
            }
            StatusCode::Tvfrr102Processing => quote::quote! {http::StatusCode::PROCESSING},
            StatusCode::Tvfrr200Ok => quote::quote! {http::StatusCode::OK},
            StatusCode::Tvfrr201Created => quote::quote! {http::StatusCode::CREATED},
            StatusCode::Tvfrr202Accepted => quote::quote! {http::StatusCode::ACCEPTED},
            StatusCode::Tvfrr203NonAuthoritativeInformation => {
                quote::quote! {http::StatusCode::NON_AUTHORITATIVE_INFORMATION}
            }
            StatusCode::Tvfrr204NoContent => quote::quote! {http::StatusCode::NO_CONTENT},
            StatusCode::Tvfrr205ResetContent => quote::quote! {http::StatusCode::RESET_CONTENT},
            StatusCode::Tvfrr206PartialContent => {
                quote::quote! {http::StatusCode::PARTIAL_CONTENT}
            }
            StatusCode::Tvfrr207MultiStatus => quote::quote! {http::StatusCode::MULTI_STATUS},
            StatusCode::Tvfrr208AlreadyReported => {
                quote::quote! {http::StatusCode::ALREADY_REPORTED}
            }
            StatusCode::Tvfrr226ImUsed => quote::quote! {http::StatusCode::IM_USED},
            StatusCode::Tvfrr300MultipleChoices => {
                quote::quote! {http::StatusCode::MULTIPLE_CHOICES}
            }
            StatusCode::Tvfrr301MovedPermanently => {
                quote::quote! {http::StatusCode::MOVED_PERMANENTLY}
            }
            StatusCode::Tvfrr302Found => quote::quote! {http::StatusCode::FOUND},
            StatusCode::Tvfrr303SeeOther => quote::quote! {http::StatusCode::SEE_OTHER},
            StatusCode::Tvfrr304NotModified => quote::quote! {http::StatusCode::NOT_MODIFIED},
            StatusCode::Tvfrr305UseProxy => quote::quote! {http::StatusCode::USE_PROXY},
            StatusCode::Tvfrr307TemporaryRedirect => {
                quote::quote! {http::StatusCode::TEMPORARY_REDIRECT}
            }
            StatusCode::Tvfrr308PermanentRedirect => {
                quote::quote! {http::StatusCode::PERMANENT_REDIRECT}
            }
            StatusCode::Tvfrr400BadRequest => quote::quote! {http::StatusCode::BAD_REQUEST},
            StatusCode::Tvfrr401Unauthorized => quote::quote! {http::StatusCode::UNAUTHORIZED},
            StatusCode::Tvfrr402PaymentRequired => {
                quote::quote! {http::StatusCode::PAYMENT_REQUIRED}
            }
            StatusCode::Tvfrr403Forbidden => quote::quote! {http::StatusCode::FORBIDDEN},
            StatusCode::Tvfrr404NotFound => quote::quote! {http::StatusCode::NOT_FOUND},
            StatusCode::Tvfrr405MethodNotAllowed => {
                quote::quote! {http::StatusCode::METHOD_NOT_ALLOWED}
            }
            StatusCode::Tvfrr406NotAcceptable => quote::quote! {http::StatusCode::NOT_ACCEPTABLE},
            StatusCode::Tvfrr407ProxyAuthenticationRequired => {
                quote::quote! {http::StatusCode::PROXY_AUTHENTICATION_REQUIRED}
            }
            StatusCode::Tvfrr408RequestTimeout => {
                quote::quote! {http::StatusCode::REQUEST_TIMEOUT}
            }
            StatusCode::Tvfrr409Conflict => quote::quote! {http::StatusCode::CONFLICT},
            StatusCode::Tvfrr410Gone => quote::quote! {http::StatusCode::GONE},
            StatusCode::Tvfrr411LengthRequired => {
                quote::quote! {http::StatusCode::LENGTH_REQUIRED}
            }
            StatusCode::Tvfrr412PreconditionFailed => {
                quote::quote! {http::StatusCode::PRECONDITION_FAILED}
            }
            StatusCode::Tvfrr413PayloadTooLarge => {
                quote::quote! {http::StatusCode::PAYLOAD_TOO_LARGE}
            }
            StatusCode::Tvfrr414UriTooLong => quote::quote! {http::StatusCode::URI_TOO_LONG},
            StatusCode::Tvfrr415UnsupportedMediaType => {
                quote::quote! {http::StatusCode::UNSUPPORTED_MEDIA_TYPE}
            }
            StatusCode::Tvfrr416RangeNotSatisfiable => {
                quote::quote! {http::StatusCode::RANGE_NOT_SATISFIABLE}
            }
            StatusCode::Tvfrr417ExpectationFailed => {
                quote::quote! {http::StatusCode::EXPECTATION_FAILED}
            }
            StatusCode::Tvfrr418ImATeapot => quote::quote! {http::StatusCode::IM_A_TEAPOT},
            StatusCode::Tvfrr421MisdirectedRequest => {
                quote::quote! {http::StatusCode::MISDIRECTED_REQUEST}
            }
            StatusCode::Tvfrr422UnprocessableEntity => {
                quote::quote! {http::StatusCode::UNPROCESSABLE_ENTITY}
            }
            StatusCode::Tvfrr423Locked => quote::quote! {http::StatusCode::LOCKED},
            StatusCode::Tvfrr424FailedDependency => {
                quote::quote! {http::StatusCode::FAILED_DEPENDENCY}
            }
            StatusCode::Tvfrr426UpgradeRequired => {
                quote::quote! {http::StatusCode::UPGRADE_REQUIRED}
            }
            StatusCode::Tvfrr428PreconditionRequired => {
                quote::quote! {http::StatusCode::PRECONDITION_REQUIRED}
            }
            StatusCode::Tvfrr429TooManyRequests => {
                quote::quote! {http::StatusCode::TOO_MANY_REQUESTS}
            }
            StatusCode::Tvfrr431RequestHeaderFieldsTooLarge => {
                quote::quote! {http::StatusCode::REQUEST_HEADER_FIELDS_TOO_LARGE}
            }
            StatusCode::Tvfrr451UnavailableForLegalReasons => {
                quote::quote! {http::StatusCode::UNAVAILABLE_FOR_LEGAL_REASONS}
            }
            StatusCode::Tvfrr500InternalServerError => {
                quote::quote! {http::StatusCode::INTERNAL_SERVER_ERROR}
            }
            StatusCode::Tvfrr501NotImplemented => {
                quote::quote! {http::StatusCode::NOT_IMPLEMENTED}
            }
            StatusCode::Tvfrr502BadGateway => quote::quote! {http::StatusCode::BAD_GATEWAY},
            StatusCode::Tvfrr503ServiceUnavailable => {
                quote::quote! {http::StatusCode::SERVICE_UNAVAILABLE}
            }
            StatusCode::Tvfrr504GatewayTimeout => {
                quote::quote! {http::StatusCode::GATEWAY_TIMEOUT}
            }
            StatusCode::Tvfrr505HttpVersionNotSupported => {
                quote::quote! {http::StatusCode::HTTP_VERSION_NOT_SUPPORTED}
            }
            StatusCode::Tvfrr506VariantAlsoNegotiates => {
                quote::quote! {http::StatusCode::VARIANT_ALSO_NEGOTIATES}
            }
            StatusCode::Tvfrr507InsufficientStorage => {
                quote::quote! {http::StatusCode::INSUFFICIENT_STORAGE}
            }
            StatusCode::Tvfrr508LoopDetected => quote::quote! {http::StatusCode::LOOP_DETECTED},
            StatusCode::Tvfrr510NotExtended => quote::quote! {http::StatusCode::NOT_EXTENDED},
            StatusCode::Tvfrr511NetworkAuthenticationRequired => {
                quote::quote! {http::StatusCode::NETWORK_AUTHENTICATION_REQUIRED}
            }
        }
    }
    pub fn to_status_code_token_stream(&self) -> proc_macro2::TokenStream {
        match self {
            StatusCode::Tvfrr100Continue => quote::quote! {100},
            StatusCode::Tvfrr101SwitchingProtocols => quote::quote! {101},
            StatusCode::Tvfrr102Processing => quote::quote! {102},
            StatusCode::Tvfrr200Ok => quote::quote! {200},
            StatusCode::Tvfrr201Created => quote::quote! {201},
            StatusCode::Tvfrr202Accepted => quote::quote! {202},
            StatusCode::Tvfrr203NonAuthoritativeInformation => quote::quote! {203},
            StatusCode::Tvfrr204NoContent => quote::quote! {204},
            StatusCode::Tvfrr205ResetContent => quote::quote! {205},
            StatusCode::Tvfrr206PartialContent => quote::quote! {206},
            StatusCode::Tvfrr207MultiStatus => quote::quote! {207},
            StatusCode::Tvfrr208AlreadyReported => quote::quote! {208},
            StatusCode::Tvfrr226ImUsed => quote::quote! {226},
            StatusCode::Tvfrr300MultipleChoices => quote::quote! {300},
            StatusCode::Tvfrr301MovedPermanently => quote::quote! {301},
            StatusCode::Tvfrr302Found => quote::quote! {302},
            StatusCode::Tvfrr303SeeOther => quote::quote! {303},
            StatusCode::Tvfrr304NotModified => quote::quote! {304},
            StatusCode::Tvfrr305UseProxy => quote::quote! {305},
            StatusCode::Tvfrr307TemporaryRedirect => quote::quote! {307},
            StatusCode::Tvfrr308PermanentRedirect => quote::quote! {308},
            StatusCode::Tvfrr400BadRequest => quote::quote! {400},
            StatusCode::Tvfrr401Unauthorized => quote::quote! {401},
            StatusCode::Tvfrr402PaymentRequired => quote::quote! {402},
            StatusCode::Tvfrr403Forbidden => quote::quote! {403},
            StatusCode::Tvfrr404NotFound => quote::quote! {404},
            StatusCode::Tvfrr405MethodNotAllowed => quote::quote! {405},
            StatusCode::Tvfrr406NotAcceptable => quote::quote! {406},
            StatusCode::Tvfrr407ProxyAuthenticationRequired => quote::quote! {407},
            StatusCode::Tvfrr408RequestTimeout => quote::quote! {408},
            StatusCode::Tvfrr409Conflict => quote::quote! {409},
            StatusCode::Tvfrr410Gone => quote::quote! {410},
            StatusCode::Tvfrr411LengthRequired => quote::quote! {411},
            StatusCode::Tvfrr412PreconditionFailed => quote::quote! {412},
            StatusCode::Tvfrr413PayloadTooLarge => quote::quote! {413},
            StatusCode::Tvfrr414UriTooLong => quote::quote! {414},
            StatusCode::Tvfrr415UnsupportedMediaType => quote::quote! {415},
            StatusCode::Tvfrr416RangeNotSatisfiable => quote::quote! {416},
            StatusCode::Tvfrr417ExpectationFailed => quote::quote! {417},
            StatusCode::Tvfrr418ImATeapot => quote::quote! {418},
            StatusCode::Tvfrr421MisdirectedRequest => quote::quote! {421},
            StatusCode::Tvfrr422UnprocessableEntity => quote::quote! {422},
            StatusCode::Tvfrr423Locked => quote::quote! {423},
            StatusCode::Tvfrr424FailedDependency => quote::quote! {424},
            StatusCode::Tvfrr426UpgradeRequired => quote::quote! {426},
            StatusCode::Tvfrr428PreconditionRequired => quote::quote! {428},
            StatusCode::Tvfrr429TooManyRequests => quote::quote! {429},
            StatusCode::Tvfrr431RequestHeaderFieldsTooLarge => quote::quote! {431},
            StatusCode::Tvfrr451UnavailableForLegalReasons => quote::quote! {451},
            StatusCode::Tvfrr500InternalServerError => quote::quote! {500},
            StatusCode::Tvfrr501NotImplemented => quote::quote! {501},
            StatusCode::Tvfrr502BadGateway => quote::quote! {502},
            StatusCode::Tvfrr503ServiceUnavailable => quote::quote! {503},
            StatusCode::Tvfrr504GatewayTimeout => quote::quote! {504},
            StatusCode::Tvfrr505HttpVersionNotSupported => quote::quote! {505},
            StatusCode::Tvfrr506VariantAlsoNegotiates => quote::quote! {506},
            StatusCode::Tvfrr507InsufficientStorage => quote::quote! {507},
            StatusCode::Tvfrr508LoopDetected => quote::quote! {508},
            StatusCode::Tvfrr510NotExtended => quote::quote! {510},
            StatusCode::Tvfrr511NetworkAuthenticationRequired => quote::quote! {511},
        }
    }
    pub fn to_status_code_description_token_stream(&self) -> proc_macro2::TokenStream {
        match self {
            StatusCode::Tvfrr100Continue => quote::quote! {"continue"},
            StatusCode::Tvfrr101SwitchingProtocols => quote::quote! {"switching protocols"},
            StatusCode::Tvfrr102Processing => quote::quote! {"processing"},
            StatusCode::Tvfrr200Ok => quote::quote! {"ok"},
            StatusCode::Tvfrr201Created => quote::quote! {"created"},
            StatusCode::Tvfrr202Accepted => quote::quote! {"accepted"},
            StatusCode::Tvfrr203NonAuthoritativeInformation => quote::quote! {"non authoritative information"},
            StatusCode::Tvfrr204NoContent => quote::quote! {"no content"},
            StatusCode::Tvfrr205ResetContent => quote::quote! {"reset content"},
            StatusCode::Tvfrr206PartialContent => quote::quote! {"partial content"},
            StatusCode::Tvfrr207MultiStatus => quote::quote! {"multi status"},
            StatusCode::Tvfrr208AlreadyReported => quote::quote! {"already reported"},
            StatusCode::Tvfrr226ImUsed => quote::quote! {"im used"},
            StatusCode::Tvfrr300MultipleChoices => quote::quote! {"multiple choices"},
            StatusCode::Tvfrr301MovedPermanently => quote::quote! {"moved permanently"},
            StatusCode::Tvfrr302Found => quote::quote! {"found"},
            StatusCode::Tvfrr303SeeOther => quote::quote! {"see other"},
            StatusCode::Tvfrr304NotModified => quote::quote! {"not modified"},
            StatusCode::Tvfrr305UseProxy => quote::quote! {"use proxy"},
            StatusCode::Tvfrr307TemporaryRedirect => quote::quote! {"temporary redirect"},
            StatusCode::Tvfrr308PermanentRedirect => quote::quote! {"permanent redirect"},
            StatusCode::Tvfrr400BadRequest => quote::quote! {"bad request"},
            StatusCode::Tvfrr401Unauthorized => quote::quote! {"unauthorized"},
            StatusCode::Tvfrr402PaymentRequired => quote::quote! {"payment required"},
            StatusCode::Tvfrr403Forbidden => quote::quote! {"forbidden"},
            StatusCode::Tvfrr404NotFound => quote::quote! {"not found"},
            StatusCode::Tvfrr405MethodNotAllowed => quote::quote! {"method not allowed"},
            StatusCode::Tvfrr406NotAcceptable => quote::quote! {"not acceptable"},
            StatusCode::Tvfrr407ProxyAuthenticationRequired => quote::quote! {"proxy authentication required"},
            StatusCode::Tvfrr408RequestTimeout => quote::quote! {"request timeout"},
            StatusCode::Tvfrr409Conflict => quote::quote! {"conflict"},
            StatusCode::Tvfrr410Gone => quote::quote! {"gone"},
            StatusCode::Tvfrr411LengthRequired => quote::quote! {"length required"},
            StatusCode::Tvfrr412PreconditionFailed => quote::quote! {"precondition failed"},
            StatusCode::Tvfrr413PayloadTooLarge => quote::quote! {"payload too large"},
            StatusCode::Tvfrr414UriTooLong => quote::quote! {"uri too long"},
            StatusCode::Tvfrr415UnsupportedMediaType => quote::quote! {"unsupported media type"},
            StatusCode::Tvfrr416RangeNotSatisfiable => quote::quote! {"range not satisfiable"},
            StatusCode::Tvfrr417ExpectationFailed => quote::quote! {"expectation failed"},
            StatusCode::Tvfrr418ImATeapot => quote::quote! {"im a teapot"},
            StatusCode::Tvfrr421MisdirectedRequest => quote::quote! {"misdirected request"},
            StatusCode::Tvfrr422UnprocessableEntity => quote::quote! {"unprocessable entity"},
            StatusCode::Tvfrr423Locked => quote::quote! {"locked"},
            StatusCode::Tvfrr424FailedDependency => quote::quote! {"failed dependency"},
            StatusCode::Tvfrr426UpgradeRequired => quote::quote! {"upgrade required"},
            StatusCode::Tvfrr428PreconditionRequired => quote::quote! {"precondition required"},
            StatusCode::Tvfrr429TooManyRequests => quote::quote! {"too many requests"},
            StatusCode::Tvfrr431RequestHeaderFieldsTooLarge => quote::quote! {"request header fields too large"},
            StatusCode::Tvfrr451UnavailableForLegalReasons => quote::quote! {"unavailable for legal reasons"},
            StatusCode::Tvfrr500InternalServerError => quote::quote! {"internal server error"},
            StatusCode::Tvfrr501NotImplemented => quote::quote! {"not implemented"},
            StatusCode::Tvfrr502BadGateway => quote::quote! {"bad gateway"},
            StatusCode::Tvfrr503ServiceUnavailable => quote::quote! {"service unavailable"},
            StatusCode::Tvfrr504GatewayTimeout => quote::quote! {"gateway timeout"},
            StatusCode::Tvfrr505HttpVersionNotSupported => quote::quote! {"http version not supported"},
            StatusCode::Tvfrr506VariantAlsoNegotiates => quote::quote! {"variant also negotiates"},
            StatusCode::Tvfrr507InsufficientStorage => quote::quote! {"insufficient storage"},
            StatusCode::Tvfrr508LoopDetected => quote::quote! {"loop detected"},
            StatusCode::Tvfrr510NotExtended => quote::quote! {"not extended"},
            StatusCode::Tvfrr511NetworkAuthenticationRequired => quote::quote! {"network authentication required"},
        }
    }
    pub fn to_proc_macro_attribute_view_token_stream(&self) -> proc_macro2::TokenStream {
        let value_stringified = format!("#[{}]", self.to_string());
        value_stringified.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{value_stringified} {}", proc_macro_common::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    }
}

impl TryFrom<&syn::Variant> for StatusCode {
    type Error = std::string::String;
    fn try_from(value: &syn::Variant) -> Result<Self, Self::Error> {
        let mut option_self: Option<Self> = None;
        for element in &value.attrs {
            if let true = element.path.segments.len() == 1 {
                match element.path.segments.first() {
                    Some(segment) => {
                        if let Ok(value) = Self::try_from(&segment.ident.to_string()) {
                            match option_self {
                                Some(value) => {
                                    return Err(format!("duplicated status_code attributes {value} are not supported"));
                                },
                                None => {
                                    option_self = Some(value);
                                }
                            }
                        }
                    },
                    None => {
                        return Err(std::string::String::from("element.path.segments.first() is None"));
                    }
                }
            }
        }
        match option_self {
            Some(value) => Ok(value),
            None => Err(std::string::String::from("status_code attribute not found")),
        }
    }
}

impl TryFrom<&&syn::Variant> for StatusCode {
    type Error = std::string::String;
    fn try_from(value: &&syn::Variant) -> Result<Self, Self::Error> {
        let mut option_self: Option<Self> = None;
        for element in &value.attrs {
            if let true = element.path.segments.len() == 1 {
                match element.path.segments.first() {
                    Some(segment) => {
                        if let Ok(value) = Self::try_from(&segment.ident.to_string()) {
                            match option_self {
                                Some(value) => {
                                    return Err(format!("duplicated status_code attributes {value} are not supported"));
                                },
                                None => {
                                    option_self = Some(value);
                                }
                            }
                        }
                    },
                    None => {
                        return Err(std::string::String::from("element.path.segments.first() is None"));
                    }
                }
            }
        }
        match option_self {
            Some(value) => Ok(value),
            None => Err(std::string::String::from("status_code attribute not found")),
        }
    }
}

impl TryFrom<&std::string::String> for StatusCode {
    type Error = ();
    fn try_from(value: &std::string::String) -> Result<Self, Self::Error> {
        if value == "tvfrr_100_continue" {
            Ok(StatusCode::Tvfrr100Continue)
        } else if value == "tvfrr_101_switching_protocols" {
            Ok(StatusCode::Tvfrr101SwitchingProtocols)
        } else if value == "tvfrr_102_processing" {
            Ok(StatusCode::Tvfrr102Processing)
        } else if value == "tvfrr_200_ok" {
            Ok(StatusCode::Tvfrr200Ok)
        } else if value == "tvfrr_201_created" {
            Ok(StatusCode::Tvfrr201Created)
        } else if value == "tvfrr_202_accepted" {
            Ok(StatusCode::Tvfrr202Accepted)
        } else if value == "tvfrr_203_non_authoritative_information" {
            Ok(StatusCode::Tvfrr203NonAuthoritativeInformation)
        } else if value == "tvfrr_204_no_content" {
            Ok(StatusCode::Tvfrr204NoContent)
        } else if value == "tvfrr_205_reset_content" {
            Ok(StatusCode::Tvfrr205ResetContent)
        } else if value == "tvfrr_206_partial_content" {
            Ok(StatusCode::Tvfrr206PartialContent)
        } else if value == "tvfrr_207_multi_status" {
            Ok(StatusCode::Tvfrr207MultiStatus)
        } else if value == "tvfrr_208_already_reported" {
            Ok(StatusCode::Tvfrr208AlreadyReported)
        } else if value == "tvfrr_226_im_used" {
            Ok(StatusCode::Tvfrr226ImUsed)
        } else if value == "tvfrr_300_multiple_choices" {
            Ok(StatusCode::Tvfrr300MultipleChoices)
        } else if value == "tvfrr_301_moved_permanently" {
            Ok(StatusCode::Tvfrr301MovedPermanently)
        } else if value == "tvfrr_302_found" {
            Ok(StatusCode::Tvfrr302Found)
        } else if value == "tvfrr_303_see_other" {
            Ok(StatusCode::Tvfrr303SeeOther)
        } else if value == "tvfrr_304_not_modified" {
            Ok(StatusCode::Tvfrr304NotModified)
        } else if value == "tvfrr_305_use_proxy" {
            Ok(StatusCode::Tvfrr305UseProxy)
        } else if value == "tvfrr_307_temporary_redirect" {
            Ok(StatusCode::Tvfrr307TemporaryRedirect)
        } else if value == "tvfrr_308_permanent_redirect" {
            Ok(StatusCode::Tvfrr308PermanentRedirect)
        } else if value == "tvfrr_400_bad_request" {
            Ok(StatusCode::Tvfrr400BadRequest)
        } else if value == "tvfrr_401_unauthorized" {
            Ok(StatusCode::Tvfrr401Unauthorized)
        } else if value == "tvfrr_402_payment_required" {
            Ok(StatusCode::Tvfrr402PaymentRequired)
        } else if value == "tvfrr_403_forbidden" {
            Ok(StatusCode::Tvfrr403Forbidden)
        } else if value == "tvfrr_404_not_found" {
            Ok(StatusCode::Tvfrr404NotFound)
        } else if value == "tvfrr_405_method_not_allowed" {
            Ok(StatusCode::Tvfrr405MethodNotAllowed)
        } else if value == "tvfrr_406_not_acceptable" {
            Ok(StatusCode::Tvfrr406NotAcceptable)
        } else if value == "tvfrr_407_proxy_authentication_required" {
            Ok(StatusCode::Tvfrr407ProxyAuthenticationRequired)
        } else if value == "tvfrr_408_request_timeout" {
            Ok(StatusCode::Tvfrr408RequestTimeout)
        } else if value == "tvfrr_409_conflict" {
            Ok(StatusCode::Tvfrr409Conflict)
        } else if value == "tvfrr_410_gone" {
            Ok(StatusCode::Tvfrr410Gone)
        } else if value == "tvfrr_411_length_required" {
            Ok(StatusCode::Tvfrr411LengthRequired)
        } else if value == "tvfrr_412_precondition_failed" {
            Ok(StatusCode::Tvfrr412PreconditionFailed)
        } else if value == "tvfrr_413_payload_too_large" {
            Ok(StatusCode::Tvfrr413PayloadTooLarge)
        } else if value == "tvfrr_414_uri_too_long" {
            Ok(StatusCode::Tvfrr414UriTooLong)
        } else if value == "tvfrr_415_unsupported_media_type" {
            Ok(StatusCode::Tvfrr415UnsupportedMediaType)
        } else if value == "tvfrr_416_range_not_satisfiable" {
            Ok(StatusCode::Tvfrr416RangeNotSatisfiable)
        } else if value == "tvfrr_417_expectation_failed" {
            Ok(StatusCode::Tvfrr417ExpectationFailed)
        } else if value == "tvfrr_418_im_a_teapot" {
            Ok(StatusCode::Tvfrr418ImATeapot)
        } else if value == "tvfrr_421_misdirected_request" {
            Ok(StatusCode::Tvfrr421MisdirectedRequest)
        } else if value == "tvfrr_422_unprocessable_entity" {
            Ok(StatusCode::Tvfrr422UnprocessableEntity)
        } else if value == "tvfrr_423_locked" {
            Ok(StatusCode::Tvfrr423Locked)
        } else if value == "tvfrr_424_failed_dependency" {
            Ok(StatusCode::Tvfrr424FailedDependency)
        } else if value == "tvfrr_426_upgrade_required" {
            Ok(StatusCode::Tvfrr426UpgradeRequired)
        } else if value == "tvfrr_428_precondition_required" {
            Ok(StatusCode::Tvfrr428PreconditionRequired)
        } else if value == "tvfrr_429_too_many_requests" {
            Ok(StatusCode::Tvfrr429TooManyRequests)
        } else if value == "tvfrr_431_request_header_fields_too_large" {
            Ok(StatusCode::Tvfrr431RequestHeaderFieldsTooLarge)
        } else if value == "tvfrr_451_unavailable_for_legal_reasons" {
            Ok(StatusCode::Tvfrr451UnavailableForLegalReasons)
        } else if value == "tvfrr_500_internal_server_error" {
            Ok(StatusCode::Tvfrr500InternalServerError)
        } else if value == "tvfrr_501_not_implemented" {
            Ok(StatusCode::Tvfrr501NotImplemented)
        } else if value == "tvfrr_502_bad_gateway" {
            Ok(StatusCode::Tvfrr502BadGateway)
        } else if value == "tvfrr_503_service_unavailable" {
            Ok(StatusCode::Tvfrr503ServiceUnavailable)
        } else if value == "tvfrr_504_gateway_timeout" {
            Ok(StatusCode::Tvfrr504GatewayTimeout)
        } else if value == "tvfrr_505_http_version_not_supported" {
            Ok(StatusCode::Tvfrr505HttpVersionNotSupported)
        } else if value == "tvfrr_506_variant_also_negotiates" {
            Ok(StatusCode::Tvfrr506VariantAlsoNegotiates)
        } else if value == "tvfrr_507_insufficient_storage" {
            Ok(StatusCode::Tvfrr507InsufficientStorage)
        } else if value == "tvfrr_508_loop_detected" {
            Ok(StatusCode::Tvfrr508LoopDetected)
        } else if value == "tvfrr_510_not_extended" {
            Ok(StatusCode::Tvfrr510NotExtended)
        } else if value == "tvfrr_511_network_authentication_required" {
            Ok(StatusCode::Tvfrr511NetworkAuthenticationRequired)
        } else {
            Err(())
        }
    }
}

pub fn get_only_one_status_code(
    variant: &syn::Variant,
    proc_macro_name_ident_stringified: &std::string::String
) -> StatusCode {
    let mut option_self = None;
    variant.attrs.iter().for_each(|attr| {
        if let true = attr.path.segments.len() == 1 {
            if let Ok(named_attribute) = StatusCode::try_from(&attr.path.segments[0].ident.to_string()) {
                if let true = option_self.is_some() {
                    panic!("{proc_macro_name_ident_stringified} duplicated status_code attributes are not supported");
                } else {
                    option_self = Some(named_attribute);
                }
            }
        }
    });
    if let Some(attr) = option_self {
        attr
    } else {
        panic!("{proc_macro_name_ident_stringified} not supported status_code attribute");
    }
}