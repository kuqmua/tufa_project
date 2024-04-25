#[derive(
    Debug,
    strum_macros::Display,
    PartialEq,
    Eq,
    Clone,
    Copy,
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
            Self::Tvfrr100Continue => quote::quote! {axum::http::StatusCode::CONTINUE},
            Self::Tvfrr101SwitchingProtocols => {
                quote::quote! {axum::http::StatusCode::SWITCHING_PROTOCOLS}
            }
            Self::Tvfrr102Processing => quote::quote! {axum::http::StatusCode::PROCESSING},
            Self::Tvfrr200Ok => quote::quote! {axum::http::StatusCode::OK},
            Self::Tvfrr201Created => quote::quote! {axum::http::StatusCode::CREATED},
            Self::Tvfrr202Accepted => quote::quote! {axum::http::StatusCode::ACCEPTED},
            Self::Tvfrr203NonAuthoritativeInformation => {
                quote::quote! {axum::http::StatusCode::NON_AUTHORITATIVE_INFORMATION}
            }
            Self::Tvfrr204NoContent => quote::quote! {axum::http::StatusCode::NO_CONTENT},
            Self::Tvfrr205ResetContent => {
                quote::quote! {axum::http::StatusCode::RESET_CONTENT}
            }
            Self::Tvfrr206PartialContent => {
                quote::quote! {axum::http::StatusCode::PARTIAL_CONTENT}
            }
            Self::Tvfrr207MultiStatus => quote::quote! {axum::http::StatusCode::MULTI_STATUS},
            Self::Tvfrr208AlreadyReported => {
                quote::quote! {axum::http::StatusCode::ALREADY_REPORTED}
            }
            Self::Tvfrr226ImUsed => quote::quote! {axum::http::StatusCode::IM_USED},
            Self::Tvfrr300MultipleChoices => {
                quote::quote! {axum::http::StatusCode::MULTIPLE_CHOICES}
            }
            Self::Tvfrr301MovedPermanently => {
                quote::quote! {axum::http::StatusCode::MOVED_PERMANENTLY}
            }
            Self::Tvfrr302Found => quote::quote! {axum::http::StatusCode::FOUND},
            Self::Tvfrr303SeeOther => quote::quote! {axum::http::StatusCode::SEE_OTHER},
            Self::Tvfrr304NotModified => quote::quote! {axum::http::StatusCode::NOT_MODIFIED},
            Self::Tvfrr305UseProxy => quote::quote! {axum::http::StatusCode::USE_PROXY},
            Self::Tvfrr307TemporaryRedirect => {
                quote::quote! {axum::http::StatusCode::TEMPORARY_REDIRECT}
            }
            Self::Tvfrr308PermanentRedirect => {
                quote::quote! {axum::http::StatusCode::PERMANENT_REDIRECT}
            }
            Self::Tvfrr400BadRequest => quote::quote! {axum::http::StatusCode::BAD_REQUEST},
            Self::Tvfrr401Unauthorized => {
                quote::quote! {axum::http::StatusCode::UNAUTHORIZED}
            }
            Self::Tvfrr402PaymentRequired => {
                quote::quote! {axum::http::StatusCode::PAYMENT_REQUIRED}
            }
            Self::Tvfrr403Forbidden => quote::quote! {axum::http::StatusCode::FORBIDDEN},
            Self::Tvfrr404NotFound => quote::quote! {axum::http::StatusCode::NOT_FOUND},
            Self::Tvfrr405MethodNotAllowed => {
                quote::quote! {axum::http::StatusCode::METHOD_NOT_ALLOWED}
            }
            Self::Tvfrr406NotAcceptable => {
                quote::quote! {axum::http::StatusCode::NOT_ACCEPTABLE}
            }
            Self::Tvfrr407ProxyAuthenticationRequired => {
                quote::quote! {axum::http::StatusCode::PROXY_AUTHENTICATION_REQUIRED}
            }
            Self::Tvfrr408RequestTimeout => {
                quote::quote! {axum::http::StatusCode::REQUEST_TIMEOUT}
            }
            Self::Tvfrr409Conflict => quote::quote! {axum::http::StatusCode::CONFLICT},
            Self::Tvfrr410Gone => quote::quote! {axum::http::StatusCode::GONE},
            Self::Tvfrr411LengthRequired => {
                quote::quote! {axum::http::StatusCode::LENGTH_REQUIRED}
            }
            Self::Tvfrr412PreconditionFailed => {
                quote::quote! {axum::http::StatusCode::PRECONDITION_FAILED}
            }
            Self::Tvfrr413PayloadTooLarge => {
                quote::quote! {axum::http::StatusCode::PAYLOAD_TOO_LARGE}
            }
            Self::Tvfrr414UriTooLong => quote::quote! {axum::http::StatusCode::URI_TOO_LONG},
            Self::Tvfrr415UnsupportedMediaType => {
                quote::quote! {axum::http::StatusCode::UNSUPPORTED_MEDIA_TYPE}
            }
            Self::Tvfrr416RangeNotSatisfiable => {
                quote::quote! {axum::http::StatusCode::RANGE_NOT_SATISFIABLE}
            }
            Self::Tvfrr417ExpectationFailed => {
                quote::quote! {axum::http::StatusCode::EXPECTATION_FAILED}
            }
            Self::Tvfrr418ImATeapot => quote::quote! {axum::http::StatusCode::IM_A_TEAPOT},
            Self::Tvfrr421MisdirectedRequest => {
                quote::quote! {axum::http::StatusCode::MISDIRECTED_REQUEST}
            }
            Self::Tvfrr422UnprocessableEntity => {
                quote::quote! {axum::http::StatusCode::UNPROCESSABLE_ENTITY}
            }
            Self::Tvfrr423Locked => quote::quote! {axum::http::StatusCode::LOCKED},
            Self::Tvfrr424FailedDependency => {
                quote::quote! {axum::http::StatusCode::FAILED_DEPENDENCY}
            }
            Self::Tvfrr426UpgradeRequired => {
                quote::quote! {axum::http::StatusCode::UPGRADE_REQUIRED}
            }
            Self::Tvfrr428PreconditionRequired => {
                quote::quote! {axum::http::StatusCode::PRECONDITION_REQUIRED}
            }
            Self::Tvfrr429TooManyRequests => {
                quote::quote! {axum::http::StatusCode::TOO_MANY_REQUESTS}
            }
            Self::Tvfrr431RequestHeaderFieldsTooLarge => {
                quote::quote! {axum::http::StatusCode::REQUEST_HEADER_FIELDS_TOO_LARGE}
            }
            Self::Tvfrr451UnavailableForLegalReasons => {
                quote::quote! {axum::http::StatusCode::UNAVAILABLE_FOR_LEGAL_REASONS}
            }
            Self::Tvfrr500InternalServerError => {
                quote::quote! {axum::http::StatusCode::INTERNAL_SERVER_ERROR}
            }
            Self::Tvfrr501NotImplemented => {
                quote::quote! {axum::http::StatusCode::NOT_IMPLEMENTED}
            }
            Self::Tvfrr502BadGateway => quote::quote! {axum::http::StatusCode::BAD_GATEWAY},
            Self::Tvfrr503ServiceUnavailable => {
                quote::quote! {axum::http::StatusCode::SERVICE_UNAVAILABLE}
            }
            Self::Tvfrr504GatewayTimeout => {
                quote::quote! {axum::http::StatusCode::GATEWAY_TIMEOUT}
            }
            Self::Tvfrr505HttpVersionNotSupported => {
                quote::quote! {axum::http::StatusCode::HTTP_VERSION_NOT_SUPPORTED}
            }
            Self::Tvfrr506VariantAlsoNegotiates => {
                quote::quote! {axum::http::StatusCode::VARIANT_ALSO_NEGOTIATES}
            }
            Self::Tvfrr507InsufficientStorage => {
                quote::quote! {axum::http::StatusCode::INSUFFICIENT_STORAGE}
            }
            Self::Tvfrr508LoopDetected => {
                quote::quote! {axum::http::StatusCode::LOOP_DETECTED}
            }
            Self::Tvfrr510NotExtended => quote::quote! {axum::http::StatusCode::NOT_EXTENDED},
            Self::Tvfrr511NetworkAuthenticationRequired => {
                quote::quote! {axum::http::StatusCode::NETWORK_AUTHENTICATION_REQUIRED}
            }
        }
    }
    pub fn to_http_status_code_token_stream(&self) -> proc_macro2::TokenStream {
        match self {
            Self::Tvfrr100Continue => quote::quote! {http::StatusCode::CONTINUE},
            Self::Tvfrr101SwitchingProtocols => {
                quote::quote! {http::StatusCode::SWITCHING_PROTOCOLS}
            }
            Self::Tvfrr102Processing => quote::quote! {http::StatusCode::PROCESSING},
            Self::Tvfrr200Ok => quote::quote! {http::StatusCode::OK},
            Self::Tvfrr201Created => quote::quote! {http::StatusCode::CREATED},
            Self::Tvfrr202Accepted => quote::quote! {http::StatusCode::ACCEPTED},
            Self::Tvfrr203NonAuthoritativeInformation => {
                quote::quote! {http::StatusCode::NON_AUTHORITATIVE_INFORMATION}
            }
            Self::Tvfrr204NoContent => quote::quote! {http::StatusCode::NO_CONTENT},
            Self::Tvfrr205ResetContent => quote::quote! {http::StatusCode::RESET_CONTENT},
            Self::Tvfrr206PartialContent => {
                quote::quote! {http::StatusCode::PARTIAL_CONTENT}
            }
            Self::Tvfrr207MultiStatus => quote::quote! {http::StatusCode::MULTI_STATUS},
            Self::Tvfrr208AlreadyReported => {
                quote::quote! {http::StatusCode::ALREADY_REPORTED}
            }
            Self::Tvfrr226ImUsed => quote::quote! {http::StatusCode::IM_USED},
            Self::Tvfrr300MultipleChoices => {
                quote::quote! {http::StatusCode::MULTIPLE_CHOICES}
            }
            Self::Tvfrr301MovedPermanently => {
                quote::quote! {http::StatusCode::MOVED_PERMANENTLY}
            }
            Self::Tvfrr302Found => quote::quote! {http::StatusCode::FOUND},
            Self::Tvfrr303SeeOther => quote::quote! {http::StatusCode::SEE_OTHER},
            Self::Tvfrr304NotModified => quote::quote! {http::StatusCode::NOT_MODIFIED},
            Self::Tvfrr305UseProxy => quote::quote! {http::StatusCode::USE_PROXY},
            Self::Tvfrr307TemporaryRedirect => {
                quote::quote! {http::StatusCode::TEMPORARY_REDIRECT}
            }
            Self::Tvfrr308PermanentRedirect => {
                quote::quote! {http::StatusCode::PERMANENT_REDIRECT}
            }
            Self::Tvfrr400BadRequest => quote::quote! {http::StatusCode::BAD_REQUEST},
            Self::Tvfrr401Unauthorized => quote::quote! {http::StatusCode::UNAUTHORIZED},
            Self::Tvfrr402PaymentRequired => {
                quote::quote! {http::StatusCode::PAYMENT_REQUIRED}
            }
            Self::Tvfrr403Forbidden => quote::quote! {http::StatusCode::FORBIDDEN},
            Self::Tvfrr404NotFound => quote::quote! {http::StatusCode::NOT_FOUND},
            Self::Tvfrr405MethodNotAllowed => {
                quote::quote! {http::StatusCode::METHOD_NOT_ALLOWED}
            }
            Self::Tvfrr406NotAcceptable => quote::quote! {http::StatusCode::NOT_ACCEPTABLE},
            Self::Tvfrr407ProxyAuthenticationRequired => {
                quote::quote! {http::StatusCode::PROXY_AUTHENTICATION_REQUIRED}
            }
            Self::Tvfrr408RequestTimeout => {
                quote::quote! {http::StatusCode::REQUEST_TIMEOUT}
            }
            Self::Tvfrr409Conflict => quote::quote! {http::StatusCode::CONFLICT},
            Self::Tvfrr410Gone => quote::quote! {http::StatusCode::GONE},
            Self::Tvfrr411LengthRequired => {
                quote::quote! {http::StatusCode::LENGTH_REQUIRED}
            }
            Self::Tvfrr412PreconditionFailed => {
                quote::quote! {http::StatusCode::PRECONDITION_FAILED}
            }
            Self::Tvfrr413PayloadTooLarge => {
                quote::quote! {http::StatusCode::PAYLOAD_TOO_LARGE}
            }
            Self::Tvfrr414UriTooLong => quote::quote! {http::StatusCode::URI_TOO_LONG},
            Self::Tvfrr415UnsupportedMediaType => {
                quote::quote! {http::StatusCode::UNSUPPORTED_MEDIA_TYPE}
            }
            Self::Tvfrr416RangeNotSatisfiable => {
                quote::quote! {http::StatusCode::RANGE_NOT_SATISFIABLE}
            }
            Self::Tvfrr417ExpectationFailed => {
                quote::quote! {http::StatusCode::EXPECTATION_FAILED}
            }
            Self::Tvfrr418ImATeapot => quote::quote! {http::StatusCode::IM_A_TEAPOT},
            Self::Tvfrr421MisdirectedRequest => {
                quote::quote! {http::StatusCode::MISDIRECTED_REQUEST}
            }
            Self::Tvfrr422UnprocessableEntity => {
                quote::quote! {http::StatusCode::UNPROCESSABLE_ENTITY}
            }
            Self::Tvfrr423Locked => quote::quote! {http::StatusCode::LOCKED},
            Self::Tvfrr424FailedDependency => {
                quote::quote! {http::StatusCode::FAILED_DEPENDENCY}
            }
            Self::Tvfrr426UpgradeRequired => {
                quote::quote! {http::StatusCode::UPGRADE_REQUIRED}
            }
            Self::Tvfrr428PreconditionRequired => {
                quote::quote! {http::StatusCode::PRECONDITION_REQUIRED}
            }
            Self::Tvfrr429TooManyRequests => {
                quote::quote! {http::StatusCode::TOO_MANY_REQUESTS}
            }
            Self::Tvfrr431RequestHeaderFieldsTooLarge => {
                quote::quote! {http::StatusCode::REQUEST_HEADER_FIELDS_TOO_LARGE}
            }
            Self::Tvfrr451UnavailableForLegalReasons => {
                quote::quote! {http::StatusCode::UNAVAILABLE_FOR_LEGAL_REASONS}
            }
            Self::Tvfrr500InternalServerError => {
                quote::quote! {http::StatusCode::INTERNAL_SERVER_ERROR}
            }
            Self::Tvfrr501NotImplemented => {
                quote::quote! {http::StatusCode::NOT_IMPLEMENTED}
            }
            Self::Tvfrr502BadGateway => quote::quote! {http::StatusCode::BAD_GATEWAY},
            Self::Tvfrr503ServiceUnavailable => {
                quote::quote! {http::StatusCode::SERVICE_UNAVAILABLE}
            }
            Self::Tvfrr504GatewayTimeout => {
                quote::quote! {http::StatusCode::GATEWAY_TIMEOUT}
            }
            Self::Tvfrr505HttpVersionNotSupported => {
                quote::quote! {http::StatusCode::HTTP_VERSION_NOT_SUPPORTED}
            }
            Self::Tvfrr506VariantAlsoNegotiates => {
                quote::quote! {http::StatusCode::VARIANT_ALSO_NEGOTIATES}
            }
            Self::Tvfrr507InsufficientStorage => {
                quote::quote! {http::StatusCode::INSUFFICIENT_STORAGE}
            }
            Self::Tvfrr508LoopDetected => quote::quote! {http::StatusCode::LOOP_DETECTED},
            Self::Tvfrr510NotExtended => quote::quote! {http::StatusCode::NOT_EXTENDED},
            Self::Tvfrr511NetworkAuthenticationRequired => {
                quote::quote! {http::StatusCode::NETWORK_AUTHENTICATION_REQUIRED}
            }
        }
    }
    pub fn to_status_code_token_stream(&self) -> proc_macro2::TokenStream {
        match self {
            Self::Tvfrr100Continue => quote::quote! {100},
            Self::Tvfrr101SwitchingProtocols => quote::quote! {101},
            Self::Tvfrr102Processing => quote::quote! {102},
            Self::Tvfrr200Ok => quote::quote! {200},
            Self::Tvfrr201Created => quote::quote! {201},
            Self::Tvfrr202Accepted => quote::quote! {202},
            Self::Tvfrr203NonAuthoritativeInformation => quote::quote! {203},
            Self::Tvfrr204NoContent => quote::quote! {204},
            Self::Tvfrr205ResetContent => quote::quote! {205},
            Self::Tvfrr206PartialContent => quote::quote! {206},
            Self::Tvfrr207MultiStatus => quote::quote! {207},
            Self::Tvfrr208AlreadyReported => quote::quote! {208},
            Self::Tvfrr226ImUsed => quote::quote! {226},
            Self::Tvfrr300MultipleChoices => quote::quote! {300},
            Self::Tvfrr301MovedPermanently => quote::quote! {301},
            Self::Tvfrr302Found => quote::quote! {302},
            Self::Tvfrr303SeeOther => quote::quote! {303},
            Self::Tvfrr304NotModified => quote::quote! {304},
            Self::Tvfrr305UseProxy => quote::quote! {305},
            Self::Tvfrr307TemporaryRedirect => quote::quote! {307},
            Self::Tvfrr308PermanentRedirect => quote::quote! {308},
            Self::Tvfrr400BadRequest => quote::quote! {400},
            Self::Tvfrr401Unauthorized => quote::quote! {401},
            Self::Tvfrr402PaymentRequired => quote::quote! {402},
            Self::Tvfrr403Forbidden => quote::quote! {403},
            Self::Tvfrr404NotFound => quote::quote! {404},
            Self::Tvfrr405MethodNotAllowed => quote::quote! {405},
            Self::Tvfrr406NotAcceptable => quote::quote! {406},
            Self::Tvfrr407ProxyAuthenticationRequired => quote::quote! {407},
            Self::Tvfrr408RequestTimeout => quote::quote! {408},
            Self::Tvfrr409Conflict => quote::quote! {409},
            Self::Tvfrr410Gone => quote::quote! {410},
            Self::Tvfrr411LengthRequired => quote::quote! {411},
            Self::Tvfrr412PreconditionFailed => quote::quote! {412},
            Self::Tvfrr413PayloadTooLarge => quote::quote! {413},
            Self::Tvfrr414UriTooLong => quote::quote! {414},
            Self::Tvfrr415UnsupportedMediaType => quote::quote! {415},
            Self::Tvfrr416RangeNotSatisfiable => quote::quote! {416},
            Self::Tvfrr417ExpectationFailed => quote::quote! {417},
            Self::Tvfrr418ImATeapot => quote::quote! {418},
            Self::Tvfrr421MisdirectedRequest => quote::quote! {421},
            Self::Tvfrr422UnprocessableEntity => quote::quote! {422},
            Self::Tvfrr423Locked => quote::quote! {423},
            Self::Tvfrr424FailedDependency => quote::quote! {424},
            Self::Tvfrr426UpgradeRequired => quote::quote! {426},
            Self::Tvfrr428PreconditionRequired => quote::quote! {428},
            Self::Tvfrr429TooManyRequests => quote::quote! {429},
            Self::Tvfrr431RequestHeaderFieldsTooLarge => quote::quote! {431},
            Self::Tvfrr451UnavailableForLegalReasons => quote::quote! {451},
            Self::Tvfrr500InternalServerError => quote::quote! {500},
            Self::Tvfrr501NotImplemented => quote::quote! {501},
            Self::Tvfrr502BadGateway => quote::quote! {502},
            Self::Tvfrr503ServiceUnavailable => quote::quote! {503},
            Self::Tvfrr504GatewayTimeout => quote::quote! {504},
            Self::Tvfrr505HttpVersionNotSupported => quote::quote! {505},
            Self::Tvfrr506VariantAlsoNegotiates => quote::quote! {506},
            Self::Tvfrr507InsufficientStorage => quote::quote! {507},
            Self::Tvfrr508LoopDetected => quote::quote! {508},
            Self::Tvfrr510NotExtended => quote::quote! {510},
            Self::Tvfrr511NetworkAuthenticationRequired => quote::quote! {511},
        }
    }
    pub fn to_status_code_description_token_stream(&self) -> proc_macro2::TokenStream {
        match self {
            Self::Tvfrr100Continue => quote::quote! {"continue"},
            Self::Tvfrr101SwitchingProtocols => quote::quote! {"switching protocols"},
            Self::Tvfrr102Processing => quote::quote! {"processing"},
            Self::Tvfrr200Ok => quote::quote! {"ok"},
            Self::Tvfrr201Created => quote::quote! {"created"},
            Self::Tvfrr202Accepted => quote::quote! {"accepted"},
            Self::Tvfrr203NonAuthoritativeInformation => {
                quote::quote! {"non authoritative information"}
            }
            Self::Tvfrr204NoContent => quote::quote! {"no content"},
            Self::Tvfrr205ResetContent => quote::quote! {"reset content"},
            Self::Tvfrr206PartialContent => quote::quote! {"partial content"},
            Self::Tvfrr207MultiStatus => quote::quote! {"multi status"},
            Self::Tvfrr208AlreadyReported => quote::quote! {"already reported"},
            Self::Tvfrr226ImUsed => quote::quote! {"im used"},
            Self::Tvfrr300MultipleChoices => quote::quote! {"multiple choices"},
            Self::Tvfrr301MovedPermanently => quote::quote! {"moved permanently"},
            Self::Tvfrr302Found => quote::quote! {"found"},
            Self::Tvfrr303SeeOther => quote::quote! {"see other"},
            Self::Tvfrr304NotModified => quote::quote! {"not modified"},
            Self::Tvfrr305UseProxy => quote::quote! {"use proxy"},
            Self::Tvfrr307TemporaryRedirect => quote::quote! {"temporary redirect"},
            Self::Tvfrr308PermanentRedirect => quote::quote! {"permanent redirect"},
            Self::Tvfrr400BadRequest => quote::quote! {"bad request"},
            Self::Tvfrr401Unauthorized => quote::quote! {"unauthorized"},
            Self::Tvfrr402PaymentRequired => quote::quote! {"payment required"},
            Self::Tvfrr403Forbidden => quote::quote! {"forbidden"},
            Self::Tvfrr404NotFound => quote::quote! {"not found"},
            Self::Tvfrr405MethodNotAllowed => quote::quote! {"method not allowed"},
            Self::Tvfrr406NotAcceptable => quote::quote! {"not acceptable"},
            Self::Tvfrr407ProxyAuthenticationRequired => {
                quote::quote! {"proxy authentication required"}
            }
            Self::Tvfrr408RequestTimeout => quote::quote! {"request timeout"},
            Self::Tvfrr409Conflict => quote::quote! {"conflict"},
            Self::Tvfrr410Gone => quote::quote! {"gone"},
            Self::Tvfrr411LengthRequired => quote::quote! {"length required"},
            Self::Tvfrr412PreconditionFailed => quote::quote! {"precondition failed"},
            Self::Tvfrr413PayloadTooLarge => quote::quote! {"payload too large"},
            Self::Tvfrr414UriTooLong => quote::quote! {"uri too long"},
            Self::Tvfrr415UnsupportedMediaType => quote::quote! {"unsupported media type"},
            Self::Tvfrr416RangeNotSatisfiable => quote::quote! {"range not satisfiable"},
            Self::Tvfrr417ExpectationFailed => quote::quote! {"expectation failed"},
            Self::Tvfrr418ImATeapot => quote::quote! {"im a teapot"},
            Self::Tvfrr421MisdirectedRequest => quote::quote! {"misdirected request"},
            Self::Tvfrr422UnprocessableEntity => quote::quote! {"unprocessable entity"},
            Self::Tvfrr423Locked => quote::quote! {"locked"},
            Self::Tvfrr424FailedDependency => quote::quote! {"failed dependency"},
            Self::Tvfrr426UpgradeRequired => quote::quote! {"upgrade required"},
            Self::Tvfrr428PreconditionRequired => quote::quote! {"precondition required"},
            Self::Tvfrr429TooManyRequests => quote::quote! {"too many requests"},
            Self::Tvfrr431RequestHeaderFieldsTooLarge => {
                quote::quote! {"request header fields too large"}
            }
            Self::Tvfrr451UnavailableForLegalReasons => {
                quote::quote! {"unavailable for legal reasons"}
            }
            Self::Tvfrr500InternalServerError => quote::quote! {"internal server error"},
            Self::Tvfrr501NotImplemented => quote::quote! {"not implemented"},
            Self::Tvfrr502BadGateway => quote::quote! {"bad gateway"},
            Self::Tvfrr503ServiceUnavailable => quote::quote! {"service unavailable"},
            Self::Tvfrr504GatewayTimeout => quote::quote! {"gateway timeout"},
            Self::Tvfrr505HttpVersionNotSupported => {
                quote::quote! {"http version not supported"}
            }
            Self::Tvfrr506VariantAlsoNegotiates => quote::quote! {"variant also negotiates"},
            Self::Tvfrr507InsufficientStorage => quote::quote! {"insufficient storage"},
            Self::Tvfrr508LoopDetected => quote::quote! {"loop detected"},
            Self::Tvfrr510NotExtended => quote::quote! {"not extended"},
            Self::Tvfrr511NetworkAuthenticationRequired => {
                quote::quote! {"network authentication required"}
            }
        }
    }
    pub fn to_proc_macro_attribute_view_token_stream(&self) -> proc_macro2::TokenStream {
        let value = format!("#[{self}]");
        value.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{value} {}", proc_macro_common::constants::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    }
}

