use crate::{
    error::Result,
    theme::{
        ThemeApp,
        windows::{color_scheme::WindowsColorScheme, wallpaper::WindowsWallpaper},
    },
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
    const NAME: &'static str = "Windows";
    type App = ();

    fn get_app() -> Option<Self::App> {
        Some(())
    }

    fn apply(&self, _app: Self::App) -> crate::error::Result<()> {
        todo!();
        Ok(())
    }

    fn get_current(_app: Self::App) -> Result<Self> {
        todo!();
        Ok(Self {
            wallpaper: None,
            color_scheme: None,
        })
    }
}
