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
use dialoguer::Confirm;
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
    const NAME: &'static str;
    type App;

    fn get_app() -> Option<Self::App>;
    fn apply(&self, app: Self::App) -> Result<()>;
    fn get_current(app: Self::App) -> Result<Self>
    where
        Self: Sized;

    fn ask_to_get_current() -> Option<Self>
    where
        Self: Sized,
    {
        if let Some(app) = Self::get_app() {
            Confirm::new()
                .with_prompt(format!("Include {} theme ?", Self::NAME))
                .interact()
                .unwrap()
                .then_some(Self::get_current(app).unwrap())
        } else {
            None
        }
    }

    fn get_apply(&self) -> Result<()> {
        if let Some(app) = Self::get_app() {
            self.apply(app)
        } else {
            Ok(())
        }
    }
}

impl Theme {
    pub fn apply(&self) -> Result<()> {
        self.wallpaper_engine
            .as_ref()
            .map(|w| w.get_apply())
            .transpose()?;

        self.spicetify.as_ref().map(|s| s.get_apply()).transpose()?;

        Ok(())
    }

    pub fn read_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let content = fs::read_to_string(path)?;
        Self::parse_json(&content)
    }

    pub fn parse_json(content: &str) -> Result<Self> {
        Ok(serde_json::from_str(content)?)
    }

    pub fn write_json<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        Ok(fs::write(path, serde_json::to_string_pretty(&self)?)?)
    }
}
