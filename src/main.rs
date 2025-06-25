pub mod error;
#[cfg(test)]
mod test;

pub mod cli;
pub mod color;
pub mod theme;

use crate::{
    cli::error,
    theme::{Theme, ThemeApp, spicetify::ThemeSpicetify, wallpaper_engine::ThemeWallpaperEngine},
};
use clap::{Arg, Command};
use colored::Colorize;
use std::{fs, path::Path};

fn cli() -> Command {
    Command::new("swapeme")
        .about("Swapeme a windows theme swapper")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .allow_external_subcommands(false)
        .subcommand(Command::new("gen_schema").about("Generates the theme json schema"))
        .subcommand(
            Command::new("apply")
                .about("Applies a theme")
                .arg(Arg::new("theme"))
                .arg_required_else_help(true),
        )
        .subcommand(Command::new("create").about("Create a new theme based on your current config"))
}

fn main() {
    let matches = cli().get_matches();

    match matches.subcommand() {
        Some(("gen_schema", _)) => {
            let schema = schemars::schema_for!(Theme);
            let contents = serde_json::to_string_pretty(&schema).unwrap();
            fs::write("resources/theme.swapeme.schema.json", &contents).unwrap();
        }
        Some(("create", _)) => {
            let theme = Theme {
                author: None,
                version: None,
                spicetify: ThemeSpicetify::ask_to_get_current(),
                windows: None,
                wallpaper_engine: ThemeWallpaperEngine::ask_to_get_current(),
            };
            theme.write_json("test.swapeme.json").unwrap()
        }
        Some(("apply", arg_matches)) => {
            let theme = arg_matches.get_one::<String>("theme").unwrap();
            Theme::read_file(Path::new("resources/test").join(format!("{}.swapeme.json", theme)))
                .expect(&error(format!(
                    "Failed to read theme, make sure it's installed and valid, run {} to know where to place an installed theme", "swapeme path".bold(),
                )
                ))
                .apply()
                .unwrap();
        }
        _ => unreachable!(),
    }
}
