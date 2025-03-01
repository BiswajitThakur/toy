use std::fmt;

#[allow(unused)]
#[derive(Debug, Clone, Copy, Default)]
pub enum Status {
    Continue,
    SwitchingProtocols,
    Processing,
    EarlyHints,
    #[default]
    OK,
    Created,
    Accepted,
    NonAuthoritativeInformation,
    NoContent,
    ResetContent,
    PartialContent,
    MultiStatus,
    AlreadyReported,
    IMUsed,
    MultipleChoices,
    MovedPermanently,
    Found,
    SeeOther,
    NotModified,
    TemporaryRedirect,
    PermanentRedirect,
    BadRequest,
    Unauthorized,
    PaymentRequired,
    Forbidden,
    NotFound,
    MethodNotAllowed,
    NotAcceptable,
    ProxyAuthenticationRequired,
    RequestTimeout,
    Conflict,
    Gone,
    LengthRequired,
    PreconditionFailed,
    ContentTooLarge,
    URITooLong,
    UnsupportedMediaType,
    RangeNotSatisfiable,
    ExpectationFailed,
    ImATeapot,
    MisdirectedRequest,
    UnprocessableContent,
    Locked,
    FailedDependency,
    TooEarly,
    UpgradeRequired,
    PreconditionRequired,
    TooManyRequests,
    RequestHeaderFieldsTooLarge,
    UnavailableForLegalReasons,
    InternalServerError,
    NotImplemented,
    BadGateway,
    ServiceUnavailable,
    GatewayTimeout,
    HTTPVersionNotSupported,
    VariantAlsoNegotiates,
    InsufficientStorage,
    LoopDetected,
    NotExtended,
    NetworkAuthenticationRequired,
}

