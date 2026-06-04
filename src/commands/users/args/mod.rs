pub(crate) mod list;

use clap::Args;
use clap::Subcommand;

use crate::commands::users::args::list::UserListCommand;

#[derive(Debug, Args)]
pub struct UserCommand {
    #[command(subcommand)]
    pub command: UserSubCommand,
}

#[derive(Debug, Subcommand)]
pub enum UserSubCommand {
    /// List users
    List(UserListCommand),
}
