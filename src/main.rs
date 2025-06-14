pub mod error;
#[cfg(test)]
mod test;
pub mod theme;

use crate::theme::Theme;
use clap::{Arg, Command};
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
                .arg(Arg::new("theme")),
        )
}

fn main() {
    let matches = cli().get_matches();

    match matches.subcommand() {
        Some(("gen_schema", _)) => {
            let schema = schemars::schema_for!(Theme);
            let contents = serde_json::to_string_pretty(&schema).unwrap();
            fs::write("resources/theme.swapeme.schema.json", &contents).unwrap();
        }
        Some(("apply", arg_matches)) => {
            let theme = arg_matches.get_one::<String>("theme").unwrap();
            Theme::read_file(Path::new("resources/test").join(format!("{}.swapeme.json", theme)))
                .unwrap()
                .apply()
                .unwrap();
        }
        _ => unreachable!(),
    }
}
