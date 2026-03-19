use crate::commands::consts;
use crate::commands::error::CliError;
use crate::commands::helpers::{build_table, construct_url, parse_api_response, CliContext};
use crate::commands::users::args::list::UserListCommand;
use crate::commands::users::models::list::User;
use simplelog::debug;

pub(crate) fn handler(command: UserListCommand, ctx: &CliContext) -> Result<(), CliError> {
    debug!("command = {:?}", command);

    let users = fetch_users(ctx)?;

    build_table(&users, Some(&["Id", "Username", "Role"])).printstd();

    Ok(())
}

pub(crate) fn fetch_users(ctx: &CliContext) -> Result<Vec<User>, CliError> {
    let url = construct_url(&ctx.base_url, consts::ENDPOINT_USERS)?;

    debug!("request = GET {:?}", url.as_str());

    let response = ctx.client.get(url).send()?;

    parse_api_response(response)
}
