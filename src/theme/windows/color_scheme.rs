use crate::{
    color::{ColorTheme, RgbaHexColor},
    error::Result,
    theme::ThemeApp,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use windows::Win32::{
    Foundation::{LPARAM, WPARAM},
    UI::WindowsAndMessaging::{
        HWND_BROADCAST, SMTO_ABORTIFHUNG, SendMessageTimeoutW, WM_SETTINGCHANGE,
    },
};
use winreg::{RegKey, enums::HKEY_CURRENT_USER};

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct WindowsColorScheme {
    system_color_theme: Option<ColorTheme>,
    app_theme: Option<ColorTheme>,
    accent_color: Option<RgbaHexColor>,
}

impl ThemeApp for WindowsColorScheme {
    const NAME: &'static str = "Windows color scheme";
    type App = ();

    fn apply(&self, app: Self::App) -> Result<()> {
        let hkcu = RegKey::predef(HKEY_CURRENT_USER);

        let (key, _) = hkcu
            .create_subkey("Software\\Microsoft\\Windows\\CurrentVersion\\Themes\\Personalize")?;

        for (reg_key, option_theme) in [
            ("AppsUseLightTheme", &self.app_theme),
            ("SystemUsesLightTheme", &self.system_color_theme),
        ] {
            if let Some(theme) = option_theme {
                match theme {
                    ColorTheme::Light => 1u32,
                    ColorTheme::Dark => 0u32,
                };
                key.set_value(
                    reg_key,
                    &match theme {
                        ColorTheme::Light => 1u32,
                        ColorTheme::Dark => 0u32,
                    },
                )?;
            }
        }

        // let (dwm, _) = hkcu.create_subkey("Software\\Microsoft\\Windows\\DWM")?;

        unsafe {
            SendMessageTimeoutW(
                HWND_BROADCAST,
                WM_SETTINGCHANGE,
                WPARAM(0),
                LPARAM(
                    "ImmersiveColorSet"
                        .encode_utf16()
                        .chain(Some(0))
                        .collect::<Vec<u16>>()
                        .as_ptr()
                        .addr()
                        .try_into()
                        .unwrap(),
                ),
                SMTO_ABORTIFHUNG,
                5000,
                Some(std::ptr::null_mut()),
            );
        }
        Ok(())
    }

    fn get_current(app: Self::App) -> Result<Self> {
        Ok(Self {
            system_color_theme: None,
            app_theme: None,
            accent_color: None,
        })
    }

    fn get_app() -> Option<Self::App> {
        Some(())
    }
}
