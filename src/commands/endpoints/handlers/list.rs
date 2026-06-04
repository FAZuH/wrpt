use simplelog::debug;

use crate::commands::consts;
use crate::commands::endpoints::args::list::EndpointListCommand;
use crate::commands::endpoints::models::list::EndpointList;
use crate::commands::error::CliError;
use crate::commands::helpers::CliContext;
use crate::commands::helpers::build_table;
use crate::commands::helpers::construct_url;
use crate::commands::helpers::parse_api_response;

pub(crate) fn handler(command: EndpointListCommand, ctx: &CliContext) -> Result<(), CliError> {
    debug!("command = {:?}", command);

    let endpoints = fetch_endpoints(ctx)?;

    build_table(&endpoints, None).printstd();

    Ok(())
}

pub(crate) fn fetch_endpoints(ctx: &CliContext) -> Result<Vec<EndpointList>, CliError> {
    let url = construct_url(&ctx.base_url, consts::ENDPOINT_ENDPOINTS)?;

    debug!("request = GET {:?}", url.as_str());

    let response = ctx
        .client
        .get(url)
        .query(&[("excludeSnapshots", "true")])
        .send()?;

    parse_api_response(response)
}
