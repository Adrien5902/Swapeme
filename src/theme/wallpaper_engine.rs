use super::ThemeApp;
use crate::error::{Error, Result};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use wallpaper_engine_cli_wrapper::WallpaperEngine;

impl ThemeApp for ThemeWallpaperEngine {
    fn apply(&self) -> Result<()> {
        let Some(we) =
            Error::error_prone_step(&|| WallpaperEngine::new().map_err(|e| (e).into()), None)
        else {
            return Ok(());
        };

        if let Some(wallpapers) = &self.wallpapers {
            for wallpaper in wallpapers {
                Error::error_prone_step(
                    &|| {
                        we.set_wallpaper(&wallpaper.wallpaper, wallpaper.monitor)
                            .map_err(|e| e.into())
                    },
                    Some(&format!(
                        "Applied wallpaper {} from {} on monitor {}",
                        wallpaper.wallpaper.id, wallpaper.wallpaper.kind, wallpaper.monitor
                    )),
                );
            }
        }

        Ok(())
    }
}

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct ThemeAuthor {
    pub name: String,
    pub url: Option<String>,
}

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct ThemeWallpaperEngine {
    pub wallpapers: Option<Vec<ThemeWEWallpaper>>,
    pub playlist: Option<Vec<ThemeWEPlaylist>>,
}

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct ThemeWEPlaylist {
    pub monitor: u32,
}

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct ThemeWEWallpaper {
    #[serde(flatten)]
    pub wallpaper: wallpaper_engine_cli_wrapper::Wallpaper,
    pub monitor: u32,
}
