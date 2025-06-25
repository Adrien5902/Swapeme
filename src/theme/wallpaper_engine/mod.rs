use super::ThemeApp;
use crate::cli::start_cmd;
use crate::error::{Error, Result};
use crate::theme::wallpaper_engine::config::WallpaperEngineUserConfig;
use crate::theme::wallpaper_engine::error::{InstallationNotFoundError, WallpaperNotFoundError};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_variant::to_variant_name;
use std::fmt::Display;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use vdf_parser::VdfValue;
use vdf_parser::error::VdfError;
use vdf_parser::parse_vdf_text;
use winreg::RegKey;
use winreg::enums::HKEY_LOCAL_MACHINE;

pub mod config;
pub mod error;

impl ThemeApp for ThemeWallpaperEngine {
    const NAME: &'static str = "Wallpaper engine";
    type App = WallpaperEngine;

    fn apply(&self, app: WallpaperEngine) -> Result<()> {
        if let Some(wallpapers) = &self.wallpapers {
            for wallpaper in wallpapers {
                Error::error_prone_step(
                    &|| {
                        app.set_wallpaper(&wallpaper.wallpaper, wallpaper.monitor)
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

    fn get_current(app: WallpaperEngine) -> Result<Self> {
        Ok((app.read_config()?.get_current_user_config().unwrap()).into())
    }
    fn get_app() -> Option<Self::App> {
        Error::error_prone_step(&|| WallpaperEngine::new().map_err(|e| (e).into()), None)
    }
}

#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct ThemeAuthor {
    pub name: String,
    pub url: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct ThemeWallpaperEngine {
    pub wallpapers: Option<Vec<ThemeWEWallpaper>>,
    pub playlist: Option<Vec<ThemeWEPlaylist>>,
}

#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct ThemeWEPlaylist {
    //TODO
    pub monitor: u32,
}

#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct ThemeWEWallpaper {
    #[serde(flatten)]
    pub wallpaper: Wallpaper,
    pub monitor: u32,
}

#[derive(Debug, Clone)]
pub struct WallpaperEngine {
    pub path: PathBuf,
}

impl WallpaperEngine {
    pub const STEAM_GAME_ID: &str = "431960";

    fn get_steam_path() -> Result<String> {
        let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
        let steam_key = hklm
            .open_subkey("SOFTWARE\\WOW6432Node\\Valve\\Steam")
            .map_err(|_| InstallationNotFoundError {})?;
        let path: String = steam_key.get_value("InstallPath")?;
        Ok(path)
    }

    fn get_wallpaper_engine_path(steam_path: &str) -> Result<String> {
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
        Err(InstallationNotFoundError {}.into())
    }

    pub fn new_with_path(path: PathBuf) -> Self {
        Self { path }
    }

    pub fn new() -> Result<Self> {
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

    pub fn set_wallpaper(&self, wallpaper: &Wallpaper, monitor: u32) -> Result<()> {
        let wallpaper_path = match &wallpaper.kind {
            WallpaperKind::Workshop => self.get_workshop_wallpaper_path(&wallpaper.id),
            other => self.get_local_wallpaper_path(&wallpaper.id, &other.to_string()),
        };

        if !wallpaper_path.exists() {
            return Err(WallpaperNotFoundError(wallpaper.clone(), self.clone()).into());
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
            .ok_or(InstallationNotFoundError {}.into())
    }

    pub fn open_workshop_page_for_wallpaper(&self, wallpaper_id: &str) -> Result<()> {
        start_cmd(&format!("steam://url/CommunityFilePage/{}", wallpaper_id))?;
        Ok(())
    }
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema, Clone)]
pub struct Wallpaper {
    pub id: String,
    pub kind: WallpaperKind,
    // pub config: TODO
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

impl From<&WallpaperEngineUserConfig> for ThemeWallpaperEngine {
    fn from(value: &WallpaperEngineUserConfig) -> Self {
        let mut wallpapers = vec![];
        let mut playlists = vec![];

        for (key, value) in value
            .general
            .wallpaper_config
            .as_ref()
            .expect("Couldn't find wallpaper config")
            .selected_wallpapers
            .iter()
        {
            if let Some(playlist) = &value.playlist {
                playlists.push(ThemeWEPlaylist { monitor: key.0 });
                todo!();
            } else {
                let wallpaper = ThemeWEWallpaper {
                    wallpaper: value.file.as_path().into(),
                    monitor: key.0,
                };
                wallpapers.push(wallpaper);
            }
        }

        ThemeWallpaperEngine {
            wallpapers: (wallpapers.len() != 0).then_some(wallpapers),
            playlist: (playlists.len() != 0).then_some(playlists),
        }
    }
}

impl From<&Path> for Wallpaper {
    fn from(value: &Path) -> Self {
        let wallpaper_dir = value.parent().unwrap();
        Wallpaper {
            id: wallpaper_dir
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .to_string(),
            kind: wallpaper_dir.parent().unwrap().into(),
        }
    }
}

impl From<&Path> for WallpaperKind {
    fn from(value: &Path) -> Self {
        let path_str = value.to_str().unwrap().to_string();
        if path_str.contains("workshop") {
            WallpaperKind::Workshop
        } else {
            serde_json::from_str(&format!(
                "\"{}\"",
                value
                    .parent()
                    .unwrap()
                    .file_name()
                    .unwrap()
                    .to_str()
                    .unwrap(),
            ))
            .unwrap()
        }
    }
}
