pub(crate) mod list;

use clap::Args;
use clap::Subcommand;

use crate::commands::teams::args::list::TeamListCommand;

#[derive(Debug, Args)]
pub struct TeamCommand {
    #[command(subcommand)]
    pub command: TeamSubCommand,
}

#[derive(Debug, Subcommand)]
pub enum TeamSubCommand {
    /// List teams
    List(TeamListCommand),
}
