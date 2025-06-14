use std::io;

#[derive(Debug)]
pub enum Error {
    IOError(io::Error),
    SerdeJSON(serde_json::Error),
    WallpaperEngine(wallpaper_engine_cli_wrapper::error::Error),
}

impl From<io::Error> for Error {
    fn from(value: io::Error) -> Self {
        Self::IOError(value)
    }
}

impl From<serde_json::Error> for Error {
    fn from(value: serde_json::Error) -> Self {
        Self::SerdeJSON(value)
    }
}

impl From<wallpaper_engine_cli_wrapper::error::Error> for Error {
    fn from(value: wallpaper_engine_cli_wrapper::error::Error) -> Self {
        Self::WallpaperEngine(value)
    }
}

pub type Result<T> = std::result::Result<T, Error>;
