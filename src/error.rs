use std::{
    fmt::{Debug, Display},
    io,
};

use vdf_parser::error::VdfError;
use zip::result::ZipError;

use crate::cli::{display_success, error, skip_dialog};

#[derive(Debug)]
pub enum Error {
    HandledError(Box<dyn HandledError>),
    UnhandledError(UnhandledError),
}

impl Error {
    pub fn error_prone_step<R, F: Fn() -> Result<R>>(
        step: &F,
        success_msg: Option<&str>,
    ) -> Option<R> {
        match step() {
            Ok(res) => {
                if let Some(success_message) = success_msg {
                    display_success(success_message);
                }
                Some(res)
            }
            Err(err) => match err {
                Error::HandledError(error) => {
                    skip_dialog(&error.to_string(), error.item(), error.action(), || {
                        error.handle();
                        Self::error_prone_step(step, success_msg)
                    })
                    .flatten()
                }
                Error::UnhandledError(e) => panic!("{:?}", e),
            },
        }
    }
}

pub trait HandledError: Display {
    fn item(&self) -> &'static str;
    fn action(&self) -> &'static str;
    fn handle(&self);
}

impl<T: HandledError + 'static> From<T> for Error {
    fn from(value: T) -> Self {
        Self::HandledError(Box::new(value))
    }
}

impl Debug for dyn HandledError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&error(self.to_string()))
    }
}

impl From<UnhandledError> for Error {
    fn from(value: UnhandledError) -> Self {
        Error::UnhandledError(value)
    }
}

#[derive(Debug)]
pub enum UnhandledError {
    SerdeJSON(serde_json::Error),
    VdfError(VdfError),
    IOError(io::Error),
    Reqwest(reqwest::Error),
    Zip(ZipError),
}

impl From<io::Error> for Error {
    fn from(value: io::Error) -> Self {
        Self::UnhandledError(UnhandledError::IOError(value))
    }
}

impl From<serde_json::Error> for Error {
    fn from(value: serde_json::Error) -> Self {
        Self::UnhandledError(UnhandledError::SerdeJSON(value))
    }
}

impl From<VdfError> for Error {
    fn from(value: VdfError) -> Self {
        Self::UnhandledError(crate::error::UnhandledError::VdfError(value))
    }
}

impl From<reqwest::Error> for Error {
    fn from(value: reqwest::Error) -> Self {
        Self::UnhandledError(crate::error::UnhandledError::Reqwest(value))
    }
}

impl From<ZipError> for Error {
    fn from(value: ZipError) -> Self {
        Self::UnhandledError(crate::error::UnhandledError::Zip(value))
    }
}

pub type Result<T> = std::result::Result<T, Error>;
