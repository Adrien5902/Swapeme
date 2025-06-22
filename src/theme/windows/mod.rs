use crate::theme::{
    ThemeApp,
    windows::{color_scheme::WindowsColorScheme, wallpaper::WindowsWallpaper},
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

pub mod color_scheme;
pub mod wallpaper;

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct ThemeWindows {
    wallpaper: Option<WindowsWallpaper>,
    color_scheme: Option<WindowsColorScheme>,
}

impl ThemeApp for ThemeWindows {
    fn apply(&self) -> crate::error::Result<()> {
        Ok(())
    }
}
