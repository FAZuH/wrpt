mod consts;
mod endpoints;
pub(crate) mod error;
mod helpers;
mod stacks;
mod teams;
mod users;
mod wrpt;

use crate::commands::helpers::CliContext;
use crate::commands::wrpt::{init_logger, WrptArgs};
use crate::commands::Command::{Endpoint, Stack, Team, User};
use clap::{Parser, Subcommand};
use simplelog::error;

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
    let args: WrptArgs = WrptArgs::parse();

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
