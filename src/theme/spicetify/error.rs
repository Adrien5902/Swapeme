use std::fmt::Display;

use crate::{
    cli::{start_cmd, wait_for_user},
    error::HandledError,
};

pub struct SpicetifyNotInstalledError {}

impl Display for SpicetifyNotInstalledError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Spicetify installation not found")
    }
}

impl HandledError for SpicetifyNotInstalledError {
    fn action(&self) -> &'static str {
        "Install spicetify for spotify theme"
    }
    fn handle(&self) {
        start_cmd("https://spicetify.app/").unwrap();
        wait_for_user("spicetify installation");
    }
    fn item(&self) -> &'static str {
        "spotify theme"
    }
}
