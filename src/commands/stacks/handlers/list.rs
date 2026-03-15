use crate::commands::consts;
use crate::commands::error::CliError;
use crate::commands::helpers::{build_table, construct_url, parse_api_response, CliContext};
use crate::commands::stacks::args::list::StackListCommand;
use crate::commands::stacks::models::list::StackList;
use simplelog::debug;

pub(crate) fn handler(command: StackListCommand, ctx: &CliContext) -> Result<(), CliError> {
    debug!("command = {:?}", command);

    let stacks = fetch_stacks(ctx)?;

    build_table(
        &stacks,
        Some(&[
            "Id",
            "Name",
            "Type",
            "Status",
            "SwarmId",
            "EndpointId",
            "ResourceControl",
            "CreationDate",
            "CreatedBy",
            "UpdateDate",
            "UpdatedBy",
        ]),
    )
    .printstd();

    Ok(())
}

pub(crate) fn fetch_stacks(ctx: &CliContext) -> Result<Vec<StackList>, CliError> {
    let url = construct_url(&ctx.base_url, consts::ENDPOINT_STACKS)?;

    debug!("request = GET {:?}", url.as_str());

    let response = ctx.client.get(url).send()?;

    parse_api_response(response)
}
