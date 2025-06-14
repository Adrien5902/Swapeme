use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::{fs, path::Path};
use wallpaper_engine_cli_wrapper::WallpaperEngine;

use crate::error::Result;

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct Theme {
    pub version: Option<String>,
    pub author: Option<ThemeAuthor>,
    pub wallpaper_engine: Option<ThemeWallpaperEngine>,
}

impl Theme {
    pub fn apply(&self) -> Result<()> {
        if let Some(wallpaper_engine_config) = &self.wallpaper_engine {
            let wallpaper_engine = WallpaperEngine::new()?;
            for wallpaper in wallpaper_engine_config
                .wallpapers
                .as_ref()
                .unwrap_or(&vec![])
            {
                if wallpaper.from == ThemeWEWallpaperSource::Workshop {
                    wallpaper_engine.set_workshop_wallpaper(&wallpaper.id, wallpaper.monitor_id);
                }
            }
        }

        Ok(())
    }

    pub fn read_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let content = fs::read_to_string(path)?;
        Self::parse_json(&content)
    }

    pub fn parse_json(content: &str) -> Result<Self> {
        Ok(serde_json::from_str(content)?)
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
    pub monitor_id: u32,
}

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct ThemeWEWallpaper {
    pub id: String,
    pub from: ThemeWEWallpaperSource,
    pub monitor_id: u32,
}

#[derive(Deserialize, Serialize, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ThemeWEWallpaperSource {
    MyProjects,
    Workshop,
}
