use std::fmt::Display;

use colored::Colorize;
use dialoguer::Select;

pub fn skip_dialog<F, R>(prompt: &str, item: &str, action: &str, on_action: F) -> Option<R>
where
    F: Fn() -> R,
{
    if let Some(picked) = Select::new()
        .default(0)
        .with_prompt(error(format!("{}\nWhat would you like to do", prompt)))
        .item(format!("Skip {}", item))
        .item(action)
        .interact_opt()
        .unwrap()
    {
        if picked == 1 {
            return Some(on_action());
        }
    }
    None
}

pub fn error(err: impl Display) -> String {
    format!("{} {}", " ERR ".white().on_red().bold(), err)
}

pub fn success(msg: impl Display) -> String {
    format!("{} {}", " SUCCESS ".white().on_green().bold(), msg)
}

pub fn display_success(msg: impl Display) {
    println!("{}", success(msg))
}
