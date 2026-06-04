pub mod args;
pub mod handlers;
pub mod models;

use crate::commands::endpoints::args::EndpointCommand;
use crate::commands::endpoints::args::EndpointSubCommand;
use crate::commands::error::CliError;
use crate::commands::helpers::CliContext;

pub fn handler(endpoint: EndpointCommand, ctx: &CliContext) -> Result<(), CliError> {
    let command = endpoint.command;

    match command {
        EndpointSubCommand::List(command) => handlers::list::handler(command, ctx),
    }
}
