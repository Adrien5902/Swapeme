pub mod spicetify;
pub mod wallpaper_engine;
pub mod windows;

use crate::{
    error::Result,
    theme::{
        spicetify::ThemeSpicetify,
        wallpaper_engine::{ThemeAuthor, ThemeWallpaperEngine},
        windows::ThemeWindows,
    },
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::{fs, path::Path};

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct Theme {
    pub version: Option<String>,
    pub author: Option<ThemeAuthor>,
    pub wallpaper_engine: Option<ThemeWallpaperEngine>,
    pub spicetify: Option<ThemeSpicetify>,
    pub windows: Option<ThemeWindows>,
}

pub trait ThemeApp {
    fn apply(&self) -> Result<()>;
}

impl Theme {
    pub fn apply(&self) -> Result<()> {
        self.wallpaper_engine
            .as_ref()
            .map(|w| w.apply())
            .transpose()?;

        self.spicetify.as_ref().map(|s| s.apply()).transpose()?;

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
