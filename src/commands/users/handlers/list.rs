use simplelog::debug;

use crate::commands::consts;
use crate::commands::error::CliError;
use crate::commands::helpers::build_table;
use crate::commands::helpers::construct_url;
use crate::commands::helpers::parse_api_response;
use crate::commands::helpers::CliContext;
use crate::commands::users::args::list::UserListCommand;
use crate::commands::users::models::list::User;

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