impl TryFrom<&syn::Variant> for StatusCode {
    type Error = std::string::String;
    fn try_from(value: &syn::Variant) -> Result<Self, Self::Error> {
        let mut option_self: Option<Self> = None;
        for element in &value.attrs {
            if element.path().segments.len() == 1 {
                match element.path().segments.first() {
                    Some(segment) => {
                        if let Ok(value) = Self::try_from(&segment.ident.to_string()) {
                            match option_self {
                                Some(value) => {
                                    return Err(format!("duplicated status_code attributes {value} are not supported"));
                                }
                                None => {
                                    option_self = Some(value);
                                }
                            }
                        }
                    }
                    None => {
                        return Err(std::string::String::from(
                            "element.path().segments.first() is None",
                        ));
                    }
                }
            }
        }
        option_self.map_or_else(|| Err(std::string::String::from("status_code attribute not found")), |value| Ok(value))
    }
}

impl TryFrom<&&syn::Variant> for StatusCode {
    type Error = std::string::String;
    fn try_from(value: &&syn::Variant) -> Result<Self, Self::Error> {
        let mut option_self: Option<Self> = None;
        for element in &value.attrs {
            if element.path().segments.len() == 1 {
                match element.path().segments.first() {
                    Some(segment) => {
                        if let Ok(value) = Self::try_from(&segment.ident.to_string()) {
                            match option_self {
                                Some(value) => {
                                    return Err(format!("duplicated status_code attributes {value} are not supported"));
                                }
                                None => {
                                    option_self = Some(value);
                                }
                            }
                        }
                    }
                    None => {
                        return Err(std::string::String::from(
                            "element.path().segments.first() is None",
                        ));
                    }
                }
            }
        }
        option_self.map_or_else(|| Err(std::string::String::from("status_code attribute not found")), |value| Ok(value))
    }
}

