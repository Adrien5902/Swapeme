pub mod error;
#[cfg(test)]
mod test;

use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;
use serde_variant::to_variant_name;
use std::fmt::Display;
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use vdf_parser::VdfValue;
use vdf_parser::error::VdfError;
use vdf_parser::parse_vdf_text;
use winreg::RegKey;
use winreg::enums::*;

use crate::error::Error;

#[derive(Debug, Clone)]
pub struct WallpaperEngine {
    pub path: PathBuf,
}

impl WallpaperEngine {
    pub const STEAM_GAME_ID: &str = "431960";

    fn get_steam_path() -> Result<String, Error> {
        let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
        let steam_key = hklm
            .open_subkey("SOFTWARE\\WOW6432Node\\Valve\\Steam")
            .map_err(|_| Error::WallpaperEngineNotInstalled)?;
        let path: String = steam_key.get_value("InstallPath")?;
        Ok(path)
    }

    fn get_wallpaper_engine_path(steam_path: &str) -> Result<String, Error> {
        let vdf_path = format!("{}/steamapps/libraryfolders.vdf", steam_path);
        let content = fs::read_to_string(&vdf_path)?;
        let data = parse_vdf_text(&content)?;

        let VdfValue::Block(libraryfolders) = &data.value else {
            return Err(
                VdfError::ValueNotFound("libraryfolders".to_string(), "".to_string()).into(),
            );
        };

        for (_, lib) in libraryfolders {
            let VdfValue::String(path) = &lib.get_string_value("path")?.value else {
                continue;
            };
            let VdfValue::Block(app_list) = &lib.get_string_value("apps")?.value else {
                continue;
            };
            if app_list.contains_key(Self::STEAM_GAME_ID) {
                return Ok(format!("{}/steamapps/common/wallpaper_engine", path));
            }
        }
        Err(Error::WallpaperEngineNotInstalled)
    }

    pub fn new_with_path(path: PathBuf) -> Self {
        Self { path }
    }

    pub fn new() -> Result<Self, Error> {
        Ok(Self::new_with_path(PathBuf::from(
            Self::get_wallpaper_engine_path(&Self::get_steam_path()?)?,
        )))
    }

    pub fn get_app32_path(&self) -> PathBuf {
        self.path.join("wallpaper32.exe")
    }

    pub fn get_workshop_path(&self) -> PathBuf {
        self.path
            .parent()
            .unwrap()
            .parent()
            .unwrap()
            .join(format!("workshop/content/{}", Self::STEAM_GAME_ID))
    }

    pub fn get_workshop_wallpaper_path(&self, wallpaper_id: &str) -> PathBuf {
        self.get_workshop_path()
            .join(wallpaper_id)
            .join("project.json")
    }

    pub fn get_local_wallpaper_path(&self, wallpaper_id: &str, dir: &str) -> PathBuf {
        self.path
            .join("projects")
            .join(dir)
            .join(wallpaper_id)
            .join("project.json")
    }

    pub fn invoke_command(&self) -> Command {
        let app_path = self.get_app32_path();
        let mut command = Command::new(app_path);
        command.arg("-control");
        command
    }

    pub fn set_wallpaper(&self, wallpaper: &Wallpaper, monitor: u32) -> Result<(), Error> {
        let wallpaper_path = match &wallpaper.kind {
            WallpaperKind::Workshop => self.get_workshop_wallpaper_path(&wallpaper.id),
            other => self.get_local_wallpaper_path(&wallpaper.id, &other.to_string()),
        };

        if !wallpaper_path.exists() {
            return Err(Error::WallpaperNotFound(wallpaper.clone(), self.clone()));
        }

        let process = self
            .invoke_command()
            .args([
                "openWallpaper",
                "-file",
                wallpaper_path.to_str().unwrap(),
                "-monitor",
                &monitor.to_string(),
            ])
            .spawn()?;

        let output = process.wait_with_output()?;
        output
            .status
            .success()
            .then_some(())
            .ok_or(Error::WallpaperEngineNotInstalled)
    }

    pub fn open_workshop_page_for_wallpaper(
        &self,
        wallpaper_id: &str,
    ) -> Result<(), std::io::Error> {
        Command::new("cmd")
            .args([
                "/C",
                "start",
                &format!("steam://url/CommunityFilePage/{}", wallpaper_id),
            ])
            .spawn()?;
        Ok(())
    }

    // pub fn get_current_wallpaper(&self, monitor: u32) -> Result<String, Error> {
    //     Ok(String::from_utf8(
    //         self.invoke_command()
    //             .args(["getWallpaper", "-monitor", &monitor.to_string()])
    //             .output()?
    //             .stdout,
    //     )
    //     .unwrap())
    // }
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema, Clone)]
pub struct Wallpaper {
    pub id: String,
    pub kind: WallpaperKind,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum WallpaperKind {
    Workshop,
    MyProjects,
    DefaultProjects,
}

impl Display for WallpaperKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(to_variant_name(self).unwrap())
    }
}
