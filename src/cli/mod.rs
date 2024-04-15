use clap::{Parser, Subcommand};

use self::{
    chpwd::Chpwd,
    clear::Clear,
    get::Get,
    list::List,
    precmd::Precmd,
    set::Set,
    show::Show,
};

mod chpwd;
mod clear;
mod get;
mod list;
mod precmd;
mod set;
mod show;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

impl Cli {
    pub fn execute(&self) {
        match &self.command {
            Commands::Set(inner) => inner.set(),
            Commands::Chpwd(inner) => inner.chpwd(),
            Commands::Precmd(inner) => inner.precmd(),
            Commands::Show(inner) => inner.show(),
            Commands::List(inner) => inner.list(),
            Commands::Get(inner) => inner.get(),
            Commands::Clear(inner) => inner.clear(),
        }
    }
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    #[clap(hide = true)]
    Chpwd(Chpwd),
    #[clap(hide = true)]
    Precmd(Precmd),
    /// Activates a profile
    Set(Set),
    /// Shows the variables of the current profile
    Show(Show),
    /// Outputs the variables and values of the current profile (default format is JSON)
    List(List),
    /// Outputs the value of a variable in the current profile
    Get(Get),
    /// Deactivates the current profile
    Clear(Clear),
}
