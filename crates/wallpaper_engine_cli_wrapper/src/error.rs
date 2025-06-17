use std::io;
use vdf_parser::error::VdfError;

use crate::{Wallpaper, WallpaperEngine};

#[derive(Debug)]
pub enum Error {
    WallpaperEngineNotInstalled,
    WallpaperNotFound(Wallpaper, WallpaperEngine),
    SteamNotInstalled,
    VdfError(VdfError),
    IOError(io::Error),
}

impl From<VdfError> for Error {
    fn from(value: VdfError) -> Self {
        Self::VdfError(value)
    }
}

impl From<io::Error> for Error {
    fn from(value: io::Error) -> Self {
        Self::IOError(value)
    }
}
