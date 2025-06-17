use std::fmt::Display;

use dialoguer::Input;
use wallpaper_engine_cli_wrapper::{Wallpaper, WallpaperEngine};

use crate::error::{Error, HandledError, UnhandledError};

impl From<wallpaper_engine_cli_wrapper::error::Error> for Error {
    fn from(value: wallpaper_engine_cli_wrapper::error::Error) -> Self {
        match value {
            wallpaper_engine_cli_wrapper::error::Error::SteamNotInstalled
            | wallpaper_engine_cli_wrapper::error::Error::WallpaperEngineNotInstalled => {
                Error::HandledError(Box::new(InstallationNotFoundError()))
            }

            wallpaper_engine_cli_wrapper::error::Error::WallpaperNotFound(w, we) => {
                Error::HandledError(Box::new(WallpaperNotFoundError(w, we)))
            }
            wallpaper_engine_cli_wrapper::error::Error::IOError(e) => {
                Error::UnhandledError(UnhandledError::IOError(e))
            }
            wallpaper_engine_cli_wrapper::error::Error::VdfError(e) => {
                Error::UnhandledError(UnhandledError::VdfError(e))
            }
        }
    }
}

pub struct WallpaperNotFoundError(Wallpaper, WallpaperEngine);

impl Display for WallpaperNotFoundError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!(
            "Wallpaper {} from {} not found",
            self.0.id,
            self.0.kind.to_string(),
        ))
    }
}
impl HandledError for WallpaperNotFoundError {
    fn action(&self) -> &'static str {
        "Install it from workshop"
    }

    fn item(&self) -> &'static str {
        "this wallpaper"
    }

    fn handle(&self) {
        self.1.open_workshop_page_for_wallpaper(&self.0.id).unwrap();

        Input::new()
            .default(true)
            .with_prompt("Waiting for you to install the wallpaper press enter when done")
            .interact()
            .unwrap();
    }
}

pub struct InstallationNotFoundError();

impl Display for InstallationNotFoundError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Couldn't find steam or wallpaper engine installation on your system")
    }
}

impl HandledError for InstallationNotFoundError {
    fn item(&self) -> &'static str {
        "wallpaper engine step"
    }

    fn action(&self) -> &'static str {
        "Set wallpaper engine's path"
    }

    fn handle(&self) {
        //TODO
    }
}
