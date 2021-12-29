use std::fmt;

#[derive(Debug)]
pub enum Error {
    PathContainNonUnicode,
    InvalidPathComponent,
    RunCommandFailed,
    Io(std::io::Error),
    Json(serde_json::Error),
    StripPrefix(std::path::StripPrefixError),
    WalkDir(walkdir::Error),
    ReservedSiteName,

    TokioJoin(tokio::task::JoinError),
    SiteNotExist,
    DocumentNotFound,
    HyperHttp(hyper::http::Error),
    HandlebarsTemplate(handlebars::TemplateError),
    HandlebarsRender(handlebars::RenderError),

    Poisoned,
    Notify(notify::Error),
    Regex(regex::Error),
    UnprocessedRegexMatch,
    UnsupportedVariable,
    UnresolvedXreflink,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for Error {}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Error {
        Error::Io(err)
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Error {
        Error::Json(err)
    }
}

impl From<std::path::StripPrefixError> for Error {
    fn from(err: std::path::StripPrefixError) -> Error {
        Error::StripPrefix(err)
    }
}

impl From<walkdir::Error> for Error {
    fn from(err: walkdir::Error) -> Error {
        Error::WalkDir(err)
    }
}

impl From<tokio::task::JoinError> for Error {
    fn from(err: tokio::task::JoinError) -> Error {
        Error::TokioJoin(err)
    }
}

impl From<hyper::http::Error> for Error {
    fn from(err: hyper::http::Error) -> Error {
        Error::HyperHttp(err)
    }
}

impl From<handlebars::TemplateError> for Error {
    fn from(err: handlebars::TemplateError) -> Error {
        Error::HandlebarsTemplate(err)
    }
}

impl From<handlebars::RenderError> for Error {
    fn from(err: handlebars::RenderError) -> Error {
        Error::HandlebarsRender(err)
    }
}

impl<T> From<std::sync::PoisonError<T>> for Error {
    fn from(_: std::sync::PoisonError<T>) -> Error {
        Error::Poisoned
    }
}

impl From<notify::Error> for Error {
    fn from(err: notify::Error) -> Error {
        Error::Notify(err)
    }
}

impl From<regex::Error> for Error {
    fn from(err: regex::Error) -> Error {
        Error::Regex(err)
    }
}