impl Into<u32> for Status {
    fn into(self) -> u32 {
        match self {
            Self::Continue => 100,
            Self::SwitchingProtocols => 101,
            Self::Processing => 102,
            Self::EarlyHints => 103,
            Self::OK => 200,
            Self::Created => 201,
            Self::Accepted => 202,
            Self::NonAuthoritativeInformation => 203,
            Self::NoContent => 204,
            Self::ResetContent => 205,
            Self::PartialContent => 206,
            Self::MultiStatus => 207,
            Self::AlreadyReported => 208,
            Self::IMUsed => 226,
            Self::MultipleChoices => 300,
            Self::MovedPermanently => 301,
            Self::Found => 302,
            Self::SeeOther => 303,
            Self::NotModified => 304,
            Self::TemporaryRedirect => 307,
            Self::PermanentRedirect => 308,
            Self::BadRequest => 400,
            Self::Unauthorized => 401,
            Self::PaymentRequired => 402,
            Self::Forbidden => 403,
            Self::NotFound => 404,
            Self::MethodNotAllowed => 405,
            Self::NotAcceptable => 406,
            Self::ProxyAuthenticationRequired => 407,
            Self::RequestTimeout => 408,
            Self::Conflict => 409,
            Self::Gone => 410,
            Self::LengthRequired => 411,
            Self::PreconditionFailed => 412,
            Self::ContentTooLarge => 413,
            Self::URITooLong => 414,
            Self::UnsupportedMediaType => 415,
            Self::RangeNotSatisfiable => 416,
            Self::ExpectationFailed => 417,
            Self::ImATeapot => 418,
            Self::MisdirectedRequest => 421,
            Self::UnprocessableContent => 422,
            Self::Locked => 423,
            Self::FailedDependency => 424,
            Self::TooEarly => 425,
            Self::UpgradeRequired => 426,
            Self::PreconditionRequired => 428,
            Self::TooManyRequests => 429,
            Self::RequestHeaderFieldsTooLarge => 431,
            Self::UnavailableForLegalReasons => 451,
            Self::InternalServerError => 500,
            Self::NotImplemented => 501,
            Self::BadGateway => 502,
            Self::ServiceUnavailable => 503,
            Self::GatewayTimeout => 504,
            Self::HTTPVersionNotSupported => 505,
            Self::VariantAlsoNegotiates => 506,
            Self::InsufficientStorage => 507,
            Self::LoopDetected => 508,
            Self::NotExtended => 510,
            Self::NetworkAuthenticationRequired => 511,
        }
    }
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Continue => write!(f, "100 Continue"),
            Self::SwitchingProtocols => write!(f, "101 Switching Protocols"),
            Self::Processing => write!(f, "102 Processing"),
            Self::EarlyHints => write!(f, "103 Early Hints"),
            Self::OK => write!(f, "200 OK"),
            Self::Created => write!(f, "201 Created"),
            Self::Accepted => write!(f, "202 Accepted"),
            Self::NonAuthoritativeInformation => write!(f, "203 Non-Authoritative Information"),
            Self::NoContent => write!(f, "204 No Content"),
            Self::ResetContent => write!(f, "205 Reset Content"),
            Self::PartialContent => write!(f, "206 Partial Content"),
            Self::MultiStatus => write!(f, "207 Multi-Status"),
            Self::AlreadyReported => write!(f, "208 Already Reported"),
            Self::IMUsed => write!(f, "226 IM Used"),
            Self::MultipleChoices => write!(f, "300 Multiple Choices"),
            Self::MovedPermanently => write!(f, "301 Moved Permanently"),
            Self::Found => write!(f, "302 Found"),
            Self::SeeOther => write!(f, "303 See Other"),
            Self::NotModified => write!(f, "304 Not Modified"),
            Self::TemporaryRedirect => write!(f, "307 Temporary Redirect"),
            Self::PermanentRedirect => write!(f, "308 Permanent Redirect"),
            Self::BadRequest => write!(f, "400 Bad Request"),
            Self::Unauthorized => write!(f, "401 Unauthorized"),
            Self::PaymentRequired => write!(f, "402 Payment Required"),
            Self::Forbidden => write!(f, "403 Forbidden"),
            Self::NotFound => write!(f, "404 Not Found"),
            Self::MethodNotAllowed => write!(f, "405 Method Not Allowed"),
            Self::NotAcceptable => write!(f, "406 Not Acceptable"),
            Self::ProxyAuthenticationRequired => write!(f, "407 Proxy Authentication Required"),
            Self::RequestTimeout => write!(f, "408 Request Timeout"),
            Self::Conflict => write!(f, "409 Conflict"),
            Self::Gone => write!(f, "410 Gone"),
            Self::LengthRequired => write!(f, "411 Length Required"),
            Self::PreconditionFailed => write!(f, "412 Precondition Failed"),
            Self::ContentTooLarge => write!(f, "413 Content Too Large"),
            Self::URITooLong => write!(f, "414 URI Too Long"),
            Self::UnsupportedMediaType => write!(f, "415 Unsupported Media Type"),
            Self::RangeNotSatisfiable => write!(f, "416 Range Not Satisfiable"),
            Self::ExpectationFailed => write!(f, "417 Expectation Failed"),
            Self::ImATeapot => write!(f, "418 I'm a teapot"),
            Self::MisdirectedRequest => write!(f, "421 Misdirected Request"),
            Self::UnprocessableContent => write!(f, "422 Unprocessable Content"),
            Self::Locked => write!(f, "423 Locked"),
            Self::FailedDependency => write!(f, "424 Failed Dependency"),
            Self::TooEarly => write!(f, "425 Too Early"),
            Self::UpgradeRequired => write!(f, "426 Upgrade Required"),
            Self::PreconditionRequired => write!(f, "428 Precondition Required"),
            Self::TooManyRequests => write!(f, "429 Too Many Requests"),
            Self::RequestHeaderFieldsTooLarge => write!(f, "431 Request Header Fields Too Large"),
            Self::UnavailableForLegalReasons => write!(f, "451 Unavailable For Legal Reasons"),
            Self::InternalServerError => write!(f, "500 Internal Server Error"),
            Self::NotImplemented => write!(f, "501 Not Implemented"),
            Self::BadGateway => write!(f, "502 Bad Gateway"),
            Self::ServiceUnavailable => write!(f, "503 Service Unavailable"),
            Self::GatewayTimeout => write!(f, "504 Gateway Timeout"),
            Self::HTTPVersionNotSupported => write!(f, "505 HTTP Version Not Supported"),
            Self::VariantAlsoNegotiates => write!(f, "506 Variant Also Negotiates"),
            Self::InsufficientStorage => write!(f, "507 Insufficient Storage"),
            Self::LoopDetected => write!(f, "508 Loop Detected"),
            Self::NotExtended => write!(f, "510 Not Extended"),
            Self::NetworkAuthenticationRequired => write!(f, "511 Network Authentication Required"),
        }
    }
}
