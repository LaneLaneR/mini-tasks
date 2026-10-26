use clap::{Args, Subcommand, ValueEnum, Parser};
use chrono::NaiveDate;

mod error;
pub use self::error::CliError;

#[derive(Parser, Debug)]
#[command(name = "mini, task", version, about = "CLI менеджер задач")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    #[command(alias = "--add")]
    Add {
        #[arg(short, long)]
        title: String,
        #[arg(short, long)]
        description: String,
        #[arg(long)]
        day: NaiveDate,
    },
    #[command(alias = "--show")]
    Show(ShowArgs),
    #[command(alias = "--delete")]
    Delete {
        #[arg(short, long)]
        id: u32,
    },
    #[command(alias = "--execute")]
    Execute {
        #[arg(short, long)]
        id: u32,
    }
}

#[derive (Args, Debug)]
#[command(group(clap::ArgGroup::new("format").args(["short", "long"]).multiple(false)))]
pub struct ShowArgs {
    #[arg(short, long)]
    pub date: Option<NaiveDate>,
    #[arg(short, long)]
    pub short: bool,
    #[arg(short, long)]
    pub long: bool,
}