impl TryFrom<&std::string::String> for StatusCode {
    type Error = ();
    fn try_from(value: &std::string::String) -> Result<Self, Self::Error> {
        if value == "tvfrr_100_continue" {
            Ok(Self::Tvfrr100Continue)
        } else if value == "tvfrr_101_switching_protocols" {
            Ok(Self::Tvfrr101SwitchingProtocols)
        } else if value == "tvfrr_102_processing" {
            Ok(Self::Tvfrr102Processing)
        } else if value == "tvfrr_200_ok" {
            Ok(Self::Tvfrr200Ok)
        } else if value == "tvfrr_201_created" {
            Ok(Self::Tvfrr201Created)
        } else if value == "tvfrr_202_accepted" {
            Ok(Self::Tvfrr202Accepted)
        } else if value == "tvfrr_203_non_authoritative_information" {
            Ok(Self::Tvfrr203NonAuthoritativeInformation)
        } else if value == "tvfrr_204_no_content" {
            Ok(Self::Tvfrr204NoContent)
        } else if value == "tvfrr_205_reset_content" {
            Ok(Self::Tvfrr205ResetContent)
        } else if value == "tvfrr_206_partial_content" {
            Ok(Self::Tvfrr206PartialContent)
        } else if value == "tvfrr_207_multi_status" {
            Ok(Self::Tvfrr207MultiStatus)
        } else if value == "tvfrr_208_already_reported" {
            Ok(Self::Tvfrr208AlreadyReported)
        } else if value == "tvfrr_226_im_used" {
            Ok(Self::Tvfrr226ImUsed)
        } else if value == "tvfrr_300_multiple_choices" {
            Ok(Self::Tvfrr300MultipleChoices)
        } else if value == "tvfrr_301_moved_permanently" {
            Ok(Self::Tvfrr301MovedPermanently)
        } else if value == "tvfrr_302_found" {
            Ok(Self::Tvfrr302Found)
        } else if value == "tvfrr_303_see_other" {
            Ok(Self::Tvfrr303SeeOther)
        } else if value == "tvfrr_304_not_modified" {
            Ok(Self::Tvfrr304NotModified)
        } else if value == "tvfrr_305_use_proxy" {
            Ok(Self::Tvfrr305UseProxy)
        } else if value == "tvfrr_307_temporary_redirect" {
            Ok(Self::Tvfrr307TemporaryRedirect)
        } else if value == "tvfrr_308_permanent_redirect" {
            Ok(Self::Tvfrr308PermanentRedirect)
        } else if value == "tvfrr_400_bad_request" {
            Ok(Self::Tvfrr400BadRequest)
        } else if value == "tvfrr_401_unauthorized" {
            Ok(Self::Tvfrr401Unauthorized)
        } else if value == "tvfrr_402_payment_required" {
            Ok(Self::Tvfrr402PaymentRequired)
        } else if value == "tvfrr_403_forbidden" {
            Ok(Self::Tvfrr403Forbidden)
        } else if value == "tvfrr_404_not_found" {
            Ok(Self::Tvfrr404NotFound)
        } else if value == "tvfrr_405_method_not_allowed" {
            Ok(Self::Tvfrr405MethodNotAllowed)
        } else if value == "tvfrr_406_not_acceptable" {
            Ok(Self::Tvfrr406NotAcceptable)
        } else if value == "tvfrr_407_proxy_authentication_required" {
            Ok(Self::Tvfrr407ProxyAuthenticationRequired)
        } else if value == "tvfrr_408_request_timeout" {
            Ok(Self::Tvfrr408RequestTimeout)
        } else if value == "tvfrr_409_conflict" {
            Ok(Self::Tvfrr409Conflict)
        } else if value == "tvfrr_410_gone" {
            Ok(Self::Tvfrr410Gone)
        } else if value == "tvfrr_411_length_required" {
            Ok(Self::Tvfrr411LengthRequired)
        } else if value == "tvfrr_412_precondition_failed" {
            Ok(Self::Tvfrr412PreconditionFailed)
        } else if value == "tvfrr_413_payload_too_large" {
            Ok(Self::Tvfrr413PayloadTooLarge)
        } else if value == "tvfrr_414_uri_too_long" {
            Ok(Self::Tvfrr414UriTooLong)
        } else if value == "tvfrr_415_unsupported_media_type" {
            Ok(Self::Tvfrr415UnsupportedMediaType)
        } else if value == "tvfrr_416_range_not_satisfiable" {
            Ok(Self::Tvfrr416RangeNotSatisfiable)
        } else if value == "tvfrr_417_expectation_failed" {
            Ok(Self::Tvfrr417ExpectationFailed)
        } else if value == "tvfrr_418_im_a_teapot" {
            Ok(Self::Tvfrr418ImATeapot)
        } else if value == "tvfrr_421_misdirected_request" {
            Ok(Self::Tvfrr421MisdirectedRequest)
        } else if value == "tvfrr_422_unprocessable_entity" {
            Ok(Self::Tvfrr422UnprocessableEntity)
        } else if value == "tvfrr_423_locked" {
            Ok(Self::Tvfrr423Locked)
        } else if value == "tvfrr_424_failed_dependency" {
            Ok(Self::Tvfrr424FailedDependency)
        } else if value == "tvfrr_426_upgrade_required" {
            Ok(Self::Tvfrr426UpgradeRequired)
        } else if value == "tvfrr_428_precondition_required" {
            Ok(Self::Tvfrr428PreconditionRequired)
        } else if value == "tvfrr_429_too_many_requests" {
            Ok(Self::Tvfrr429TooManyRequests)
        } else if value == "tvfrr_431_request_header_fields_too_large" {
            Ok(Self::Tvfrr431RequestHeaderFieldsTooLarge)
        } else if value == "tvfrr_451_unavailable_for_legal_reasons" {
            Ok(Self::Tvfrr451UnavailableForLegalReasons)
        } else if value == "tvfrr_500_internal_server_error" {
            Ok(Self::Tvfrr500InternalServerError)
        } else if value == "tvfrr_501_not_implemented" {
            Ok(Self::Tvfrr501NotImplemented)
        } else if value == "tvfrr_502_bad_gateway" {
            Ok(Self::Tvfrr502BadGateway)
        } else if value == "tvfrr_503_service_unavailable" {
            Ok(Self::Tvfrr503ServiceUnavailable)
        } else if value == "tvfrr_504_gateway_timeout" {
            Ok(Self::Tvfrr504GatewayTimeout)
        } else if value == "tvfrr_505_http_version_not_supported" {
            Ok(Self::Tvfrr505HttpVersionNotSupported)
        } else if value == "tvfrr_506_variant_also_negotiates" {
            Ok(Self::Tvfrr506VariantAlsoNegotiates)
        } else if value == "tvfrr_507_insufficient_storage" {
            Ok(Self::Tvfrr507InsufficientStorage)
        } else if value == "tvfrr_508_loop_detected" {
            Ok(Self::Tvfrr508LoopDetected)
        } else if value == "tvfrr_510_not_extended" {
            Ok(Self::Tvfrr510NotExtended)
        } else if value == "tvfrr_511_network_authentication_required" {
            Ok(Self::Tvfrr511NetworkAuthenticationRequired)
        } else {
            Err(())
        }
    }
}

pub fn get_only_one(
    variant: &syn::Variant,
    proc_macro_name_ident_stringified: &std::string::String,
) -> StatusCode {
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
    option_self.map_or_else(|| {
        panic!("{proc_macro_name_ident_stringified} not supported status_code attribute");
    }, |attr| attr)
}
