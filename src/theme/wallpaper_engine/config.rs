use crate::{error::Result, theme::wallpaper_engine::WallpaperEngine};
use core::fmt;
use serde::{
    Deserialize, Deserializer, Serialize, Serializer,
    de::{self, Visitor},
};
use serde_json::Value;
use std::path::PathBuf;
use std::{collections::HashMap, fs};

impl WallpaperEngine {
    pub fn read_config(&self) -> Result<WallpaperEngineConfig> {
        let content = fs::read_to_string(&self.path.join("config.json"))?;
        Ok(serde_json::from_str(&content)?)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WallpaperEngineConfig {
    #[serde(rename = "?installdirectory")]
    pub path: PathBuf,
    #[serde(flatten)]
    pub users: HashMap<String, WallpaperEngineUserConfig>,
}

impl WallpaperEngineConfig {
    pub fn get_current_user_config(&self) -> Option<&WallpaperEngineUserConfig> {
        self.users
            .get(dirs::desktop_dir()?.parent()?.file_name()?.to_str()?)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WallpaperEngineUserConfig {
    pub general: WallpaperEngineUserGeneralConfig,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WallpaperEngineUserGeneralConfig {
    #[serde(rename = "wallpaperconfig")]
    pub wallpaper_config: Option<UserWallpaperConfig>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserWallpaperConfig {
    pub layout: u32,
    #[serde(rename = "selectedwallpapers")]
    pub selected_wallpapers: HashMap<UserWallpaperConfigMonitorId, UserWallpaperConfigForMonitor>,
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct UserWallpaperConfigMonitorId(pub u32);

impl Serialize for UserWallpaperConfigMonitorId {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = format!("Monitor{}", self.0);
        serializer.serialize_str(&s)
    }
}

impl<'de> Deserialize<'de> for UserWallpaperConfigMonitorId {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct MonitorIdVisitor;

        impl<'de> Visitor<'de> for MonitorIdVisitor {
            type Value = UserWallpaperConfigMonitorId;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str(r#""Monitor<id>" string"#)
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: de::Error,
            {
                if let Some(suffix) = value.strip_prefix("Monitor") {
                    suffix
                        .parse::<u32>()
                        .map(UserWallpaperConfigMonitorId)
                        .map_err(|_| E::custom("Expected Monitor followed by a number"))
                } else {
                    Err(E::custom("Expected string to start with 'Monitor'"))
                }
            }
        }

        deserializer.deserialize_str(MonitorIdVisitor)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserWallpaperConfigForMonitor {
    pub file: PathBuf,
    pub playlist: Option<Value>,
}
