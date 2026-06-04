pub mod args;
pub mod handlers;
pub mod models;

use crate::commands::error::CliError;
use crate::commands::helpers::CliContext;
use crate::commands::stacks::args::StackCommand;
use crate::commands::stacks::args::StackSubCommand;

pub fn handler(stack: StackCommand, ctx: &CliContext) -> Result<(), CliError> {
    let command = stack.command;

    match command {
        StackSubCommand::List(command) => handlers::list::handler(command, ctx),
        StackSubCommand::Deploy(command) => handlers::deploy::handler(command, ctx),
        StackSubCommand::Remove(command) => handlers::remove::handler(command, ctx),
        StackSubCommand::ResourceControl(command) => {
            handlers::resource_control::handler(command, ctx)
        }
        StackSubCommand::Start(command) => handlers::start::handler(command, ctx),
        StackSubCommand::Stop(command) => handlers::stop::handler(command, ctx),
    }
}
