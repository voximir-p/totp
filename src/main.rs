mod account;
mod cli;

use crate::account::AccountArray;
use crate::cli::{Cli, make_styles};
use clap::{CommandFactory, FromArgMatches};
use std::fs;

fn main() {
    let data_dir = std::env::home_dir().unwrap();
    let totp_dir = data_dir.join(".totp");
    fs::create_dir_all(&totp_dir).unwrap();

    let secret_path = totp_dir.join("secrets.json");
    if !secret_path.exists() {
        fs::write(&secret_path, "[]").unwrap();
    }

    let mut acc_array = AccountArray::new(&secret_path);
    acc_array.load();

    let cmd = Cli::command().styles(make_styles());

    let parser = cmd.get_matches();
    let parser = Cli::from_arg_matches(&parser).unwrap();

    acc_array.match_cmd(parser.command);
}
