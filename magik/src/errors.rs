use std::borrow::Cow;
use std::{error, fmt};

#[derive(Debug)]
pub enum Error {
    ParseError(Cow<'static, str>),
    RenderError(Cow<'static, str>),
    TemplateNotFound(Cow<'static, str>),
    TemplateReadError(Cow<'static, str>),
    InvalidSyntax(Cow<'static, str>),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::ParseError(msg) => write!(f, "Parse error: {}", msg),
            Error::RenderError(msg) => write!(f, "Render error: {}", msg),
            Error::TemplateNotFound(msg) => write!(f, "Template not found: {}", msg),
            Error::TemplateReadError(msg) => write!(f, "Template read error: {}", msg),
            Error::InvalidSyntax(msg) => write!(f, "Invalid syntax: {}", msg),
        }
    }
}

impl error::Error for Error {}
