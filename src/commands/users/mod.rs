pub mod args;
pub mod handlers;
pub mod models;

use crate::commands::error::CliError;
use crate::commands::helpers::CliContext;
use crate::commands::users::args::{UserCommand, UserSubCommand};

pub fn handler(endpoint: UserCommand, ctx: &CliContext) -> Result<(), CliError> {
    let command = endpoint.command;

    match command {
        UserSubCommand::List(command) => handlers::list::handler(command, ctx),
    }
}
