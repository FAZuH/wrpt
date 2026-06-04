mod autoport;
mod consts;
mod endpoints;
pub(crate) mod error;
mod helpers;
mod stacks;
mod teams;
mod users;

use clap::Parser;
use clap::Subcommand;
use simplelog::error;

use crate::commands::Command::Endpoint;
use crate::commands::Command::Stack;
use crate::commands::Command::Team;
use crate::commands::Command::User;
use crate::commands::autoport::AutoportArgs;
use crate::commands::autoport::init_logger;
use crate::commands::helpers::CliContext;

#[derive(Debug, Subcommand)]
pub(crate) enum Command {
    /// Endpoint subcommands (list, ...)
    Endpoint(endpoints::args::EndpointCommand),

    /// Stacks subcommands (list, deploy, inspect, ...)
    Stack(stacks::args::StackCommand),

    /// Teams subcommands (list, ...)
    Team(teams::args::TeamCommand),

    /// Users subcommands (list, ...)
    User(users::args::UserCommand),
}

pub fn init() -> Result<(), ()> {
    let args: AutoportArgs = AutoportArgs::parse();

    init_logger(&args);

    let ctx = CliContext::from_global_args(&args.global_args).map_err(|e| {
        error!("{}", e);
    })?;

    let result = match args.command {
        Endpoint(command) => endpoints::handler(command, &ctx),
        Stack(command) => stacks::handler(command, &ctx),
        Team(command) => teams::handler(command, &ctx),
        User(command) => users::handler(command, &ctx),
    };

    result.map_err(|e| {
        error!("{}", e);
    })
}
