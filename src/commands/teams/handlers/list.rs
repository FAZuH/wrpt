use simplelog::debug;

use crate::commands::consts;
use crate::commands::error::CliError;
use crate::commands::helpers::build_table;
use crate::commands::helpers::construct_url;
use crate::commands::helpers::parse_api_response;
use crate::commands::helpers::CliContext;
use crate::commands::teams::args::list::TeamListCommand;
use crate::commands::teams::models::list::TeamList;

pub(crate) fn handler(command: TeamListCommand, ctx: &CliContext) -> Result<(), CliError> {
    debug!("command = {:?}", command);

    let teams = fetch_teams(ctx)?;

    build_table(&teams, None).printstd();

    Ok(())
}

pub(crate) fn fetch_teams(ctx: &CliContext) -> Result<Vec<TeamList>, CliError> {
    let url = construct_url(&ctx.base_url, consts::ENDPOINT_TEAMS)?;

    debug!("request = GET {:?}", url.as_str());

    let response = ctx.client.get(url).send()?;

    parse_api_response(response)
}
