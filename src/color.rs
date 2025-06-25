use schemars::{JsonSchema, json_schema};

use hex_color::HexColor;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct RgbaHexColor(HexColor);

impl JsonSchema for RgbaHexColor {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "color".into()
    }
    fn json_schema(_generator: &mut schemars::SchemaGenerator) -> schemars::Schema {
        json_schema!({
            "type": "string",
            "pattern": "^#([A-Fa-f0-9]{8})$",
        })
    }
}

#[derive(Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "lowercase")]
pub enum ColorTheme {
    Light,
    Dark,
}
