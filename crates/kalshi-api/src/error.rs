use std::fmt;

use reqwest::StatusCode;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TransportKind {
    Connect,
    Timeout,
    Other,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BodyErrorKind {
    Reset,
    Timeout,
    Other,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VenueError {
    SchemaDrift {
        operation: String,
        status: u16,
        prefix: Vec<u8>,
        truncated: bool,
        total: u64,
    },
    NonJson {
        status: u16,
        body_prefix: Vec<u8>,
        truncated: bool,
        total: u64,
    },
    Http {
        status: u16,
        code: Option<String>,
        message: Option<String>,
        raw: Vec<u8>,
        truncated: bool,
        total: u64,
    },
    Transport {
        kind: TransportKind,
    },
    HttpBodyFailed {
        status: u16,
        body_error: BodyErrorKind,
        prefix: Vec<u8>,
    },
    BodyTooLarge {
        status: u16,
        total_seen: u64,
        prefix: Vec<u8>,
    },
    /// The retained prefix is not a complete JSON value. The call is not repeated.
    TruncatedRaw {
        status: u16,
        prefix: Vec<u8>,
        total: u64,
    },
    IndexOutOfRange {
        value: i64,
    },
    RequestBuild {
        detail: String,
    },
}

impl VenueError {
    pub fn is_rate_limited(&self) -> bool {
        match self {
            Self::Http { status, .. }
            | Self::HttpBodyFailed { status, .. }
            | Self::BodyTooLarge { status, .. }
            | Self::SchemaDrift { status, .. }
            | Self::NonJson { status, .. }
            | Self::TruncatedRaw { status, .. } => {
                *status == StatusCode::TOO_MANY_REQUESTS.as_u16()
            }
            Self::Transport { .. } | Self::IndexOutOfRange { .. } | Self::RequestBuild { .. } => {
                false
            }
        }
    }

    pub fn status(&self) -> Option<u16> {
        match self {
            Self::Transport { .. } | Self::IndexOutOfRange { .. } | Self::RequestBuild { .. } => {
                None
            }
            Self::Http { status, .. }
            | Self::HttpBodyFailed { status, .. }
            | Self::BodyTooLarge { status, .. }
            | Self::SchemaDrift { status, .. }
            | Self::NonJson { status, .. }
            | Self::TruncatedRaw { status, .. } => Some(*status),
        }
    }
}

impl fmt::Display for VenueError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SchemaDrift {
                operation, status, ..
            } => {
                write!(f, "schema drift on {operation} (HTTP {status})")
            }
            Self::NonJson { status, .. } => write!(f, "non-JSON body (HTTP {status})"),
            Self::Http { status, .. } => write!(f, "HTTP {status}"),
            Self::Transport { kind } => write!(f, "transport error ({kind:?})"),
            Self::HttpBodyFailed {
                status, body_error, ..
            } => {
                write!(f, "body read failed after HTTP {status} ({body_error:?})")
            }
            Self::BodyTooLarge {
                status, total_seen, ..
            } => {
                write!(
                    f,
                    "body larger than the limit after HTTP {status} ({total_seen} bytes seen)"
                )
            }
            Self::TruncatedRaw { status, total, .. } => {
                write!(f, "truncated raw body (HTTP {status}, {total} bytes)")
            }
            Self::IndexOutOfRange { value } => {
                write!(
                    f,
                    "exchange index {value} does not fit in the query parameter"
                )
            }
            Self::RequestBuild { detail } => write!(f, "request build failed: {detail}"),
        }
    }
}

impl std::error::Error for VenueError {}

#[derive(Debug)]
pub struct MiddlewareFault(pub String);

impl fmt::Display for MiddlewareFault {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl std::error::Error for MiddlewareFault {}
