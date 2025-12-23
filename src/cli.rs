use anstyle::{AnsiColor, Color, Style};
use clap::builder::Styles;
use clap::{Args, Parser, Subcommand};

pub fn make_styles() -> Styles {
    Styles::styled()
        .header(
            Style::new()
                .bold()
                .fg_color(Some(Color::Ansi(AnsiColor::Green))),
        )
        .usage(
            Style::new()
                .bold()
                .fg_color(Some(Color::Ansi(AnsiColor::Green))),
        )
        .literal(
            Style::new()
                .bold()
                .fg_color(Some(Color::Ansi(AnsiColor::Cyan))),
        )
        .placeholder(Style::new().fg_color(Some(Color::Ansi(AnsiColor::Cyan))))
        .error(
            Style::new()
                .bold()
                .fg_color(Some(Color::Ansi(AnsiColor::Red))),
        )
}

#[derive(Parser)]
#[command(
    about = "An extremely fast TOTP account manager.",
    version = "1.0.0 (2025-12-23)"
)]
pub(crate) struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub(crate) enum Command {
    List(ListArgs),
    Path(PathArgs),
    Add(AddArgs),
    Remove(RemoveArgs),
    Clean(CleanArgs),
    Get(GetArgs),
}

#[derive(Args)]
#[command(about = "List all saved accounts (use --secret to include secrets)")]
pub(crate) struct ListArgs {
    #[arg(long)]
    #[arg(help = "Include secrets")]
    pub secret: bool,
}

#[derive(Args)]
#[command(about = "Print the JSON file path")]
pub(crate) struct PathArgs;

#[derive(Args)]
#[command(about = "Add a new account (ignored if it already exists)")]
pub(crate) struct AddArgs {
    #[arg(help = "The account's name")]
    pub name: String,

    #[arg(help = "The account's secret")]
    pub secret: String,
}

#[derive(Args)]
#[command(about = "Remove an account (requires confirmation)")]
pub(crate) struct RemoveArgs {
    #[arg(help = "The account's name")]
    pub name: String,
}

#[derive(Args)]
#[command(about = "Remove all accounts (requires confirmation)")]
pub(crate) struct CleanArgs;

#[derive(Args)]
#[command(about = "Show the current code and copy to clipboard (skip copy with -n | --no-copy)")]
pub(crate) struct GetArgs {
    #[arg(help = "The account's name", required = false)]
    pub name: Option<String>,

    #[arg(short = 'n', long = "no-copy")]
    #[arg(help = "Disable copying to clipboard")]
    pub no_copy: bool,
}
