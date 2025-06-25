use crate::cli::wait_for_user;
use crate::error::HandledError;
use crate::theme::wallpaper_engine::{Wallpaper, WallpaperEngine};
use std::fmt::Display;

pub struct WallpaperNotFoundError(pub Wallpaper, pub WallpaperEngine);

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

        wait_for_user("wallpaper installation");
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
        todo!();
    }
}
