pub mod error;

use crate::{
    cli::display_success,
    error::{Error, HandledError, Result},
    theme::{ThemeApp, spicetify::error::SpicetifyNotInstalledError},
};
use reqwest::blocking::get;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::{
    fmt::Display,
    fs::{self, File},
    io::{self, Cursor, Read},
    path::PathBuf,
    process::{Command, Stdio},
};
use zip::ZipArchive;

#[derive(Clone)]
struct Spicetify {
    is_global: bool,
    path: PathBuf,
}

#[derive(Deserialize, Serialize, JsonSchema, Clone)]
pub struct ThemeSpicetify {
    name: String,
    color_scheme: Option<String>,
    url: String,
}

impl ThemeApp for ThemeSpicetify {
    fn apply(&self) -> Result<()> {
        if let Some(spicetify) = Error::error_prone_step(&|| Spicetify::new(), None) {
            Error::error_prone_step(
                &|| {
                    spicetify.set_theme(&self)?;
                    Ok(())
                },
                Some(&format!("Applied theme {} to spotify", self.name)),
            );
        }
        Ok(())
    }
}

impl Spicetify {
    const APP_NAME: &str = "spicetify";
    const EXE_NAME: &str = "spicetify.exe";

    pub fn new() -> Result<Self> {
        let (path, is_global) = {
            match Command::new(Self::APP_NAME).arg("path").output() {
                Ok(output) => {
                    let output_str = String::from_utf8(output.stdout).unwrap();
                    let path = PathBuf::from(output_str.lines().next().unwrap());
                    (path, true)
                }
                Err(_err) => {
                    let path = dirs::cache_dir().unwrap().join(Self::APP_NAME);
                    Command::new(path.join(Self::EXE_NAME))
                        .spawn()
                        .map_err(|_| SpicetifyNotInstalledError {})?;
                    (path, false)
                }
            }
        };

        Ok(Spicetify { is_global, path })
    }

    pub fn invoke_command(&self) -> Command {
        let mut cmd = if self.is_global {
            Command::new(Self::APP_NAME)
        } else {
            let exe_path = self.path.join(Self::EXE_NAME);
            Command::new(exe_path)
        };
        cmd.stdout(Stdio::null());
        cmd.stderr(Stdio::null());
        cmd
    }

    pub fn config(&self) -> Command {
        let mut cmd = self.invoke_command();
        cmd.arg("config");
        cmd
    }

    pub fn set_color_scheme(&self, color_scheme: &str) -> Result<()> {
        self.config()
            .arg("color_scheme")
            .arg(color_scheme)
            .output()?;
        Ok(())
    }

    pub fn apply(&self) -> Result<()> {
        self.invoke_command().arg("apply").output()?;
        Ok(())
    }

    pub fn get_theme_path(&self, name: &str) -> PathBuf {
        self.path.join("Themes").join(name)
    }

    pub fn set_theme(&self, theme: &ThemeSpicetify) -> Result<()> {
        if !self.get_theme_path(&theme.name).join("color.ini").exists() {
            return Err(SpicetifyThemeNotFoundError(self.clone(), theme.clone()).into());
        }

        self.config()
            .arg("current_theme")
            .arg(&theme.name)
            .output()?;

        self.set_color_scheme(
            &theme
                .color_scheme
                .as_ref()
                .unwrap_or(&"Spotify".to_string()),
        )?;

        self.apply()?;
        Ok(())
    }

    pub fn download_theme(&self, theme: &ThemeSpicetify) -> Result<()> {
        let mut response = get(&theme.url)?;

        if !response.status().is_success() {
            panic!("" /* TODO */)
        }

        let path = self.get_theme_path(&theme.name);
        fs::create_dir_all(&path)?;

        let mut content = Vec::new();
        response.read_to_end(&mut content)?;
        let reader = Cursor::new(content);
        let mut zip = ZipArchive::new(reader)?;
        for i in 0..zip.len() {
            let mut file = zip.by_index(i)?;
            let file_name = file.name().to_owned();
            if file_name.contains(&theme.name)
                && ["user.css", "color.ini", "theme.js"]
                    .iter()
                    .find(|name| file_name.contains(*name))
                    .is_some()
            {
                let out =
                    &mut File::create(path.join(PathBuf::from(file_name).file_name().unwrap()))?;
                io::copy(&mut file, out)?;
            }
        }

        Ok(())
    }
}

pub struct SpicetifyThemeNotFoundError(Spicetify, ThemeSpicetify);

impl Display for SpicetifyThemeNotFoundError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("{} spicetify theme not installed", self.1.name))
    }
}

impl HandledError for SpicetifyThemeNotFoundError {
    fn action(&self) -> &'static str {
        "Install theme"
    }
    fn item(&self) -> &'static str {
        "spotify theme"
    }
    fn handle(&self) {
        println!("Downloading theme...");
        self.0.download_theme(&self.1).unwrap();
        display_success(format!("Downloaded theme {}", self.1.name));
    }
}
