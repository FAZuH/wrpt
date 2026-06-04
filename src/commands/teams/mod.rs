pub mod args;
pub mod handlers;
pub mod models;

use crate::commands::error::CliError;
use crate::commands::helpers::CliContext;
use crate::commands::teams::args::TeamCommand;
use crate::commands::teams::args::TeamSubCommand;

pub fn handler(endpoint: TeamCommand, ctx: &CliContext) -> Result<(), CliError> {
    let command = endpoint.command;

    match command {
        TeamSubCommand::List(command) => handlers::list::handler(command, ctx),
    }
}
