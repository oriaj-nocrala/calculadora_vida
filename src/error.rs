use std::{io::ErrorKind, fmt::{Debug, Display, Formatter}};

use chrono::{ParseError, FixedOffset};

pub struct Error {
    pub kind: ErrorKind,
    pub message: String,
}

impl Error {
    pub fn new(kind: ErrorKind, message: String) -> Error {
        Error {
            kind,
            message,
        }
    }
}

impl From<chrono::format::ParseError> for Error {
    fn from(error: ParseError) -> Self {
        Error::new(ErrorKind::InvalidData, error.to_string())
    }
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Error::new(error.kind(), error.to_string())
    }
}

impl From<chrono::DateTime<FixedOffset>> for Error {
    fn from(error: chrono::DateTime<FixedOffset>) -> Self {
        Error::new(ErrorKind::InvalidData, error.to_string())
    }
}

impl Debug for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let kind = kind_to_string(self.kind);
        write!(f, "Kind: {}\nError: {}", kind, self.message)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let kind = kind_to_string(self.kind);
        write!(f, "Kind: {}\nError: {}", kind, self.message)
    }
}

fn kind_to_string(kind:ErrorKind) -> String {
    match kind{
        ErrorKind::NotFound => "NotFound".to_string(),
        ErrorKind::PermissionDenied => "PermissionDenied".to_string(),
        ErrorKind::ConnectionRefused => "ConnectionRefused".to_string(),
        ErrorKind::ConnectionReset => "ConnectionReset".to_string(),
        ErrorKind::ConnectionAborted => "ConnectionAborted".to_string(),
        ErrorKind::NotConnected => "NotConnected".to_string(),
        ErrorKind::AddrInUse => "AddrInUse".to_string(),
        ErrorKind::AddrNotAvailable => "AddrNotAvailable".to_string(),
        ErrorKind::BrokenPipe => "BrokenPipe".to_string(),
        ErrorKind::AlreadyExists => "AlreadyExists".to_string(),
        ErrorKind::WouldBlock => "WouldBlock".to_string(),
        ErrorKind::InvalidInput => "InvalidInput".to_string(),
        ErrorKind::InvalidData => "InvalidData".to_string(),
        ErrorKind::TimedOut => "TimedOut".to_string(),
        ErrorKind::WriteZero => "WriteZero".to_string(),
        ErrorKind::Interrupted => "Interrupted".to_string(),
        ErrorKind::Other => "Other".to_string(),
        ErrorKind::UnexpectedEof => "UnexpectedEof".to_string(),
        ErrorKind::Unsupported => "Unsupported".to_string(),
        ErrorKind::OutOfMemory => "OutOfMemory".to_string(),
        _ => "Unknown".to_string(),
    }
}