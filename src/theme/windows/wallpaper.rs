use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, JsonSchema)]
#[serde(tag = "type")]
#[serde(rename_all = "lowercase")]
pub enum WindowsWallpaper {
    Single { url: String },
    Diaporama { urls: Vec<String> },
}
